use alloc::vec::Vec;
use core::cell::{Cell, UnsafeCell};
use core::mem::size_of;
use core::ops::Range;
use core::ptr::null_mut;

use crate::util::virtual_memory::{
    self, alloc, alloc_dual_mapping, flush_instruction_cache, protect_jit_memory, release,
    release_dual_mapping, DualMapping, MemoryFlags, ProtectJitAccess,
};
use crate::util::{
    align_down, align_up, bit_vector_clear, bit_vector_fill, bit_vector_get_bit,
    bit_vector_index_of, bit_vector_set_bit,
};
use crate::AsmError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
/// A policy that can be used with `reset()` functions.
pub enum ResetPolicy {
    /// Soft reset, does not deeallocate memory (default).
    Soft = 0,

    /// Hard reset, releases all memory used, if any.
    Hard = 1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JitAllocatorOptions {
    /// Enables the use of an anonymous memory-mapped memory that is mapped into two buffers having a different pointer.
    /// The first buffer has read and execute permissions and the second buffer has read+write permissions.
    ///
    /// See [alloc_dual_mapping](crate::virtual_memory::alloc_dual_mapping) for more details about this feature.
    ///
    /// ## Remarks
    ///
    /// Dual mapping would be automatically turned on by [JitAllocator] in case of hardened runtime that
    /// enforces `W^X` policy, so specifying this flag is essentually forcing to use dual mapped pages even when RWX
    /// pages can be allocated and dual mapping is not necessary.
    pub use_dual_mapping: bool,
    /// Enables the use of multiple pools with increasing granularity instead of a single pool. This flag would enable
    /// 3 internal pools in total having 64, 128, and 256 bytes granularity.
    ///
    /// This feature is only recommended for users that generate a lot of code and would like to minimize the overhead
    /// of `JitAllocator` itself by having blocks of different allocation granularities. Using this feature only for
    /// few allocations won't pay off as the allocator may need to create more blocks initially before it can take the
    /// advantage of variable block granularity.
    pub use_multiple_pools: bool,
    /// Always fill reserved memory by a fill-pattern.
    ///
    /// Causes a new block to be cleared by the fill pattern and freshly released memory to be cleared before making
    /// it ready for another use.
    pub fill_unused_memory: bool,
    /// When this flag is set the allocator would immediately release unused blocks during `release()` or `reset()`.
    /// When this flag is not set the allocator would keep one empty block in each pool to prevent excessive virtual
    /// memory allocations and deallocations in border cases, which involve constantly allocating and deallocating a
    /// single block caused by repetitive calling `alloc()` and `release()` when the allocator has either no blocks
    /// or have all blocks fully occupied.
    pub immediate_release: bool,
    pub custom_fill_pattern: Option<u32>,

    pub block_size: u32,
    pub granularity: u32,
}

impl Default for JitAllocatorOptions {
    fn default() -> Self {
        Self {
            use_dual_mapping: true,
            use_multiple_pools: true,
            fill_unused_memory: true,
            immediate_release: false,
            custom_fill_pattern: None,
            block_size: 0,
            granularity: 0,
        }
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const DEFAULT_FILL_PATTERN: u32 = 0xCCCCCCCC; // int3
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
const DEFAULT_FILL_PATTERN: u32 = 0x0; // int3

/// Number of pools to use when `JitAllocatorOptions::use_multiple_pools` is set.
///
/// Each pool increases granularity twice to make memory management more
/// efficient. Ideal number of pools appears to be 3 to 4 as it distributes
/// small and large functions properly.
const MULTI_POOL_COUNT: usize = 3;

/// Minimum granularity (and the default granularity for pool #0).
const MIN_GRANULARITY: usize = 64;

/// Maximum block size (32MB).
const MAX_BLOCK_SIZE: usize = 32 * 1024 * 1024;

struct BitVectorRangeIterator<'a, const B: u32> {
    slice: &'a [u32],
    idx: usize,
    end: usize,
    bit_word: u32,
}

const BIT_WORD_SIZE: usize = core::mem::size_of::<u32>() * 8;

impl<'a, const B: u32> BitVectorRangeIterator<'a, B> {
    const XOR_MASK: u32 = if B == 0 { u32::MAX } else { 0 };

    fn from_slice_and_nbitwords(data: &'a [u32], num_bit_words: usize) -> Self {
        Self::new(data, num_bit_words, 0, num_bit_words * BIT_WORD_SIZE)
    }

    fn new(data: &'a [u32], _num_bit_words: usize, start: usize, end: usize) -> Self {
        let idx = align_down(start, BIT_WORD_SIZE);
        let slice = &data[idx / BIT_WORD_SIZE..];

        let mut bit_word = 0;

        if idx < end {
            bit_word =
                (slice[0] ^ Self::XOR_MASK) & (u32::MAX << (start as u32 % BIT_WORD_SIZE as u32));
        }

        Self {
            slice,
            idx,
            end,
            bit_word,
        }
    }

    fn next_range(&mut self, range_hint: u32) -> Option<Range<u32>> {
        while self.bit_word == 0 {
            self.idx += BIT_WORD_SIZE;

            if self.idx >= self.end {
                return None;
            }

            self.slice = &self.slice[1..];
            self.bit_word = self.slice[0] ^ Self::XOR_MASK;
        }

        let i = self.bit_word.trailing_zeros();
        let start = self.idx as u32 + i;
        self.bit_word = !(self.bit_word ^ !(u32::MAX << i));
        let mut end;
        if self.bit_word == 0 {
            end = (self.idx as u32 + BIT_WORD_SIZE as u32).min(self.end as _);

            while end.wrapping_sub(start) < range_hint {
                self.idx += BIT_WORD_SIZE;

                if self.idx >= self.end {
                    break;
                }

                self.slice = &self.slice[1..];
                self.bit_word = self.slice[0] ^ Self::XOR_MASK;

                if self.bit_word != u32::MAX {
                    let j = self.bit_word.trailing_zeros();
                    end = (self.idx as u32 + j).min(self.end as _);
                    self.bit_word = !(self.bit_word ^ !(u32::MAX << j));
                    break;
                }

                end = (self.idx as u32 + BIT_WORD_SIZE as u32).min(self.end as _);
                self.bit_word = 0;
                continue;
            }

            Some(start..end)
        } else {
            let j = self.bit_word.trailing_zeros();
            end = (self.idx as u32 + j).min(self.end as _);

            self.bit_word = !(self.bit_word ^ !(u32::MAX << j));

            Some(start..end)
        }
    }
}

impl<'a> Iterator for BitVectorRangeIterator<'a, 0> {
    type Item = Range<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_range(u32::MAX)
    }
}

use intrusive_collections::{intrusive_adapter, rbtree::*};
use intrusive_collections::{KeyAdapter, UnsafeRef};

struct JitAllocatorBlock {
    node: Link,
    list_node: intrusive_collections::LinkedListLink,

    /// Link to the pool that owns this block.
    pool: *mut JitAllocatorPool,
    /// Virtual memory mapping - either single mapping (both pointers equal) or
    /// dual mapping, where one pointer is Read+Execute and the second Read+Write.
    mapping: DualMapping,
    /// Virtual memory size (block size) [bytes].
    block_size: usize,

    flags: Cell<u32>,
    area_size: Cell<u32>,
    area_used: Cell<u32>,
    largest_unused_area: Cell<u32>,
    search_start: Cell<u32>,
    search_end: Cell<u32>,

    used_bitvector: UnsafeCell<alloc::vec::Vec<u32>>,
    stop_bitvector: UnsafeCell<alloc::vec::Vec<u32>>,
}

impl JitAllocatorBlock {
    const FLAG_EMPTY: u32 = 0x00000001;
    const FLAG_DIRTY: u32 = 0x00000002;
    const FLAG_DUAL_MAPPED: u32 = 0x00000004;

    fn pool(&self) -> *mut JitAllocatorPool {
        self.pool
    }

    fn rx_ptr(&self) -> *const u8 {
        self.mapping.rx
    }

    fn rw_ptr(&self) -> *mut u8 {
        self.mapping.rw
    }

    fn flags(&self) -> u32 {
        self.flags.get()
    }

    fn add_flags(&self, flags: u32) {
        self.flags.set(self.flags() | flags);
    }

    fn clear_flags(&self, flags: u32) {
        self.flags.set(self.flags() & !flags);
    }

    fn is_dirty(&self) -> bool {
        (self.flags() & Self::FLAG_DIRTY) != 0
    }

    fn block_size(&self) -> usize {
        self.block_size
    }

    fn area_used(&self) -> u32 {
        self.area_used.get()
    }

    fn area_size(&self) -> u32 {
        self.area_size.get()
    }

    fn largest_unused_area(&self) -> u32 {
        self.largest_unused_area.get()
    }

    fn search_start(&self) -> u32 {
        self.search_start.get()
    }

    fn search_end(&self) -> u32 {
        self.search_end.get()
    }

    fn used_bitvector(&self) -> &alloc::vec::Vec<u32> {
        unsafe { &*self.used_bitvector.get() }
    }

    fn stop_bitvector(&self) -> &alloc::vec::Vec<u32> {
        unsafe { &*self.stop_bitvector.get() }
    }

    fn used_bitvector_mut(&self) -> &mut alloc::vec::Vec<u32> {
        unsafe { &mut *self.used_bitvector.get() }
    }

    fn stop_bitvector_mut(&self) -> &mut alloc::vec::Vec<u32> {
        unsafe { &mut *self.stop_bitvector.get() }
    }

    fn area_available(&self) -> u32 {
        self.area_size() - self.area_used()
    }

    fn mark_allocated_area(&self, allocated_area_start: u32, allocated_area_end: u32) {
        let allocated_area_size = allocated_area_end - allocated_area_start;

        bit_vector_fill(
            self.used_bitvector_mut(),
            allocated_area_start as _,
            allocated_area_size as _,
        );
        bit_vector_set_bit(
            self.stop_bitvector_mut(),
            allocated_area_end as usize - 1,
            true,
        );

        // SAFETY: Done inside JitAllocator behind mutex and pool is valid.
        unsafe {
            (*self.pool).total_area_used += allocated_area_size as usize;
        }

        self.area_used
            .set(self.area_used() + allocated_area_size as u32);

        if self.area_available() == 0 {
            self.search_start.set(self.area_size());
            self.search_end.set(0);
            self.largest_unused_area.set(0);
            self.clear_flags(Self::FLAG_DIRTY);
        } else {
            if self.search_start.get() == allocated_area_start {
                self.search_start.set(allocated_area_end as _);
            }

            if self.search_end.get() == allocated_area_end {
                self.search_end.set(allocated_area_start as _);
            }

            self.add_flags(Self::FLAG_DIRTY);
        }
    }
    fn mark_released_area(&self, released_area_start: u32, released_area_end: u32) {
        let released_area_size = released_area_end - released_area_start;

        // SAFETY: Done behind mutex and pool is valid.
        unsafe {
            (*self.pool).total_area_used -= released_area_size as usize;
        }

        self.area_used
            .set(self.area_used() - released_area_size as u32);
        self.search_start
            .set(self.search_start.get().min(released_area_start));
        self.search_end
            .set(self.search_end.get().max(released_area_end));

        bit_vector_clear(
            self.used_bitvector_mut(),
            released_area_start as _,
            released_area_size as _,
        );
        bit_vector_set_bit(
            self.stop_bitvector_mut(),
            released_area_end as usize - 1,
            false,
        );

        if self.area_used() == 0 {
            self.search_start.set(0);
            self.search_end.set(self.area_size());
            self.largest_unused_area.set(self.area_size());
            self.add_flags(Self::FLAG_EMPTY);
            self.clear_flags(Self::FLAG_DIRTY);
        } else {
            self.add_flags(Self::FLAG_DIRTY);
        }
    }

    fn mark_shrunk_area(&self, shrunk_area_start: u32, shrunk_area_end: u32) {
        let shrunk_area_size = shrunk_area_end - shrunk_area_start;

        // Shrunk area cannot start at zero as it would mean that we have shrunk the first
        // block to zero bytes, which is not allowed as such block must be released instead.
        assert!(shrunk_area_start != 0);
        assert!(shrunk_area_end != self.area_size());

        // SAFETY: Done behind mutex and pool is valid.
        unsafe {
            (*self.pool).total_area_used -= shrunk_area_size as usize;
        }

        self.area_used.set(self.area_used() - shrunk_area_size);
        self.search_start
            .set(self.search_start.get().min(shrunk_area_start));
        self.search_end
            .set(self.search_end.get().max(shrunk_area_end));

        bit_vector_clear(
            &mut self.used_bitvector_mut(),
            shrunk_area_start as _,
            shrunk_area_size as _,
        );
        bit_vector_set_bit(
            &mut self.stop_bitvector_mut(),
            shrunk_area_end as usize - 1,
            false,
        );
        bit_vector_set_bit(
            &mut self.stop_bitvector_mut(),
            shrunk_area_start as usize - 1,
            true,
        );

        self.add_flags(Self::FLAG_DIRTY);
    }
}

impl PartialOrd for JitAllocatorBlock {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.rx_ptr().partial_cmp(&other.rx_ptr())
    }
}

impl Ord for JitAllocatorBlock {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.rx_ptr().cmp(&other.rx_ptr())
    }
}

impl PartialEq for JitAllocatorBlock {
    fn eq(&self, other: &Self) -> bool {
        self.rx_ptr() == other.rx_ptr()
    }
}

impl Eq for JitAllocatorBlock {}
use intrusive_collections::linked_list::LinkedList;
intrusive_adapter!(JitAllocatorBlockAdapter = UnsafeRef<JitAllocatorBlock> : JitAllocatorBlock { node: Link });
intrusive_adapter!(BlockListAdapter = UnsafeRef<JitAllocatorBlock> : JitAllocatorBlock { list_node: intrusive_collections::LinkedListLink });

struct BlockKey {
    rxptr: *const u8,
    block_size: u32,
}

impl PartialEq for BlockKey {
    fn eq(&self, other: &Self) -> bool {
        self.rxptr == other.rxptr
    }
}

impl Eq for BlockKey {}

impl PartialOrd for BlockKey {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BlockKey {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let addr_off = other.rxptr as usize + other.block_size as usize;

        if addr_off <= self.rxptr as usize {
            return core::cmp::Ordering::Less;
        } else if other.rxptr > self.rxptr {
            return core::cmp::Ordering::Greater;
        } else {
            return core::cmp::Ordering::Equal;
        }
    }
}

impl<'a> KeyAdapter<'a> for JitAllocatorBlockAdapter {
    type Key = BlockKey;

    fn get_key(
        &self,
        value: &'a <Self::PointerOps as intrusive_collections::PointerOps>::Value,
    ) -> Self::Key {
        BlockKey {
            rxptr: value.rx_ptr(),
            block_size: value.block_size as _,
        }
    }
}

struct JitAllocatorPool {
    blocks: LinkedList<BlockListAdapter>,
    cursor: *mut JitAllocatorBlock,

    block_count: u32,
    granularity: u16,
    granularity_log2: u8,
    empty_block_count: u8,
    total_area_size: usize,
    total_area_used: usize,
    total_overhead_bytes: usize,
}

impl JitAllocatorPool {
    fn new(granularity: u32) -> Self {
        let granularity_log2 = granularity.trailing_zeros() as u8;
        let granularity = granularity as u16;

        Self {
            blocks: LinkedList::new(BlockListAdapter::new()),
            cursor: core::ptr::null_mut(),
            block_count: 0,
            granularity,
            granularity_log2,
            empty_block_count: 0,
            total_area_size: 0,
            total_area_used: 0,
            total_overhead_bytes: 0,
        }
    }

    fn reset(&mut self) {
        self.blocks.clear();
        self.cursor = core::ptr::null_mut();
        self.block_count = 0;
        self.total_area_size = 0;
        self.total_area_used = 0;
        self.total_overhead_bytes = 0;
    }

    fn byte_size_from_area_size(&self, area_size: u32) -> usize {
        area_size as usize * self.granularity as usize
    }

    fn area_size_from_byte_size(&self, byte_size: usize) -> u32 {
        ((byte_size + self.granularity as usize - 1) >> self.granularity_log2) as u32
    }

    fn bit_word_count_from_area_size(&self, area_size: u32) -> usize {
        align_up(area_size as _, 32) / 32
    }
}
use alloc::boxed::Box;

/// A simple implementation of memory manager that uses [virtual_memory](crate::virtual_memory).
/// functions to manage virtual memory for JIT compiled code.
///
/// Implementation notes:
///
/// - Granularity of allocated blocks is different than granularity for a typical C malloc. In addition, the allocator
///   can use several memory pools having a different granularity to minimize the maintenance overhead. Multiple pools
///   feature requires `use_multiple_pools` flag to be set.
///
/// - The allocator doesn't store any information in executable memory, instead, the implementation uses two
///   bit-vectors to manage allocated memory of each allocator-block. The first bit-vector called 'used' is used to
///   track used memory (where each bit represents memory size defined by granularity) and the second bit vector called
///   'stop' is used as a sentinel to mark where the allocated area ends.
///
/// - Internally, the allocator also uses RB tree to keep track of all blocks across all pools. Each inserted block is
///   added to the tree so it can be matched fast during `release()` and `shrink()`.
pub struct JitAllocator {
    options: JitAllocatorOptions,
    block_size: usize,
    granulariy: usize,
    fill_pattern: u32,

    allocation_count: usize,
    tree: RBTree<JitAllocatorBlockAdapter>,
    pools: Box<[*mut JitAllocatorPool]>,
}

impl JitAllocator {
    /// Creates a new JitAllocator instance.
    pub fn new(params: JitAllocatorOptions) -> Box<Self> {
        let vm_info = virtual_memory::info();

        let mut block_size = params.block_size;
        let mut granularity = params.block_size;

        let mut pool_count = 1;

        if params.use_multiple_pools {
            pool_count = MULTI_POOL_COUNT;
        }

        if block_size < 64 * 1024 || block_size > 256 * 1024 * 1024 || !block_size.is_power_of_two()
        {
            block_size = vm_info.page_granularity as _;
        }

        if granularity < 64 || granularity > 256 || !granularity.is_power_of_two() {
            granularity = MIN_GRANULARITY as _;
        }

        let fill_pattern = params.custom_fill_pattern.unwrap_or(DEFAULT_FILL_PATTERN);

        let mut pools = Vec::with_capacity(pool_count);

        for _ in 0..pool_count {
            pools.push(Box::into_raw(Box::new(JitAllocatorPool::new(granularity))));
        }

        log::debug!("JitAllocator created with={{");
        log::debug!("  dual_mapping={}", params.use_dual_mapping);
        log::debug!("  block_size={}", block_size);
        log::debug!("  granularity={}", granularity);
        log::debug!("}}");

        let allocator = Box::new(Self {
            options: params,
            block_size: block_size as _,
            granulariy: granularity as _,
            fill_pattern,
            allocation_count: 0,
            tree: RBTree::new(JitAllocatorBlockAdapter::new()),
            pools: pools.into_boxed_slice(),
        });

        allocator
    }

    fn size_to_pool_id(&self, size: usize) -> usize {
        let mut pool_id = self.pools.len() - 1;
        let mut granularity = self.granulariy << pool_id;

        while pool_id != 0 {
            if align_up(size, granularity) == size {
                break;
            }

            pool_id -= 1;
            granularity >>= 1;
        }

        pool_id
    }

    fn bitvector_size_to_byte_size(area_size: u32) -> usize {
        ((area_size as usize + 32 - 1) / 32) * size_of::<u32>()
    }

    fn calculate_ideal_block_size(
        &self,
        pool: *mut JitAllocatorPool,
        allocation_size: usize,
    ) -> usize {
        unsafe {
            let last = (*pool).blocks.back();

            let mut block_size = if !last.is_null() {
                last.get().unwrap().block_size()
            } else {
                self.block_size
            };

            if block_size < MAX_BLOCK_SIZE {
                block_size *= 2;
            }

            if allocation_size > block_size {
                block_size = align_up(allocation_size, block_size);

                // overflow
                if block_size < allocation_size {
                    return 0;
                }
            }

            block_size
        }
    }

    unsafe fn new_block(
        &mut self,
        pool: *mut JitAllocatorPool,
        block_size: usize,
    ) -> Result<Box<JitAllocatorBlock>, AsmError> {
        let area_size = (block_size + (*pool).granularity as usize - 1) >> (*pool).granularity_log2;
        let num_bit_words = (area_size + 32 - 1) / 32;

        let mut block = Box::new(JitAllocatorBlock {
            node: Link::new(),
            list_node: intrusive_collections::LinkedListLink::new(),
            pool,
            mapping: DualMapping {
                rx: null_mut(),
                rw: null_mut(),
            },
            block_size: block_size as _,
            flags: Cell::new(0),
            area_size: Cell::new(0),
            area_used: Cell::new(0),
            largest_unused_area: Cell::new(area_size as _),
            search_end: Cell::new(area_size as _),
            search_start: Cell::new(0),
            used_bitvector: UnsafeCell::new({
                let mut v = Vec::with_capacity(num_bit_words);
                v.resize(num_bit_words * size_of::<u32>(), 0);
                v
            }),
            stop_bitvector: UnsafeCell::new({
                let mut v = Vec::with_capacity(num_bit_words);
                v.resize(num_bit_words * size_of::<u32>(), 0);
                v
            }),
        });
        let mut block_flags = 0;
        let virt_mem = if self.options.use_dual_mapping {
            block_flags |= JitAllocatorBlock::FLAG_DUAL_MAPPED;
            alloc_dual_mapping(block_size, MemoryFlags::ACCESS_RWX.into())?
        } else {
            let rx = alloc(block_size, MemoryFlags::ACCESS_RWX.into())?;
            DualMapping { rx, rw: rx }
        };

        if self.options.fill_unused_memory {
            protect_jit_memory(ProtectJitAccess::ReadWrite);
            fill_pattern(virt_mem.rw, self.fill_pattern, block_size);
            protect_jit_memory(ProtectJitAccess::ReadExecute);
            flush_instruction_cache(virt_mem.rx, block_size);
        }

        block.area_size.set(area_size as _);
        block.mapping = virt_mem;
        block.flags.set(block_flags);
        Ok(block)
    }

    unsafe fn delete_block(&mut self, block: *mut JitAllocatorBlock) {
        let mut block = Box::from_raw(block);
        if (block.flags() & JitAllocatorBlock::FLAG_DUAL_MAPPED) != 0 {
            let _ = release_dual_mapping(&mut block.mapping, block.block_size);
        } else {
            let _ = release(block.mapping.rx as _, block.block_size);
        }

        drop(block);
    }

    unsafe fn insert_block(&mut self, block: *mut JitAllocatorBlock) {
        let b = &mut *block;
        let pool = &mut *b.pool();

        if pool.cursor.is_null() {
            pool.cursor = block;
        }

        self.tree.insert(UnsafeRef::from_raw(block));
        pool.blocks.push_front(UnsafeRef::from_raw(block));

        pool.block_count += 1;
        pool.total_area_size += b.area_size() as usize;

        pool.total_overhead_bytes +=
            size_of::<JitAllocatorBlock>() + Self::bitvector_size_to_byte_size(b.area_size()) * 2;
    }

    unsafe fn remove_block(
        &mut self,
        block: &mut intrusive_collections::linked_list::CursorMut<'_, BlockListAdapter>,
    ) -> *mut JitAllocatorBlock {
        let b = block.get().unwrap();
        let pool = &mut *b.pool();

        if pool.cursor == b as *const JitAllocatorBlock as *mut _ {
            pool.cursor = if let Some(block) = block.peek_prev().get() {
                block as *const _ as *mut _
            } else if let Some(block) = block.peek_next().get() {
                block as *const _ as *mut _
            } else {
                null_mut()
            };
        }

        match self.tree.entry(&BlockKey {
            rxptr: b.rx_ptr(),
            block_size: b.block_size as _,
        }) {
            Entry::Occupied(mut c) => {
                assert_eq!(
                    UnsafeRef::into_raw(c.remove().unwrap()),
                    b as *const _ as *mut JitAllocatorBlock,
                    "blocks are not the same"
                );
            }

            _ => (),
        }
        let area_size = b.area_size();

        pool.block_count -= 1;
        pool.total_area_size -= area_size as usize;

        pool.total_overhead_bytes -=
            size_of::<JitAllocatorBlock>() + Self::bitvector_size_to_byte_size(area_size) * 2;

        UnsafeRef::into_raw(block.remove().unwrap())
    }

    unsafe fn wipe_out_block(
        &mut self,
        block: &mut intrusive_collections::linked_list::CursorMut<'_, BlockListAdapter>,
    ) {
        let b = block.get().unwrap();
        if (b.flags() & JitAllocatorBlock::FLAG_EMPTY) != 0 {
            return;
        }

        let pool = &mut *b.pool();

        let area_size = b.area_size();
        let granularity = pool.granularity;

        virtual_memory::protect_jit_memory(ProtectJitAccess::ReadWrite);

        if self.options.fill_unused_memory {
            let rw_ptr = b.rw_ptr();

            let it = BitVectorRangeIterator::from_slice_and_nbitwords(
                &b.stop_bitvector(),
                pool.bit_word_count_from_area_size(b.area_size()),
            );

            for range in it {
                let span_ptr = rw_ptr.add(range.start as usize * granularity as usize);
                let span_size = (range.end as usize - range.start as usize) * granularity as usize;

                let mut n = 0;
                while n < span_size {
                    *span_ptr.add(n).cast::<u32>() = self.fill_pattern;
                    n += size_of::<u32>();
                }

                virtual_memory::flush_instruction_cache(span_ptr, span_size as usize);
            }
        }

        virtual_memory::protect_jit_memory(ProtectJitAccess::ReadExecute);

        let b = &mut *UnsafeRef::into_raw(block.remove().unwrap());
        b.used_bitvector_mut().fill(0);
        b.stop_bitvector_mut().fill(0);

        b.area_used.set(0);
        b.largest_unused_area.set(area_size);
        b.search_start.set(0);
        b.search_end.set(area_size);
        b.add_flags(JitAllocatorBlock::FLAG_EMPTY);
        b.clear_flags(JitAllocatorBlock::FLAG_DIRTY);
    }

    /// Resets current allocator by emptying all pools and blocks.
    ///
    /// Frees all memory is `ResetPolicy::Hard` is specified or `immediate_release` in [JitAllocatorOptions] is specific.
    pub unsafe fn reset(&mut self, reset_policy: ResetPolicy) {
        self.tree.clear();

        let pool_count = self.pools.len();

        for pool_id in 0..pool_count {
            let pool = unsafe { &mut *self.pools[pool_id] };

            let mut cursor = pool.blocks.cursor_mut();
            cursor.move_next();
            let mut block_to_keep = false;
            if reset_policy != ResetPolicy::Hard && !self.options.immediate_release {
                block_to_keep = true;
                cursor.move_next();
            }
            unsafe {
                while !cursor.is_null() {
                    let block = UnsafeRef::into_raw(cursor.remove().unwrap());
                    self.delete_block(block);
                    cursor.move_next();
                }

                pool.reset();

                if block_to_keep {
                    let mut front = pool.blocks.cursor_mut();
                    front.move_next();
                    self.wipe_out_block(&mut front);
                    pool.empty_block_count = 1;
                }
            }
        }
    }

    /// Allocates `size` bytes in the executable memory region.
    /// Returns two pointers. One points to Read-Execute mapping and another to Read-Write mapping.
    /// All code writes *must* go to the Read-Write mapping.
    pub fn alloc(&mut self, size: usize) -> Result<Span, AsmError> {
        const NO_INDEX: u32 = u32::MAX;

        let size = align_up(size, self.granulariy);

        if size == 0 {
            return Err(AsmError::InvalidArgument);
        }

        if size > u32::MAX as usize / 2 {
            return Err(AsmError::TooLarge);
        }

        unsafe {
            let pool_id = self.size_to_pool_id(size);
            let pool = &mut *self.pools[pool_id];

            let mut area_index = NO_INDEX;
            let area_size = pool.area_size_from_byte_size(size);

            let mut block = pool.blocks.cursor();
            block.move_next();
            if let Some(initial) = block.get().map(|x| x as *const JitAllocatorBlock) {
                loop {
                    let b = block.get().unwrap();

                    if b.area_available() >= area_size {
                        if b.is_dirty() || b.largest_unused_area() >= area_size {
                            let mut it = BitVectorRangeIterator::<0>::new(
                                b.used_bitvector(),
                                pool.bit_word_count_from_area_size(b.area_size()),
                                b.search_start() as _,
                                b.search_end() as _,
                            );

                            let mut range_start;
                            let mut range_end = b.area_size() as usize;

                            let mut search_start = usize::MAX;
                            let mut largest_area = 0;

                            while let Some(range) = it.next_range(area_size as _) {
                                range_start = range.start as _;
                                range_end = range.end as _;

                                let range_size = range_end - range_start;

                                if range_size >= area_size as usize {
                                    area_index = range_start as _;
                                    break;
                                }

                                search_start = search_start.min(range_start);
                                largest_area = largest_area.max(range_size);
                            }

                            if area_index != NO_INDEX {
                                break;
                            }

                            if search_start != usize::MAX {
                                let search_end = range_end;

                                b.search_start.set(search_start as _);

                                b.search_end.set(search_end as _);
                                b.largest_unused_area.set(largest_area as _);
                                b.clear_flags(JitAllocatorBlock::FLAG_DIRTY);
                            }
                        }
                    }

                    block.move_next();

                    if block.get().map(|x| x as *const _) == Some(initial) {
                        break;
                    }

                    if block.is_null() {
                        break;
                    }
                }
            }

            let mut block = block.get();

            if area_index == NO_INDEX {
                let block_size = self.calculate_ideal_block_size(pool, size);

                {
                    let nblock = self.new_block(pool, block_size)?;

                    area_index = 0;

                    nblock.search_start.set(area_size as _);

                    nblock
                        .largest_unused_area
                        .set(nblock.area_size() - area_size);

                    let nblock = Box::into_raw(nblock);

                    self.insert_block(nblock);

                    block = Some(&*nblock);
                }
            } else if (block.unwrap().flags() & JitAllocatorBlock::FLAG_EMPTY) != 0 {
                pool.empty_block_count -= 1;
                block.unwrap().clear_flags(JitAllocatorBlock::FLAG_EMPTY);
            }

            self.allocation_count += 1;

            let block = block.unwrap();

            block.mark_allocated_area(area_index, area_index + area_size);

            let offset = pool.byte_size_from_area_size(area_index);

            let rx = block.rx_ptr().add(offset);
            let rw = block.rw_ptr().add(offset);

            Ok(Span {
                block: block as *const JitAllocatorBlock as _,
                rx,
                rw,
                size,
                icache_clean: true,
            })

            //Ok((block.rx_ptr().add(offset), block.rw_ptr().add(offset)))
        }
    }

    /// Releases the memory allocated by `alloc`.
    ///
    /// # SAFETY
    /// - `rx_ptr` must have been returned from `alloc`
    /// - `rx_ptr` must have been allocaetd from this allocator
    /// - `rx_ptr` must not have been passed to `release` before
    /// - `rx_ptr` must point to read-execute part of memory returned from `alloc`.
    pub unsafe fn release(&mut self, rx_ptr: *const u8) -> Result<(), AsmError> {
        if rx_ptr.is_null() {
            return Err(AsmError::InvalidArgument);
        }

        let block = self.tree.find(&BlockKey {
            rxptr: rx_ptr,
            block_size: 0,
        });

        let Some(block) = block.get() else {
            return Err(AsmError::InvalidState);
        };

        unsafe {
            let pool = &mut *block.pool;

            let offset = rx_ptr as usize - block.rx_ptr() as usize;

            let area_index = (offset >> pool.granularity_log2 as usize) as u32;
            let area_end =
                bit_vector_index_of(&block.stop_bitvector(), area_index as _, true) as u32 + 1;
            let area_size = area_end - area_index;

            self.allocation_count -= 1;

            block.mark_released_area(area_index, area_end);

            if self.options.fill_unused_memory {
                let span_ptr = block
                    .rw_ptr()
                    .add(area_index as usize * pool.granularity as usize);
                let span_size = area_size as usize * pool.granularity as usize;

                protect_jit_memory(ProtectJitAccess::ReadWrite);
                fill_pattern(span_ptr, self.fill_pattern, span_size);
                protect_jit_memory(ProtectJitAccess::ReadExecute);
                flush_instruction_cache(span_ptr, span_size);
            }

            if block.area_used() == 0 {
                if pool.empty_block_count != 0 || self.options.immediate_release {
                    let mut cursor = pool.blocks.cursor_mut_from_ptr(block);
                    let block = self.remove_block(&mut cursor);

                    self.delete_block(block);
                } else {
                    pool.empty_block_count += 1;
                }
            }
        }

        Ok(())
    }
    /// Shrinks the memory allocated by `alloc`.
    ///
    /// # SAFETY
    ///
    /// `rx_ptr` must be a pointer returned by `alloc`.
    pub unsafe fn shrink(&mut self, rx_ptr: *const u8, new_size: usize) -> Result<(), AsmError> {
        if rx_ptr.is_null() {
            return Err(AsmError::InvalidArgument);
        }

        if new_size == 0 {
            return self.release(rx_ptr);
        }

        let Some(block) = self
            .tree
            .find(&BlockKey {
                rxptr: rx_ptr,
                block_size: 0,
            })
            .get()
        else {
            return Err(AsmError::InvalidArgument);
        };

        unsafe {
            let pool = &mut *block.pool;
            let offset = rx_ptr as usize - block.rx_ptr() as usize;
            let area_start = (offset >> pool.granularity_log2 as usize) as u32;

            let is_used = bit_vector_get_bit(block.used_bitvector(), area_start as _);

            if !is_used {
                return Err(AsmError::InvalidArgument);
            }

            let area_end =
                bit_vector_index_of(&block.stop_bitvector(), area_start as _, true) as u32 + 1;

            let area_prev_size = area_end - area_start;
            let area_shrunk_size = pool.area_size_from_byte_size(new_size);

            if area_shrunk_size > area_prev_size {
                return Err(AsmError::InvalidState);
            }

            let area_diff = area_prev_size - area_shrunk_size;

            if area_diff != 0 {
                block.mark_shrunk_area(area_start + area_shrunk_size, area_end);

                if self.options.fill_unused_memory {
                    let span_ptr = block
                        .rw_ptr()
                        .add(area_start as usize * pool.granularity as usize);
                    let span_size = area_diff as usize * pool.granularity as usize;

                    protect_jit_memory(ProtectJitAccess::ReadWrite);
                    fill_pattern(span_ptr, self.fill_pattern, span_size);
                    protect_jit_memory(ProtectJitAccess::ReadExecute);
                    flush_instruction_cache(span_ptr, span_size);
                }
            }
        }

        Ok(())
    }

    /// Takes a pointer into the JIT memory and tries to query
    /// RX, RW mappings and size of the allocation.
    pub fn query(&self, rx_ptr: *const u8) -> Result<Span, AsmError> {
        let Some(block) = self
            .tree
            .find(&BlockKey {
                rxptr: rx_ptr,
                block_size: 0,
            })
            .get()
        else {
            return Err(AsmError::InvalidArgument);
        };

        unsafe {
            let pool = &mut *block.pool;
            let offset = rx_ptr as usize - block.rx_ptr() as usize;

            let area_start = (offset >> pool.granularity_log2 as usize) as u32;

            let is_used = bit_vector_get_bit(block.used_bitvector(), area_start as _);

            if !is_used {
                return Err(AsmError::InvalidArgument);
            }

            let area_end =
                bit_vector_index_of(&block.stop_bitvector(), area_start as _, true) as u32 + 1;
            let byte_offset = pool.byte_size_from_area_size(area_start);
            let byte_size = pool.byte_size_from_area_size(area_end - area_start);

            Ok(Span {
                rx: block.rx_ptr().add(byte_offset),
                rw: block.rw_ptr().add(byte_offset),
                size: byte_size,
                icache_clean: false,
                block: block as *const JitAllocatorBlock as _,
            })
        }
    }

    pub unsafe fn write(
        &mut self,
        span: &mut Span,
        mut write_func: impl FnMut(&mut Span),
    ) -> Result<(), AsmError> {
        let size = span.size();

        if size == 0 {
            return Ok(());
        }

        protect_jit_memory(ProtectJitAccess::ReadWrite);
        write_func(span);
        protect_jit_memory(ProtectJitAccess::ReadExecute);
        flush_instruction_cache(span.rx(), span.size());

        if span.size() != size {
            self.shrink(span.rx(), span.size)?;
        }

        Ok(())
    }

    pub unsafe fn copy_from_slice(
        &mut self,
        span: &mut Span,
        offset: usize,
        slice: &[u8],
    ) -> Result<(), AsmError> {
        if slice.len() == 0 {
            return Ok(());
        }

        protect_jit_memory(ProtectJitAccess::ReadWrite);
        span.rw()
            .add(offset)
            .copy_from_nonoverlapping(slice.as_ptr(), slice.len());
        protect_jit_memory(ProtectJitAccess::ReadExecute);
        flush_instruction_cache(span.rx(), span.size());
        Ok(())
    }
}

#[inline]
unsafe fn fill_pattern(mem: *mut u8, pattern: u32, size_in_bytes: usize) {
    let n = size_in_bytes / 4;

    let p = mem as *mut u32;

    for i in 0..n {
        p.add(i).write(pattern);
    }
}

unsafe impl Send for JitAllocator {}

/// A memory reference returned by [`JitAllocator::alloc`]
#[allow(dead_code)]
pub struct Span {
    rx: *const u8,
    rw: *mut u8,
    size: usize,
    block: *mut u8,
    icache_clean: bool,
}

impl Span {
    /// Returns a pointer having Read & Execute permissions (references executable memory).
    ///
    /// This pointer is never NULL if the allocation succeeded, it points to an executable memory.
    pub const fn rx(&self) -> *const u8 {
        self.rx
    }
    /// Returns a pointer having Read & Write permissions (references writable memory).
    ///
    /// Depending on the type of the allocation strategy this could either be:
    ///
    ///   - the same address as returned by `rx()` if the allocator uses RWX mapping (pages have all of Read, Write,
    ///     and Execute permissions) or MAP_JIT, which requires to call [protect_jit_memory()](crate::util::virtual_memory::protect_jit_memory) manually.
    ///   - a valid pointer, but not the same as `rx` - this would be valid if dual mapping is used.
    ///   - NULL pointer, in case that the allocation strategy doesn't use RWX, MAP_JIT, or dual mapping. In this
    ///     case only [JitAllocator] can copy new code into the executable memory referenced by [Span].
    pub const fn rw(&self) -> *mut u8 {
        self.rw
    }

    pub const fn size(&self) -> usize {
        self.size
    }

    pub fn is_icache_clean(&self) -> bool {
        self.icache_clean
    }

    pub fn is_directly_writeable(&self) -> bool {
        !self.rw.is_null()
    }
}
