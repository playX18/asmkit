#!/usr/bin/python3

import argparse
import bisect
from collections import OrderedDict, defaultdict, namedtuple
from enum import Enum
from itertools import product
import re
from typing import NamedTuple, FrozenSet, List, Tuple, Union
from docenizer_amd64 import collect_instruction_docs
from rustbuilder import *


INSTR_FLAGS_FIELDS, INSTR_FLAGS_SIZES = zip(*[
    ("modrm_idx", 2),
    ("modreg_idx", 2),
    ("vexreg_idx", 2), # note: vexreg w/o vex prefix is zeroreg_val
    ("imm_idx", 2),
    ("evex_bcst", 1),
    ("evex_mask", 1),
    ("zeroreg_val", 1),
    ("lock", 1),
    ("imm_control", 3),
    ("vsib", 1),
    ("modrm_size", 2),
    ("modreg_size", 2),
    ("vexreg_size", 2),
    ("imm_size", 2),
    ("legacy", 1),
    ("unused2", 1),
    ("size_fix1", 3),
    ("size_fix2", 2),
    ("instr_width", 1),
    ("modrm_ty", 3),
    ("modreg_ty", 3),
    ("vexreg_ty", 2),
    ("imm_ty", 0),
    ("evex_rc", 2),
    ("evex_bcst16", 1),
    ("opsize", 3),
    ("modrm", 1),
    ("ign66", 1),
][::-1])
class InstrFlags(namedtuple("InstrFlags", INSTR_FLAGS_FIELDS)):
    def __new__(cls, **kwargs):
        init = {**{f: 0 for f in cls._fields}, **kwargs}
        return super(InstrFlags, cls).__new__(cls, **init)
    def _encode(self):
        enc = 0
        for value, size in zip(self, INSTR_FLAGS_SIZES):
            enc = enc << size | (value & ((1 << size) - 1))
        return enc

ENCODINGS = {
    "NP": InstrFlags(),
    "M": InstrFlags(modrm=1, modrm_idx=0^3),
    "R": InstrFlags(modrm=1, modreg_idx=0^3), # AMX TILEZERO
    "M1": InstrFlags(modrm=1, modrm_idx=0^3, imm_idx=1^3, imm_control=1),
    "MI": InstrFlags(modrm=1, modrm_idx=0^3, imm_idx=1^3, imm_control=4),
    "IM": InstrFlags(modrm=1, modrm_idx=1^3, imm_idx=0^3, imm_control=4),
    "MC": InstrFlags(modrm=1, modrm_idx=0^3, vexreg_idx=1^3, zeroreg_val=1),
    "MR": InstrFlags(modrm=1, modrm_idx=0^3, modreg_idx=1^3),
    "RM": InstrFlags(modrm=1, modrm_idx=1^3, modreg_idx=0^3),
    "RMA": InstrFlags(modrm=1, modrm_idx=1^3, modreg_idx=0^3, vexreg_idx=2^3),
    "MRI": InstrFlags(modrm=1, modrm_idx=0^3, modreg_idx=1^3, imm_idx=2^3, imm_control=4),
    "RMI": InstrFlags(modrm=1, modrm_idx=1^3, modreg_idx=0^3, imm_idx=2^3, imm_control=4),
    "MRC": InstrFlags(modrm=1, modrm_idx=0^3, modreg_idx=1^3, vexreg_idx=2^3, zeroreg_val=1),
    "AM": InstrFlags(modrm=1, modrm_idx=1^3, vexreg_idx=0^3),
    "MA": InstrFlags(modrm=1, modrm_idx=0^3, vexreg_idx=1^3),
    "I": InstrFlags(imm_idx=0^3, imm_control=4),
    "IA": InstrFlags(vexreg_idx=0^3, imm_idx=1^3, imm_control=4),
    "O": InstrFlags(modrm_idx=0^3),
    "OI": InstrFlags(modrm_idx=0^3, imm_idx=1^3, imm_control=4),
    "OA": InstrFlags(modrm_idx=0^3, vexreg_idx=1^3),
    "S": InstrFlags(modreg_idx=0^3), # segment register in bits 3,4,5
    "A": InstrFlags(vexreg_idx=0^3),
    "D": InstrFlags(imm_idx=0^3, imm_control=6),
    "FD": InstrFlags(vexreg_idx=0^3, imm_idx=1^3, imm_control=2),
    "TD": InstrFlags(vexreg_idx=1^3, imm_idx=0^3, imm_control=2),

    "RVM": InstrFlags(modrm=1, modrm_idx=2^3, modreg_idx=0^3, vexreg_idx=1^3),
    "RVMI": InstrFlags(modrm=1, modrm_idx=2^3, modreg_idx=0^3, vexreg_idx=1^3, imm_idx=3^3, imm_control=4),
    "RVMR": InstrFlags(modrm=1, modrm_idx=2^3, modreg_idx=0^3, vexreg_idx=1^3, imm_idx=3^3, imm_control=3),
    "RMV": InstrFlags(modrm=1, modrm_idx=1^3, modreg_idx=0^3, vexreg_idx=2^3),
    "VM": InstrFlags(modrm=1, modrm_idx=1^3, vexreg_idx=0^3),
    "VMI": InstrFlags(modrm=1, modrm_idx=1^3, vexreg_idx=0^3, imm_idx=2^3, imm_control=4),
    "MVR": InstrFlags(modrm=1, modrm_idx=0^3, modreg_idx=2^3, vexreg_idx=1^3),
    "MRV": InstrFlags(modrm=1, modrm_idx=0^3, modreg_idx=1^3, vexreg_idx=2^3),
}
ENCODING_OPTYS = ["modrm", "modreg", "vexreg", "imm"]
ENCODING_OPORDER = { enc: sorted(ENCODING_OPTYS, key=lambda ty: getattr(ENCODINGS[enc], ty+"_idx")^3) for enc in ENCODINGS}

OPKIND_CANONICALIZE = {
    "I": "IMM", # immediate
        "A": "IMM", # Direct address, far jmp
    "J": "IMM", # RIP-relative address
    "M": "MEM", # ModRM.r/m selects memory only
        "O": "MEM", # Direct address, FD/TD encoding
    "R": "GP", # ModRM.r/m selects GP
        "B": "GP", # VEX.vvvv selects GP
        "E": "GP", # ModRM.r/m selects GP or memory
        "G": "GP", # ModRM.reg selects GP
    "P": "MMX", # ModRM.reg selects MMX
        "N": "MMX", # ModRM.r/m selects MMX
        "Q": "MMX", # ModRM.r/m selects MMX or memory
    "V": "XMM", # ModRM.reg selects XMM
        "H": "XMM", # VEX.vvvv selects XMM
        "L": "XMM", # bits7:4 of imm8 select XMM
        "U": "XMM", # ModRM.r/m selects XMM
        "W": "XMM", # ModRM.r/m selects XMM or memory
    "S": "SEG", # ModRM.reg selects SEG
    "C": "CR", # ModRM.reg selects CR
    "D": "DR", # ModRM.reg selects DR

    # Custom names
    "F": "FPU", # F is used for RFLAGS by Intel
    "K": "MASK",
    "T": "TMM",
    "Z": "BND",
}
# Mapping from Rust operand type name to a runtime check expression.
# {i} is replaced with the operand parameter name (e.g. "op0", "op1", ...).
DYN_OPTYPE_CHECK = {
    'GpbLo':          'op{i}.is_reg_type_of(RegType::Gp8Lo)',
    'GpbHi':          'op{i}.is_reg_type_of(RegType::Gp8Hi)',
    'Gpw':            'op{i}.is_reg_type_of(RegType::Gp16)',
    'Gpd':            'op{i}.is_gp32()',
    'Gpq':            'op{i}.is_gp64()',
    'Xmm':            'op{i}.is_vec128()',
    'Ymm':            'op{i}.is_vec256()',
    'Zmm':            'op{i}.is_vec512()',
    'Mm':             'op{i}.is_reg_group_of(RegGroup::X86MM)',
    'KReg':           'op{i}.is_mask()',
    'CReg':           'op{i}.is_reg_group_of(RegGroup::X86CReg)',
    'DReg':           'op{i}.is_reg_group_of(RegGroup::X86DReg)',
    'St':             'op{i}.is_reg_group_of(RegGroup::X86St)',
    'SReg':           'op{i}.is_reg_group_of(RegGroup::X86SReg)',
    'Bnd':            'op{i}.is_reg_group_of(RegGroup::X86Bnd)',
    'Tmm':            'op{i}.is_reg_group_of(RegGroup::X86Tmm)',
    'Mem':            'op{i}.is_mem()',
    'Imm':            'op{i}.is_imm()',
    'Label':          'op{i}.is_label()',
    'Sym':            'op{i}.is_sym()',
    'AbsoluteAddress':'op{i}.is_mem()',
}

OPKIND_SIZES = {
    "b": 1,
    "w": 2,
    "d": 4,
    "ss": 4, # Scalar single of XMM (d)
    "q": 8,
    "sd": 8, # Scalar double of XMM (q)
    "t": 10, # FPU/ten-byte
    "dq": 16,
    "qq": 32,
    "oq": 64, # oct-quadword
    "": 0, # for MEMZ
    "v": -1, # operand size (w/d/q)
    "y": -1, # operand size (d/q)
    "z": -1, # w/d (immediates, min(operand size, 4))
    "a": -1, # z:z
    "p": -1, # w:z
    "x": -2, # vector size
    "h": -3, # half x
    "f": -4, # fourth x
    "e": -5, # eighth x
    "pd": -2, # packed double (x)
    "ps": -2, # packed single (x)

    # Custom names
    "bs": -1, # sign-extended immediate
    "zd": 4, # z-immediate, but always 4-byte operand
    "zq": 8, # z-immediate, but always 8-byte operand
}
class OpKind(NamedTuple):
    regkind: str
    sizestr: str

    SZ_OP = -1
    SZ_VEC = -2
    SZ_VEC_HALF = -3
    SZ_VEC_QUARTER = -4
    SZ_VEC_EIGHTH = -5

    def abssize(self, opsz=None, vecsz=None):
        res = opsz if self.size == self.SZ_OP else \
              vecsz if self.size == self.SZ_VEC else \
              vecsz >> 1 if self.size == self.SZ_VEC_HALF else \
              vecsz >> 2 if self.size == self.SZ_VEC_QUARTER else \
              vecsz >> 3 if self.size == self.SZ_VEC_EIGHTH else self.size
        if res is None:
            raise Exception("unspecified operand size")
        return res
    def immsize(self, opsz):
        maxsz = 1 if self.sizestr == "bs" else 4 if self.sizestr[0] == "z" else 8
        return min(maxsz, self.abssize(opsz))
    @property
    def kind(self):
        return OPKIND_CANONICALIZE[self.regkind]
    @property
    def size(self):
        return OPKIND_SIZES[self.sizestr]
    @classmethod
    def parse(cls, op):
        return cls(op[0], op[1:])

    def __eq__(self, other):
        # Custom equality for canonicalization of kind/size.
        return isinstance(other, OpKind) and self.kind == other.kind and self.size == other.size

class InstrDesc(NamedTuple):
    mnemonic: str
    encoding: str
    operands: Tuple[str, ...]
    flags: FrozenSet[str]

    OPKIND_REGTYS = {
        ("modrm", "GP"): 1,   ("modreg", "GP"): 1,   ("vexreg", "GP"): 1,
        ("modrm", "XMM"): 0,  ("modreg", "XMM"): 0,  ("vexreg", "XMM"): 0,
        ("modrm", "MMX"): 5,  ("modreg", "MMX"): 5,
        ("modrm", "FPU"): 4,                         ("vexreg", "FPU"): 3,
        ("modrm", "TMM"): 6,  ("modreg", "TMM"): 6,  ("vexreg", "TMM"): 3,
        ("modrm", "MASK"): 7, ("modreg", "MASK"): 7, ("vexreg", "MASK"): 2,
                              ("modreg", "SEG"): 3,
                              ("modreg", "DR"): 0, # handled in code
                              ("modreg", "CR"): 0, # handled in code
        ("modrm", "MEM"): 0,
        ("imm", "MEM"): 0, ("imm", "IMM"): 0, ("imm", "XMM"): 0,
    }
    OPKIND_SIZES = {
        0: 0, 1: 1, 2: 2, 4: 3, 8: 4, 16: 5, 32: 6, 64: 7, 10: 0,
        # OpKind.SZ_OP: -2, OpKind.SZ_VEC: -3, OpKind.SZ_HALFVEC: -4,
    }

    @classmethod
    def parse(cls, desc):
        desc = desc.split()
        mnem, _, compactDesc = desc[5].partition("+")
        flags = frozenset(desc[6:] + [{
            "w": "INSTR_WIDTH",
            "a": "U67",
            "s": "USEG",
            "k": "MASK",
            "b": "BCST",
            "e": "SAE",
            "r": "ER",
        }[c] for c in compactDesc])
        operands = tuple(OpKind.parse(op) for op in desc[1:5] if op != "-")
        return cls(mnem, desc[0], operands, flags)

    def imm_size(self, opsz):
        flags = ENCODINGS[self.encoding]
        if flags.imm_control < 3:
            return 0
        if flags.imm_control == 3:
            return 1
        if self.mnemonic == "ENTER":
            return 3
        return self.operands[flags.imm_idx^3].immsize(opsz)

    def dynsizes(self):
        dynopsz = set(op.size for op in self.operands if op.size < 0)
        if {"INSTR_WIDTH", "SZ8"} & self.flags: dynopsz.add(OpKind.SZ_OP)
        if OpKind.SZ_OP in dynopsz and len(dynopsz) > 1:
            raise Exception(f"conflicting dynamic operand sizes in {self}")
        return dynopsz

    def encode(self, mnem, ign66, modrm):
        flags = ENCODINGS[self.encoding]
        extraflags = {}

        dynopsz = self.dynsizes()
        # Operand size either refers to vectors or GP, but not both
        if dynopsz and OpKind.SZ_OP not in dynopsz: # Vector operand size
            if self.flags & {"SZ8", "D64", "F64", "INSTR_WIDTH", "LOCK", "U66"}:
                raise Exception(f"incompatible flags in {self}")
            # Allow at most the vector size together with one alternative
            dynsizes = [OpKind.SZ_VEC] + list(dynopsz - {OpKind.SZ_VEC})
            extraflags["opsize"] = 4 | (OpKind.SZ_VEC - dynsizes[-1])
            if len(dynsizes) > 2:
                raise Exception(f"conflicting vector operand sizes in {self}")
        else: # either empty or GP operand size
            dynsizes = [OpKind.SZ_OP]
            if "SZ8" in self.flags:
                dynsizes = []
            if "D64" in self.flags: extraflags["opsize"] = 2
            if "F64" in self.flags: extraflags["opsize"] = 3
            extraflags["lock"] = "LOCK" in self.flags

        if (self.flags & {"SZ8", "INSTR_WIDTH"} or
            mnem in ("MOVSX", "MOVZX", "XCHG_NOP", "3DNOW")):
            extraflags["legacy"] = 1
            # INSTR_WIDTH defaults to zero, so only enable when SZ8 is unset
            if "INSTR_WIDTH" in self.flags and "SZ8" not in self.flags:
                extraflags["instr_width"] = 1

        imm_byte = self.imm_size(4) == 1
        extraflags["imm_control"] = flags.imm_control | imm_byte

        # Sort fixed sizes encodable in size_fix2 as second element.
        # But: byte-sized immediates are handled specially and don't cost space.
        fixed = set(self.OPKIND_SIZES[op.size] for op in self.operands if
                    op.size >= 0 and not (imm_byte and op.kind == "IMM"))
        fixed = sorted(fixed, key=lambda x: 1 <= x <= 4)
        if len(fixed) > 2 or (len(fixed) == 2 and not (1 <= fixed[1] <= 4)):
            raise Exception(f"invalid fixed sizes {fixed} in {self}")
        sizes = (fixed + [1, 1])[:2] + dynsizes # See operand_sizes in decode.c.
        extraflags["size_fix1"] = sizes[0]
        extraflags["size_fix2"] = sizes[1] - 1

        for i, opkind in enumerate(self.operands):
            sz = self.OPKIND_SIZES[opkind.size] if opkind.size >= 0 else opkind.size
            if opkind.kind == "IMM":
                if imm_byte and sz not in [1] + dynsizes[:1]:
                    raise Exception(f"imm_byte with opsize {sz} in {self}")
                extraflags[f"imm_size"] = sz == 1 if imm_byte else sizes.index(sz)
            else:
                opname = ENCODING_OPORDER[self.encoding][i]
                extraflags[f"{opname}_size"] = sizes.index(sz)
                extraflags[f"{opname}_ty"] = self.OPKIND_REGTYS[opname, opkind.kind]

        # Miscellaneous Flags
        if "VSIB" in self.flags:        extraflags["vsib"] = 1
        if "BCST" in self.flags:        extraflags["evex_bcst"] = 1
        if "BCST16" in self.flags:      extraflags["evex_bcst16"] = 1
        if "MASK" in self.flags:        extraflags["evex_mask"] = 1
        if "SAE" in self.flags:         extraflags["evex_rc"] = 1
        if "ER" in self.flags:          extraflags["evex_rc"] = 3
        if modrm:                       extraflags["modrm"] = 1

        if "U66" not in self.flags and (ign66 or "I66" in self.flags):
            extraflags["ign66"] = 1

        enc = flags._replace(**extraflags)._encode()
        enc = tuple((enc >> i) & 0xffff for i in range(0, 48, 16))
        # First 2 bytes are the mnemonic, last 6 bytes are the encoding.
        return f"InstDesc::new(Opcode::{mnem}, {enc[0]}, {enc[1]}, {enc[2]})"

class EntryKind(Enum):
    NONE = 0x00
    PREFIX = 0x10
    INSTR = 0x20
    WEAKINSTR = 0x30
    TABLE16 = 0x01
    TABLE8E = 0x11
    ESCAPE = 0x02
    TABLE256 = 0x12
    TABLE_VEX = 0x22
    TABLE_PREFIX = 0x03
    TABLE_ROOT = -1
    @property
    def is_table(self):
        return self != EntryKind.INSTR and self != EntryKind.WEAKINSTR and self != EntryKind.PREFIX

opcode_regex = re.compile(
     r"^(?:(?P<prefixes>(?P<vex>E?VEX\.)?(?P<legacy>NP|66|F2|F3|NFx)\." +
                     r"(?:W(?P<rexw>[01])\.)?(?:L(?P<vexl>0|1|12|2|IG)\.)?))?" +
     r"(?P<escape>0f38|0f3a|0f|M[567]\.|)" +
     r"(?P<opcode>[0-9a-f]{2})" +
     r"(?:/(?P<modreg>[0-7]|[rm][0-7]?|[0-7][rm])|(?P<opcext>[c-f][0-9a-f]))?(?P<extended>\+)?$")

class Opcode(NamedTuple):
    prefix: Union[None, str] # None/NP/66/F2/F3/NFx
    escape: int # [0, 0f, 0f38, 0f3a]
    opc: int
    extended: bool # Extend opc or opcext in ModRM.rm, if present
    # Fixed ModRM.mod ("r"/"m"), ModRM.reg, ModRM.rm (opcext + AMX)
    modrm: Tuple[Union[None, str], Union[None, int], Union[None, int]]
    vex: int # 0 = legacy, 1 = VEX, 2 = EVEX
    vexl: Union[str, None] # 0, 1, 12, 2, IG, None = used, both
    rexw: Union[str, None] # 0, 1, None = both (or ignored)

    @classmethod
    def parse(cls, opcode_string):
        match = opcode_regex.match(opcode_string)
        if match is None:
            raise Exception(opcode_string)
            return None

        opcext = int(match.group("opcext") or "0", 16)
        modreg = match.group("modreg")
        if opcext:
            modrm = "r", (opcext >> 3) & 7, opcext & 7
        elif modreg:
            if modreg[0] in "rm":
                modrm = modreg[0], None, int(modreg[1:]) if modreg[1:] else None
            else:
                modrm = modreg[1:] or None, int(modreg[0]), None
        else:
            modrm = None, None, None

        return cls(
            prefix=match.group("legacy"),
            escape=["", "0f", "0f38", "0f3a", "M4.", "M5.", "M6.", "M7."].index(match.group("escape")),
            opc=int(match.group("opcode"), 16),
            extended=match.group("extended") is not None,
            modrm=modrm,
            vex=[None, "VEX.", "EVEX."].index(match.group("vex")),
            vexl=match.group("vexl"),
            rexw=match.group("rexw"),
        )

def verifyOpcodeDesc(opcode, desc):
    flags = ENCODINGS[desc.encoding]
    oporder = ENCODING_OPORDER[desc.encoding]
    expected_immkinds = ["", "I", "O", "L", "IA", "", "J"][flags.imm_control]
    fixed_mod = opcode.modrm[0]
    if opcode.extended or desc.mnemonic in ("MOV_CR2G", "MOV_DR2G", "MOV_G2CR", "MOV_G2DR"):
        fixed_mod = "r"
    expected_modrmkinds = {None: "EQWFKT", "r": "RNUFKT", "m": "M"}[fixed_mod]
    # allow F and R for zeroreg, which we overlap with vexreg
    expected_vexkinds = "BHKT" if opcode.vex else "BHRF"
    for i, opkind in enumerate(desc.operands):
        if oporder[i] == "modrm" and opkind.regkind not in expected_modrmkinds:
            raise Exception(f"modrm operand-regkind mismatch {opcode}, {desc}")
        if oporder[i] == "modreg" and opkind.regkind not in "GPVSCDFKT":
            raise Exception(f"modreg operand-regkind mismatch {opcode}, {desc}")
        if oporder[i] == "vexreg" and opkind.regkind not in expected_vexkinds:
            raise Exception(f"vexreg operand-regkind mismatch {opcode}, {desc}")
        if oporder[i] == "imm" and opkind.regkind not in expected_immkinds:
            raise Exception(f"imm operand-regkind mismatch {opcode}, {desc}")
    if "INSTR_WIDTH" in desc.flags and len(desc.operands) > 3:
        raise Exception(f"+w with four operands {opcode}, {desc}")
    if opcode.escape == 2 and flags.imm_control != 0:
        raise Exception(f"0f38 has no immediate operand {opcode}, {desc}")
    if opcode.escape == 3 and desc.imm_size(4) != 1:
        raise Exception(f"0f3a must have immediate byte {opcode}, {desc}")
    if opcode.escape == 0 and opcode.prefix is not None:
        raise Exception(f"unescaped opcode has prefix {opcode}, {desc}")
    if opcode.escape == 0 and opcode.vexl is not None:
        raise Exception(f"unescaped opcode has L specifier {opcode}, {desc}")
    if opcode.escape == 0 and opcode.rexw is not None:
        raise Exception(f"unescaped opcode has W specifier {opcode}, {desc}")
    if opcode.escape == 0 and opcode.vex:
        raise Exception(f"VEX opcode without escape {opcode}, {desc}")
    if opcode.vex and opcode.extended:
        raise Exception(f"VEX/EVEX must not be extended {opcode}, {desc}")
    if opcode.vex and opcode.prefix not in ("NP", "66", "F2", "F3"):
        raise Exception(f"VEX/EVEX must have mandatory prefix {opcode}, {desc}")
    if opcode.vexl == "IG" and desc.dynsizes() - {OpKind.SZ_OP}:
        raise Exception(f"(E)VEX.LIG with dynamic vector size {opcode}, {desc}")
    if "VSIB" in desc.flags and opcode.modrm[0] != "m":
        raise Exception(f"VSIB for non-memory opcode {opcode}, {desc}")
    if opcode.vex == 2 and flags.vexreg_idx:
        # Checking this here allows to omit check for V' in decoder.
        if desc.operands[flags.vexreg_idx ^ 3].kind != "XMM":
            raise Exception(f"EVEX.vvvv must refer to XMM {opcode}, {desc}")
    if opcode.vex == 2 and flags.modreg_idx and flags.modreg_idx ^ 3 != 0:
        # EVEX.z=0 is only checked for mask operands in ModReg
        if desc.operands[flags.modreg_idx ^ 3].kind == "MASK":
            raise Exception(f"ModRM.reg mask not first operand {opcode}, {desc}")
    # Verify tuple type
    if opcode.vex == 2 and opcode.modrm[0] != "r":
        tts = [s for s in desc.flags if s.startswith("TUPLE")]
        if len(tts) != 1:
            raise Exception(f"missing tuple type in {opcode}, {desc}")
        if flags.modrm_idx == 3 ^ 3:
            raise Exception(f"missing memory operand {opcode}, {desc}")
        # From Intel SDM
        bcst, evexw, vszs = {
            "TUPLE_FULL_16":      (2,    "0",  (  16,   32,   64)),
            "TUPLE_FULL_32":      (4,    "0",  (  16,   32,   64)),
            "TUPLE_FULL_64":      (8,    "1",  (  16,   32,   64)),
            "TUPLE_HALF_16":      (2,    "0",  (   8,   16,   32)),
            "TUPLE_HALF_32":      (4,    "0",  (   8,   16,   32)),
            "TUPLE_HALF_64":      (8,    "1",  (   8,   16,   32)),
            "TUPLE_QUARTER_16":   (2,    "0",  (   4,    8,   16)),
            "TUPLE_FULL_MEM":     (None, None, (  16,   32,   64)),
            "TUPLE_HALF_MEM":     (None, None, (   8,   16,   32)),
            "TUPLE_QUARTER_MEM":  (None, None, (   4,    8,   16)),
            "TUPLE_EIGHTH_MEM":   (None, None, (   2,    4,    8)),
            "TUPLE1_SCALAR_8":    (None, None, (   1,    1,    1)),
            "TUPLE1_SCALAR_16":   (None, None, (   2,    2,    2)),
            "TUPLE1_SCALAR_32":   (None, "0",  (   4,    4,    4)),
            "TUPLE1_SCALAR_64":   (None, "1",  (   8,    8,    8)),
            "TUPLE1_SCALAR_OPSZ": (None, None, (   0,    0,    0)),
            "TUPLE1_FIXED_32":    (None, None, (   4,    4,    4)),
            "TUPLE1_FIXED_64":    (None, None, (   8,    8,    8)),
            "TUPLE2_32":          (None, "0",  (   8,    8,    8)),
            "TUPLE2_64":          (None, "1",  (None,   16,   16)),
            "TUPLE4_32":          (None, "0",  (None,   16,   16)),
            "TUPLE4_64":          (None, "1",  (None, None,   32)),
            "TUPLE8_32":          (None, "0",  (None, None,   32)),
            "TUPLE_MEM128":       (None, None, (  16,   16,   16)),
            # TODO: Fix MOVDDUP tuple size :(
            "TUPLE_MOVDDUP":      (None, None, (  16,   32,   64)),
        }[tts[0]]
        if "BCST" in desc.flags:
            if bcst is None:
                raise Exception(f"broadcast on incompatible type {opcode}, {desc}")
            if ("BCST16" in desc.flags) != (bcst == 2):
                raise Exception(f"bcst16 mismatch, should be {bcst} {opcode}, {desc}")
        # EVEX.W is used to distinguish 4/8-byte broadcast size
        if evexw and opcode.rexw != evexw:
            raise Exception(f"incompatible EVEX.W {opcode}, {desc}")
        for l, tupsz in enumerate(vszs):
            opsz = desc.operands[flags.modrm_idx ^ 3].abssize(0, 16 << l)
            if tupsz is not None and opsz != tupsz:
                raise Exception(f"memory size {opsz} != {tupsz} {opcode}, {desc}")

class Trie:
    KIND_ORDER = (EntryKind.TABLE_ROOT, EntryKind.ESCAPE, EntryKind.TABLE256,
                  EntryKind.TABLE_PREFIX, EntryKind.TABLE16,
                  EntryKind.TABLE8E, EntryKind.TABLE_VEX)
    TABLE_LENGTH = {
        EntryKind.TABLE_ROOT: 256,
        EntryKind.ESCAPE: 8,
        EntryKind.TABLE256: 256,
        EntryKind.TABLE_PREFIX: 4,
        EntryKind.TABLE16: 16,
        EntryKind.TABLE8E: 8,
        EntryKind.TABLE_VEX: 8,
    }

    def __init__(self, root_count):
        self.trie = []
        self.trie.append([None] * root_count)
        self.kindmap = defaultdict(list)

    def _add_table(self, kind):
        self.trie.append([None] * self.TABLE_LENGTH[kind])
        self.kindmap[kind].append(len(self.trie) - 1)
        return len(self.trie) - 1

    def _clone(self, elem):
        if not elem or not elem[0].is_table:
            return elem
        new_num = self._add_table(elem[0])
        self.trie[new_num] = [self._clone(e) for e in self.trie[elem[1]]]
        return elem[0], new_num

    def _transform_opcode(self, opc):
        realopcext = opc.extended and opc.modrm[2] is None
        topc = [opc.opc + i for i in range(8 if realopcext else 1)]
        if opc.escape == 0 and opc.opc in (0xc4, 0xc5, 0x62):
            assert opc.prefix is None
            assert opc.modrm == ("m", None, None)
            assert opc.rexw is None
            assert opc.vexl is None
            # We do NOT encode /m, this is handled by prefix code.
            # Order must match KIND_ORDER.
            return topc, [0], None, None, None, None, None
        elif opc.escape == 0:
            troot, tescape, topc = topc, None, None
        else:
            troot = [[0x0f], [0xc4, 0xc5], [0x62]][opc.vex]
            tescape = [opc.escape]

        tprefix, t16, t8e, tvex = None, None, None, None
        if opc.prefix == "NFx":
            tprefix = [0, 1]
        elif opc.prefix:
            tprefix = [["NP", "66", "F3", "F2"].index(opc.prefix)]
        if opc.modrm != (None, None, None):
            # TODO: optimize for /r and /m specifiers to reduce size
            mod = {"m": [0], "r": [1], None: [0, 1]}[opc.modrm[0]]
            reg = [opc.modrm[1]] if opc.modrm[1] is not None else list(range(8))
            t16 = [x + (y << 1) for x in mod for y in reg]
            if opc.modrm[2] is not None and not opc.extended:
                t8e = [opc.modrm[2]]
        if opc.rexw is not None or (opc.vexl or "IG") != "IG":
            rexw = {"0": [0], "1": [1<<0], None: [0, 1<<0]}[opc.rexw]
            if opc.vex < 2:
                vexl = {"0": [0], "1": [1<<1], "IG": [0, 1<<1]}[opc.vexl or "IG"]
            else:
                vexl = {"0": [0], "12": [1<<1, 2<<1], "2": [2<<1], "IG": [0, 1<<1, 2<<1, 3<<1]}[opc.vexl or "IG"]
            tvex = list(map(sum, product(rexw, vexl)))
        # Order must match KIND_ORDER.
        return troot, tescape, topc, tprefix, t16, t8e, tvex

    def add_opcode(self, opcode, descidx, root_idx, weak=False):
        opcode = self._transform_opcode(opcode)
        frontier = [(0, root_idx)]
        for elem_kind, elem in zip(self.KIND_ORDER, opcode):
            new_frontier = []
            for entry_num, entry_idx in frontier:
                entry = self.trie[entry_num]
                if elem is None:
                    if entry[entry_idx] is None or entry[entry_idx][0] != elem_kind:
                        new_frontier.append((entry_num, entry_idx))
                        continue
                    elem = list(range(self.TABLE_LENGTH[elem_kind]))
                if entry[entry_idx] is None:
                    new_num = self._add_table(elem_kind)
                    entry[entry_idx] = elem_kind, new_num
                elif entry[entry_idx][0] != elem_kind:
                    # Need to add a new node here and copy entry one level below
                    new_num = self._add_table(elem_kind)
                    # Keep original entry, but clone others recursively
                    self.trie[new_num][0] = entry[entry_idx]
                    for i in range(1, len(self.trie[new_num])):
                        self.trie[new_num][i] = self._clone(entry[entry_idx])
                    entry[entry_idx] = elem_kind, new_num
                for elem_idx in elem:
                    new_frontier.append((entry[entry_idx][1], elem_idx))
            frontier = new_frontier
        for entry_num, entry_idx in frontier:
            entry = self.trie[entry_num]
            if not entry[entry_idx] or entry[entry_idx][0] == EntryKind.WEAKINSTR:
                kind = EntryKind.INSTR if not weak else EntryKind.WEAKINSTR
                entry[entry_idx] = kind, descidx << 2
            elif not weak:
                raise Exception(f"redundant non-weak {opcode}")

    def add_prefix(self, byte, prefix, root_idx):
        if self.trie[0][root_idx] is None:
            self.trie[0][root_idx] = EntryKind.TABLE_ROOT, self._add_table(EntryKind.TABLE_ROOT)
        self.trie[self.trie[0][root_idx][1]][byte] = EntryKind.PREFIX, prefix

    def deduplicate(self):
        synonyms = {}
        for kind in self.KIND_ORDER[::-1]:
            entries = {}
            for num in self.kindmap[kind]:
                # Replace previous synonyms
                entry = self.trie[num]
                for i, elem in enumerate(entry):
                    if elem and elem[0].is_table and elem[1] in synonyms:
                        entry[i] = synonyms[elem[1]]

                unique_entry = tuple(entry)
                if len(set(unique_entry)) == 1:
                    # Omit kind if all entries point to the same child
                    synonyms[num] = entry[0]
                    self.trie[num] = None
                elif unique_entry in entries:
                    # Deduplicate entries of this kind
                    synonyms[num] = kind, entries[unique_entry]
                    self.trie[num] = None
                else:
                    entries[unique_entry] = num

    def compile(self):
        offsets = [None] * len(self.trie)
        last_off = 0
        for num, entry in enumerate(self.trie[1:], start=1):
            if not entry:
                continue
            offsets[num] = last_off
            last_off += (len(entry) + 3) & ~3
        if last_off >= 0x8000:
            raise Exception(f"maximum table size exceeded: {last_off:#x}")

        data = [0] * last_off
        for off, entry in zip(offsets, self.trie):
            if off is None:
                continue
            for i, elem in enumerate(entry, start=off):
                if elem is not None:
                    value = offsets[elem[1]] if elem[0].is_table else elem[1]
                    data[i] = value | (elem[0].value & 3)
        return tuple(data), [offsets[v] for _, v in self.trie[0]]

    @property
    def stats(self):
        return {k.name: sum(self.trie[e] is not None for e in v)
                for k, v in self.kindmap.items()}


def superstring(strs):
    # This faces the "shortest superstring" problem, which is NP-hard.
    # Preprocessing: remove any strings which are already completely covered
    realstrs = []
    for s in sorted(strs, key=len, reverse=True):
        for s2 in realstrs:
            if s in s2:
                break
        else:
            realstrs.append(s)

    # Greedy heuristic generally yields acceptable results, though it depends on
    # the order of the menmonics. More compact results are possible, but the
    # expectable gains of an optimal result (probably with O(n!)) are small.
    # First sort strings and later do a binary search for each possible prefix.
    realstrs.sort()
    merged = ""
    while realstrs:
        for i in range(min(16, len(merged)), 0, -1):
            idx = bisect.bisect_left(realstrs, merged[-i:])
            if idx < len(realstrs) and realstrs[idx][:i] == merged[-i:]:
                merged += realstrs.pop(idx)[i:]
                break
        else:
            merged += realstrs.pop()
    return merged

def decode_table(entries, args):
    modes = args.modes

    trie = Trie(root_count=len(modes))
    for i, mode in enumerate(modes):
        # Magic values must match PF_* enum in decode.c.
        trie.add_prefix(0x66, 0xfffa, i)
        trie.add_prefix(0x67, 0xfffb, i)
        trie.add_prefix(0xf0, 0xfffc, i)
        trie.add_prefix(0xf2, 0xfffd, i)
        trie.add_prefix(0xf3, 0xfffd, i)
        trie.add_prefix(0x64, 0xfff9, i)
        trie.add_prefix(0x65, 0xfff9, i)
        for seg in (0x26, 0x2e, 0x36, 0x3e):
            trie.add_prefix(seg, 0xfff8 + (mode <= 32), i)
        if mode > 32:
            for rex in range(0x40, 0x50):
                trie.add_prefix(rex, 0xfffe, i)

    # pause is hardcoded together with XCHG_NOP.
    mnems, descs, desc_map = {"PAUSE"}, [], {}
    descs.append("InstDesc::invalid()") # desc index zero is "invalid"
    for weak, opcode, desc in entries:
        ign66 = opcode.prefix in ("NP", "66", "F2", "F3")
        modrm = opcode.modrm != (None, None, None)
        mnem = {
            "PUSH_SEG": "PUSH", "POP_SEG": "POP",
            "MOV_CR2G": "MOV_CR", "MOV_G2CR": "MOV_CR",
            "MOV_DR2G": "MOV_DR", "MOV_G2DR": "MOV_DR",
            "MMX_MOVD_M2G": "MMX_MOVD", "MMX_MOVD_G2M": "MMX_MOVD",
            "MMX_MOVQ_M2G": "MMX_MOVQ", "MMX_MOVQ_G2M": "MMX_MOVQ",
            "SSE_MOVD_X2G": "SSE_MOVD", "SSE_MOVD_G2X": "SSE_MOVD",
            "SSE_MOVQ_X2G": "SSE_MOVQ", "SSE_MOVQ_G2X": "SSE_MOVQ",
            "VMOVD_X2G": "VMOVD", "VMOVD_G2X": "VMOVD",
            "VMOVQ_X2G": "VMOVQ", "VMOVQ_G2X": "VMOVQ",
        }.get(desc.mnemonic, desc.mnemonic)
        mnems.add(mnem)
        descenc = desc.encode(mnem, ign66, modrm)
        desc_idx = desc_map.get(descenc)
        if desc_idx is None:
            desc_idx = desc_map[descenc] = len(descs)
            descs.append(descenc)
        for i, mode in enumerate(modes):
            if "IO"[mode <= 32]+"64" not in desc.flags:
                trie.add_opcode(opcode, desc_idx, i, weak)

    trie.deduplicate()
    table_data, root_offsets = trie.compile()

    mnems = sorted(mnems)
    decode_mnems_lines = [f"    {m if not m.startswith('3') else '_' + m} = {i}" for i, m in enumerate(mnems)]

    mnemonics_intel = [m.replace("SSE_", "").replace("MMX_", "")
                        .replace("EVX_", "V")
                        .replace("MOVABS", "MOV").replace("RESERVED_", "")
                        .replace("JMPF", "JMP FAR").replace("CALLF", "CALL FAR")
                        .replace("_S2G", "").replace("_G2S", "")
                        .replace("_X2G", "").replace("_G2X", "")
                        .replace("_CR", "").replace("_DR", "")
                        .replace("REP_", "REP ").replace("CMPXCHGD", "CMPXCHG")
                        .replace("JCXZ", "JCXZ JECXZJRCXZ")
                        .replace("C_SEP", "CWD CDQ CQO")
                        .replace("C_EX", "CBW CWDECDQE").replace("XCHG_NOP", "")
                        .lower() for m in mnems]
    mnemonics_str = superstring(mnemonics_intel)

    if args.stats:
        print(f"Decode stats: Descs -- {len(descs)} ({8*len(descs)} bytes); ",
              f"Trie -- {2*len(table_data)} bytes, {trie.stats}; "
              f"Mnems -- {len(mnemonics_str)} + {3*len(mnemonics_intel)} bytes")

    defines = ["pub const DECODE_TABLE_OFFSET_%d: usize = %d;\n"%k for k in zip(modes, root_offsets)]

    return "#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)] pub enum Opcode {\n#[default]" + (",\n".join(decode_mnems_lines)) + "\n}", f"""
pub static DECODE_TABLE: &[u16] = &[
{"".join(f"{e:#06x}," for e in table_data)}
];
pub static TABLE_DESCS: &[InstDesc] = &[
{"  ,".join(descs)}];
pub static STRTAB1: &str =
"{mnemonics_str}";
pub static STRTAB2: &[u16] = &[
{",".join(str(mnemonics_str.index(mnem)) for mnem in mnemonics_intel)}
];
pub static STRTAB3: &[u16] = &[
{",".join(str(len(mnem)) for mnem in mnemonics_intel)}
];
{"".join("" + line for line in defines)}
"""

class EncodeVariant(NamedTuple):
    opcode: Opcode
    desc: InstrDesc
    evexbcst: bool = False
    evexmask: int = 0 # 0 = none, 1 = must have mask, 2 = mask + EVEX.z
    evexsae: int = 0 # 0 = no EVEX.b, 1 = EVEX.b, 2 = EVEX.b + L'L is rounding mode
    evexdisp8scale: int = 0 # EVEX disp8 shift
    downgrade: int = 0 # 0 = none, 1 = to VEX, 2 = to VEX flipping REXW
    flexcc: bool = False # Flexible condition code

def encode_mnems(entries):
    # mapping from (mnem, opsize, ots) -> (opcode, desc)
    mnemonics = defaultdict(list)
    # Cannot have PAUSE in instrs.txt, because opcodes in without escape must
    # not have mandatory prefixes. For decode, this is hardcoded.
    mnemonics["PAUSE", 0, ""] = [EncodeVariant(Opcode.parse("F3.90"), InstrDesc.parse("NP - - - - NOP"))]
    for weak, opcode, desc in entries:
        if "I64" in desc.flags or desc.mnemonic[:9] == "RESERVED_":
            continue
        mnem_name = {"MOVABS": "MOV", "XCHG_NOP": "XCHG", 
                     "MOV_CR2G": "MOV", "MOV_DR2G": "MOV", 
                     "MOV_G2CR": "MOV", "MOV_G2DR": "MOV"}.get(desc.mnemonic, desc.mnemonic)
        mnem_name = mnem_name.replace("EVX_", "V")

        opsizes, vecsizes = {0}, {0}
        prepend_opsize, prepend_vecsize = False, False
        # Where to put the operand size in the mnemonic
        separate_opsize = "ENC_SEPSZ" in desc.flags

        if "ENC_NOSZ" in desc.flags or not desc.dynsizes():
            pass
        elif OpKind.SZ_OP in desc.dynsizes():
            if opcode.rexw is not None:
                raise Exception(f"unexpected REXW specifier {desc}")
            opsizes = {8} if "SZ8" in desc.flags else {16, 32, 64}
            if opcode.prefix in ("NP", "66", "F2", "F3") and "U66" not in desc.flags:
                opsizes -= {16}
            if "I66" in desc.flags:
                opsizes -= {16}
            if "D64" in desc.flags:
                opsizes -= {32}
            prepend_opsize = not separate_opsize
            if "F64" in desc.flags:
                opsizes = {64}
                prepend_opsize = False
        elif opcode.vex and opcode.vexl != "IG": # vectors; don't care for SSE
            vecsizes = {128, 256, 512} if opcode.vex == 2 else {128, 256}
            if opcode.vexl:
                vecsizes = {128 << int(c) for c in opcode.vexl}
            prepend_vecsize = not separate_opsize

        # All encoding types; reg is r/k (mask); modrm is r/m/b (broadcast)
        optypes_base = []
        for i, opkind in enumerate(desc.operands):
            reg = "k" if opkind.kind == "MASK" else "r"
            opname = ENCODING_OPORDER[desc.encoding][i]
            if opname == "modrm":
                modrm_type = (opcode.modrm[0] or "rm").replace("r", reg)
                if opcode.extended or desc.mnemonic in ("MOV_CR2G", "MOV_DR2G", "MOV_G2CR", "MOV_G2DR"):
                    modrm_type = reg
                if "BCST" in desc.flags:
                    modrm_type += "b"
                optypes_base.append(modrm_type)
            elif opname == "modreg" or opname == "vexreg":
                optypes_base.append(reg)
            else:
                optypes_base.append(" iariioo"[ENCODINGS[desc.encoding].imm_control])
        optypes = ["".join(x) for x in product(*optypes_base)]

        prefixes = [("", "")]
        if "LOCK" in desc.flags:
            prefixes.append(("LOCK_", "LOCK"))
        if "ENC_REP" in desc.flags:
            prefixes.append(("REP_", "F3"))
        if "ENC_REPCC" in desc.flags:
            prefixes.append(("REPNZ_", "F2"))
            prefixes.append(("REPZ_", "F3"))

        evexmasks = [0]
        if "MASK" in desc.flags:
            if "VSIB" in desc.flags:
                evexmasks = [1]
            else:
                evexmasks.append(1)
                if desc.operands[0].kind != "MASK":
                    evexmasks.append(2) # maskz only for non-mask destinations
        evexsaes = [0]
        if "SAE" in desc.flags:
            evexsaes.append(1)
        elif "ER" in desc.flags:
            evexsaes.append(2)

        keys = (opsizes, vecsizes, prefixes, optypes, evexmasks, evexsaes)
        for opsize, vecsize, prefix, ots, evexmask, evexsae in product(*keys):
            has_memory = "m" in ots or "b" in ots
            if prefix[1] == "LOCK" and ots[0] != "m":
                continue
            if evexmask == 2 and ots[0] != "r":
                continue # EVEX.z must be zero for memory destination
            if evexsae and (vecsize not in (0, 512) or has_memory):
                continue # SAE/ER only works with 512 bit width and no memory

            spec_opcode = opcode
            if prefix[1]:
                spec_opcode = spec_opcode._replace(prefix=prefix[1])
            if opsize == 64 and "D64" not in desc.flags and "F64" not in desc.flags:
                spec_opcode = spec_opcode._replace(rexw="1")
            if vecsize == 512:
                spec_opcode = spec_opcode._replace(vexl="2")
            if vecsize == 256:
                spec_opcode = spec_opcode._replace(vexl="1")
            if vecsize == 128:
                spec_opcode = spec_opcode._replace(vexl="0")
            if spec_opcode.vexl == "IG":
                spec_opcode = spec_opcode._replace(vexl="0")
            if ENCODINGS[desc.encoding].modrm_idx:
                modrm = ("m" if has_memory else "r",) + spec_opcode.modrm[1:]
                spec_opcode = spec_opcode._replace(modrm=modrm)
            if ENCODINGS[desc.encoding].modrm or None not in opcode.modrm:
                assert spec_opcode.modrm[0] in ("r", "m")

            evexbcst = "b" in ots
            evexdisp8scale = 0
            if spec_opcode.vex == 2 and has_memory:
                if not evexbcst:
                    op = desc.operands[ENCODINGS[desc.encoding].modrm_idx^3]
                    size = op.abssize(opsize//8, vecsize//8)
                    evexdisp8scale = size.bit_length() - 1
                elif "BCST16" in desc.flags:
                    evexdisp8scale = 1
                else:
                    evexdisp8scale = 2 if spec_opcode.rexw != "1" else 3

            # Construct mnemonic name
            name = prefix[0] + mnem_name

            # Transform MOV_G2X/X2G into MOVD/MOVQ_G2X/X2G. This isn't done for
            # VEX for historical reasons and there's no reason to break
            # backwards compatibility. This enables EVEX->VEX fallback.
            if desc.mnemonic in ("EVX_MOV_G2X", "EVX_MOV_X2G"):
                name = name[:-4] + "DQ"[opsize == 64] + name[-4:]
                prepend_opsize, opsize = False, 0
            # For VMOVD with memory operand, there's no need to be explicit
            # about G2X/X2G, as there's no alternative. For VMOVQ, another
            # opcode exists, so keep G2X/X2G there for distinguishing.
            if name in ("VMOVD_G2X", "VMOVD_X2G") and has_memory:
                name = name.replace("_G2X", "").replace("_X2G", "")
            # PEXTR/PBROADCAST/PINSR are stored without size suffix in the table
            # to avoid having different tables for 32/64 bit mode due to EVEX.W
            # being ignored in 32-bit mode. Add suffix here.
            if desc.mnemonic == "EVX_PEXTR":
                name += " BW D   Q"[desc.operands[0].abssize(opsize//8, vecsize//8)]
                prepend_opsize, opsize = False, 0
            if desc.mnemonic == "EVX_PBROADCAST":
                name += " BW D   Q"[desc.operands[1].abssize(opsize//8, vecsize//8)]
                name += "_GP"
                prepend_opsize, opsize = False, 0
            if desc.mnemonic == "EVX_PINSR":
                name += " BW D   Q"[desc.operands[2].abssize(opsize//8, vecsize//8)]
                prepend_opsize, opsize = False, 0

            if prepend_opsize and not ("D64" in desc.flags and opsize == 64):
                name += f"_{opsize}"[name[-1] not in "0123456789":]
            if prepend_vecsize:
                name += f"_{vecsize}"[name[-1] not in "0123456789":]
            for ot, op in zip(ots, desc.operands):
                name += ot.replace("o", "")
                if separate_opsize:
                    name += f"{op.abssize(opsize//8, vecsize//8)*8}"
            if "VSIB" not in desc.flags:
                # VSIB implies non-zero mask register, so suffix is not required
                name += ["", "_mask", "_maskz"][evexmask]
            name += ["", "_sae", "_er"][evexsae]
            variant = EncodeVariant(spec_opcode, desc, evexbcst, evexmask, evexsae, evexdisp8scale)
            mnemonics[name, opsize, ots].append(variant)
            altname = {
                "C_EX16": "CBW", "C_EX32": "CWDE", "C_EX64": "CDQE",
                "C_SEP16": "CWD", "C_SEP32": "CDQ", "C_SEP64": "CQO",
                "CMPXCHGD32m": "CMPXCHG8Bm", "CMPXCHGD64m": "CMPXCHG16Bm",
            }.get(name)
            if altname:
                mnemonics[altname, opsize, ots].append(variant)
            if "ENC_CC_BEGIN" in desc.flags:
                # Replace last "O" with "cc"
                ccname = "cc".join(name.rsplit("O", 1))
                ccvariant = variant._replace(flexcc=True)
                mnemonics[ccname, opsize, ots].append(ccvariant)

    for (mnem, opsize, ots), all_variants in mnemonics.items():
        dedup = OrderedDict()
        for i, variant in enumerate(all_variants):
            PRIO = ["O", "OA", "AO", "AM", "MA", "IA", "OI"]
            enc_prio = PRIO.index(variant.desc.encoding) if variant.desc.encoding in PRIO else len(PRIO)
            unique = 0 if variant.desc.encoding != "S" else i
            # Prefer VEX over EVEX for shorter encoding
            key = variant.desc.imm_size(opsize//8), variant.opcode.vex, enc_prio, unique
            if key not in dedup:
                dedup[key] = variant
        variants = [dedup[k] for k in sorted(dedup.keys())]
        if len(variants) > 1 and any(v.opcode.vex for v in variants):
            # Case 1: VEX -> EVEX promotion (AVX-512, APX)
            # Case 2: legacy -> EVEX promotion (APX)
            # In any case, there should be exactly one EVEX opcode.
            if len(variants) != 2:
                raise Exception(f"VEX/EVEX mnemonic with more than two encodings {mnem} {opcode}")
            if variants[0].opcode.vex == 2 or variants[1].opcode.vex != 2:
                raise Exception(f"EVEX mnemonic not with non-EVEX pair {mnem} {opcode} {variants}")
            no_evex, evex = variants[0], variants[1]

            # Make sure that for promotions, only minor things vary.
            # REX.W is special, EVEX might mandate W1 while VEX mandates W0/WIG.
            # Technically ok: IG -> IG/IG -> 0/0 -> IG/0 -> 0/1 -> IG/1 -> 1
            # rexwdowngrade = (no_evex.opcode.rexw is None or
            #                  no_evex.opcode.rexw == evex.opcode.rexw)
            #
            # However, other encoders always use W0 in case of WIG for VEX, and
            # that's probably most beneficial... so:
            # Possible downgrades: IG -> IG/IG -> 0/0 -> IG/0 -> 0/1 -> 1
            # This affects quite a few instructions, so we use an extra bit to
            # flip EVEX.W to VEX.W.

            if (no_evex.opcode.prefix != evex.opcode.prefix or
                no_evex.opcode.escape != evex.opcode.escape or
                no_evex.opcode.opc != evex.opcode.opc or
                # reg/mem doesn't matter, it's already fixed in the mnemonic
                no_evex.opcode.modrm[1:] != evex.opcode.modrm[1:] or
                no_evex.opcode.vexl != evex.opcode.vexl or
                # we don't check rexw_flip here, we can always handle it
                no_evex.desc.encoding != evex.desc.encoding or
                no_evex.desc.operands != evex.desc.operands):
                print(mnem, no_evex)
                print(mnem, evex)
                # Should not happen.
                raise Exception("cannot downgrade EVEX?")
            else:
                rexw_flip = (no_evex.opcode.rexw == "1") != (evex.opcode.rexw == "1")
                variants = [evex._replace(downgrade=1 if not rexw_flip else 2)]
        mnemonics[mnem, opsize, ots] = variants

    return dict(mnemonics)

def _operand_rust_type(ot_char, op_kind, opsize, vecsize):
    """Map an operand type character + OpKind + sizes to a concrete Rust type name.
    
    Returns None if the mapping is ambiguous or cannot be determined.
    """
    if ot_char in ('m', 'b'):
        return 'Mem'
    if ot_char == 'i':
        return 'Imm'
    if ot_char == 'o':
        # Label/address/symbol operand — returns multiple types
        return ['Imm', 'Sym', 'Label']
    if ot_char == 'a':
        return 'AbsoluteAddress'
    if ot_char in ('r', 'k'):
        kind = op_kind.kind if hasattr(op_kind, 'kind') else op_kind
        if kind == 'GP':
            sz = op_kind.abssize(opsize // 8) if hasattr(op_kind, 'abssize') and opsize else 0
            return {1: 'GpbLo', 2: 'Gpw', 4: 'Gpd', 8: 'Gpq'}.get(sz, 'Gpd')
        elif kind == 'XMM':
            sz = 0
            if hasattr(op_kind, 'abssize'):
                try:
                    sz = op_kind.abssize(opsize // 8 if opsize else None,
                                        vecsize // 8 if vecsize else None)
                except Exception:
                    # Some legacy SSE forms have fixed vector width without
                    # explicit opsize/vecsize in this generation phase.
                    sz = 0

            if sz == 0 and vecsize:
                sz = vecsize // 8

            if sz == 0 and hasattr(op_kind, 'sizestr'):
                # Fallback for fixed-width vector forms when size context is
                # not carried through encode3 (mostly legacy SSE forms).
                fallback = {
                    'x': 16, 'ps': 16, 'pd': 16, 'dq': 16,
                    'ss': 16, 'sd': 16, 'q': 16, 'h': 16,
                    'f': 16, 'e': 16, 'qq': 32, 'oq': 64,
                    'y': 16,
                }
                sz = fallback.get(op_kind.sizestr, 0)

            if sz == 0:
                return None
            if sz <= 16:
                return 'Xmm'
            elif sz <= 32:
                return 'Ymm'
            else:
                return 'Zmm'
        elif kind == 'MMX':
            return 'Mm'
        elif kind == 'MASK':
            return 'KReg'
        elif kind == 'CR':
            return 'CReg'
        elif kind == 'DR':
            return 'DReg'
        elif kind == 'FPU':
            return 'St'
        elif kind == 'SEG':
            return 'SReg'
        elif kind == 'BND':
            return 'Bnd'
        elif kind == 'TMM':
            return 'Tmm'
        elif kind == 'MEM':
            return 'Mem'
        elif kind == 'IMM':
            return 'Imm'
        else:
            print(f"Unknown register kind: {kind} for operand with ot_char '{ot_char}'")
            return None
    print(f"Unknown operand type character: '{ot_char}'")
    return None


def _trait_name_for_mnemonic(base_mnem):
    """Convert a base mnemonic name to a PascalCase trait-friendly name.
    e.g. 'MOV' -> 'Mov', 'VADDPS' -> 'Vaddps', 'C_EX' -> 'CEx'
    """
    # Handle leading underscores or digits
    prefix = ''
    name = base_mnem
    if name.startswith('_'):
        prefix = '_'
        name = name[1:]

    parts = name.split('_')
    result = ''
    for part in parts:
        if part:
            result += part[0].upper() + part[1:].lower()
    
    full = prefix + result
    # Ensure the result is a valid Rust identifier (cannot start with digit)
    if full and full[0].isdigit():
        full = '_' + full
    return full


def _compute_base_mnemonic(mnem, ots, opsize, desc, has_separate_opsize):
    """Compute the base mnemonic name by stripping operand type suffixes and size suffixes.
    
    This groups variants like MOV64rr, MOV64rm, MOV32rr etc. under 'MOV64' or just 'MOV'.
    """
    # Strip encoding suffixes (_mask, _maskz, _sae, _er)
    extra_suffix = ""
    clean = mnem
    for s in ("_sae", "_er"):
        if clean.endswith(s):
            extra_suffix = s + extra_suffix
            clean = clean[:-len(s)]
            break
    for s in ("_maskz", "_mask"):
        if clean.endswith(s):
            extra_suffix = s + extra_suffix
            clean = clean[:-len(s)]
            break

    # Strip the ots suffix
    ots_suffix = ots.replace("o", "")
    if ots_suffix and not has_separate_opsize and clean.endswith(ots_suffix):
        clean = clean[:-len(ots_suffix)]

    return clean + extra_suffix


def encode_mnems3(entries):
    # mapping from (base_mnem, mnem, opsize, ots) -> variants
    mnemonics = defaultdict(list)
    mnemonics["PAUSE", "PAUSE", 0, ""] = [EncodeVariant(Opcode.parse("F3.90"), InstrDesc.parse("NP - - - - NOP"))]

    for weak, opcode, desc in entries:
        if "I64" in desc.flags or desc.mnemonic[:9] == "RESERVED_":
            continue

        mnem_name = {"MOVABS": "MOV", "XCHG_NOP": "XCHG"}.get(desc.mnemonic, desc.mnemonic)
        mnem_name = mnem_name.replace("EVX_", "V")

        opsizes, vecsizes = {0}, {0}
        prepend_opsize, prepend_vecsize = False, False
        separate_opsize = "ENC_SEPSZ" in desc.flags

        if "ENC_NOSZ" in desc.flags or not desc.dynsizes():
            pass
        elif OpKind.SZ_OP in desc.dynsizes():
            if opcode.rexw is not None:
                raise Exception(f"unexpected REXW specifier {desc}")
            opsizes = {8} if "SZ8" in desc.flags else {16, 32, 64}
            if opcode.prefix in ("NP", "66", "F2", "F3") and "U66" not in desc.flags:
                opsizes -= {16}
            if "I66" in desc.flags:
                opsizes -= {16}
            if "D64" in desc.flags:
                opsizes -= {32}
            prepend_opsize = not separate_opsize
            if "F64" in desc.flags:
                opsizes = {64}
                prepend_opsize = False
        elif opcode.vex and opcode.vexl != "IG":
            vecsizes = {128, 256, 512} if opcode.vex == 2 else {128, 256}
            if opcode.vexl:
                vecsizes = {128 << int(c) for c in opcode.vexl}
            prepend_vecsize = not separate_opsize

        optypes_base = []
        for i, opkind in enumerate(desc.operands):
            reg = "k" if opkind.kind == "MASK" else "r"
            opname = ENCODING_OPORDER[desc.encoding][i]
            if opname == "modrm":
                modrm_type = (opcode.modrm[0] or "rm").replace("r", reg)
                if opcode.extended or desc.mnemonic in ("MOV_CR2G", "MOV_DR2G", "MOV_G2CR", "MOV_G2DR"):
                    modrm_type = reg
                if "BCST" in desc.flags:
                    modrm_type += "b"
                optypes_base.append(modrm_type)
            elif opname == "modreg" or opname == "vexreg":
                optypes_base.append(reg)
            else:
                optypes_base.append(" iariioo"[ENCODINGS[desc.encoding].imm_control])
        optypes = ["".join(x) for x in product(*optypes_base)]

        prefixes = [("", "")]
        if "LOCK" in desc.flags:
            prefixes.append(("LOCK_", "LOCK"))
        if "ENC_REP" in desc.flags:
            prefixes.append(("REP_", "F3"))
        if "ENC_REPCC" in desc.flags:
            prefixes.append(("REPNZ_", "F2"))
            prefixes.append(("REPZ_", "F3"))

        evexmasks = [0]
        if "MASK" in desc.flags:
            if "VSIB" in desc.flags:
                evexmasks = [1]
            else:
                evexmasks.append(1)
                if desc.operands[0].kind != "MASK":
                    evexmasks.append(2)
        evexsaes = [0]
        if "SAE" in desc.flags:
            evexsaes.append(1)
        elif "ER" in desc.flags:
            evexsaes.append(2)

        keys = (opsizes, vecsizes, prefixes, optypes, evexmasks, evexsaes)
        for opsize, vecsize, prefix, ots, evexmask, evexsae in product(*keys):
            has_memory = "m" in ots or "b" in ots
            if prefix[1] == "LOCK" and ots[0] != "m":
                continue
            if evexmask == 2 and ots[0] != "r":
                continue
            if evexsae and (vecsize not in (0, 512) or has_memory):
                continue

            spec_opcode = opcode
            if prefix[1]:
                spec_opcode = spec_opcode._replace(prefix=prefix[1])
            if opsize == 64 and "D64" not in desc.flags and "F64" not in desc.flags:
                spec_opcode = spec_opcode._replace(rexw="1")
            if vecsize == 512:
                spec_opcode = spec_opcode._replace(vexl="2")
            if vecsize == 256:
                spec_opcode = spec_opcode._replace(vexl="1")
            if vecsize == 128:
                spec_opcode = spec_opcode._replace(vexl="0")
            if spec_opcode.vexl == "IG":
                spec_opcode = spec_opcode._replace(vexl="0")
            if ENCODINGS[desc.encoding].modrm_idx:
                modrm = ("m" if has_memory else "r",) + spec_opcode.modrm[1:]
                spec_opcode = spec_opcode._replace(modrm=modrm)
            if ENCODINGS[desc.encoding].modrm or None not in opcode.modrm:
                assert spec_opcode.modrm[0] in ("r", "m")

            evexbcst = "b" in ots
            evexdisp8scale = 0
            if spec_opcode.vex == 2 and has_memory:
                if not evexbcst:
                    op = desc.operands[ENCODINGS[desc.encoding].modrm_idx ^ 3]
                    size = op.abssize(opsize // 8, vecsize // 8)
                    evexdisp8scale = size.bit_length() - 1
                elif "BCST16" in desc.flags:
                    evexdisp8scale = 1
                else:
                    evexdisp8scale = 2 if spec_opcode.rexw != "1" else 3

            name = prefix[0] + mnem_name
            local_prepend_opsize = prepend_opsize
            local_opsize = opsize

            if desc.mnemonic in ("EVX_MOV_G2X", "EVX_MOV_X2G"):
                name = name[:-4] + "DQ"[local_opsize == 64] + name[-4:]
                local_prepend_opsize, local_opsize = False, 0
            if name in ("VMOVD_G2X", "VMOVD_X2G") and has_memory:
                name = name.replace("_G2X", "").replace("_X2G", "")
            if desc.mnemonic == "EVX_PEXTR":
                name += " BW D   Q"[desc.operands[0].abssize(local_opsize // 8, vecsize // 8)]
                local_prepend_opsize, local_opsize = False, 0
            if desc.mnemonic == "EVX_PBROADCAST":
                name += " BW D   Q"[desc.operands[1].abssize(local_opsize // 8, vecsize // 8)]
                name += "_GP"
                local_prepend_opsize, local_opsize = False, 0
            if desc.mnemonic == "EVX_PINSR":
                name += " BW D   Q"[desc.operands[2].abssize(local_opsize // 8, vecsize // 8)]
                local_prepend_opsize, local_opsize = False, 0

            base_name = name
            full_name = name

            if local_prepend_opsize and not ("D64" in desc.flags and local_opsize == 64):
                full_name += f"_{local_opsize}"[full_name[-1] not in "0123456789":]
            if prepend_vecsize:
                full_name += f"_{vecsize}"[full_name[-1] not in "0123456789":]

            for ot, op in zip(ots, desc.operands):
                full_name += ot.replace("o", "")
                if separate_opsize:
                    full_name += f"{op.abssize(local_opsize // 8, vecsize // 8) * 8}"

            if "VSIB" not in desc.flags:
                mask_suffix = ["", "_mask", "_maskz"][evexmask]
                base_name += mask_suffix
                full_name += mask_suffix
            sae_suffix = ["", "_sae", "_er"][evexsae]
            base_name += sae_suffix
            full_name += sae_suffix

            variant = EncodeVariant(spec_opcode, desc, evexbcst, evexmask, evexsae, evexdisp8scale)
            mnemonics[base_name, full_name, opsize, ots].append(variant)

            altname = {
                "C_EX16": "CBW", "C_EX32": "CWDE", "C_EX64": "CDQE",
                "C_SEP16": "CWD", "C_SEP32": "CDQ", "C_SEP64": "CQO",
                "CMPXCHGD32m": "CMPXCHG8Bm", "CMPXCHGD64m": "CMPXCHG16Bm",
            }.get(full_name)
            if altname:
                mnemonics[altname, altname, opsize, ots].append(variant)

            if "ENC_CC_BEGIN" in desc.flags:
                ccname = "cc".join(full_name.rsplit("O", 1))
                ccbase = "cc".join(base_name.rsplit("O", 1))
                ccvariant = variant._replace(flexcc=True)
                mnemonics[ccbase, ccname, opsize, ots].append(ccvariant)

    for (base_mnem, mnem, opsize, ots), all_variants in mnemonics.items():
        dedup = OrderedDict()
        for i, variant in enumerate(all_variants):
            PRIO = ["O", "OA", "AO", "AM", "MA", "IA", "OI"]
            enc_prio = PRIO.index(variant.desc.encoding) if variant.desc.encoding in PRIO else len(PRIO)
            unique = 0 if variant.desc.encoding != "S" else i
            key = variant.desc.imm_size(opsize // 8), variant.opcode.vex, enc_prio, unique
            if key not in dedup:
                dedup[key] = variant
        variants = [dedup[k] for k in sorted(dedup.keys())]
        if len(variants) > 1 and any(v.opcode.vex for v in variants):
            if len(variants) != 2:
                raise Exception(f"VEX/EVEX mnemonic with more than two encodings {mnem} {opcode}")
            if variants[0].opcode.vex == 2 or variants[1].opcode.vex != 2:
                raise Exception(f"EVEX mnemonic not with non-EVEX pair {mnem} {opcode} {variants}")
            no_evex, evex = variants[0], variants[1]
            if (no_evex.opcode.prefix != evex.opcode.prefix or
                no_evex.opcode.escape != evex.opcode.escape or
                no_evex.opcode.opc != evex.opcode.opc or
                no_evex.opcode.modrm[1:] != evex.opcode.modrm[1:] or
                no_evex.opcode.vexl != evex.opcode.vexl or
                no_evex.desc.encoding != evex.desc.encoding or
                no_evex.desc.operands != evex.desc.operands):
                print(mnem, no_evex)
                print(mnem, evex)
                raise Exception("cannot downgrade EVEX?")
            else:
                rexw_flip = (no_evex.opcode.rexw == "1") != (evex.opcode.rexw == "1")
                variants = [evex._replace(downgrade=1 if not rexw_flip else 2)]
        mnemonics[base_mnem, mnem, opsize, ots] = variants

    return dict(mnemonics)


def encode3(entries, args):
    """Emit one trait per opcode with generic type parameters for operand types,
    and one impl per concrete operand type combination.
    
    Example output:
        pub trait MovEmitter<A, B> { fn mov(&mut self, op0: A, op1: B); }
        impl MovEmitter<Gpq, Gpq> for Assembler<'_> {
            fn mov(&mut self, op0: Gpq, op1: Gpq) {
                self.emit(MOV64RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
            }
        }
    """
    mnemonics = encode_mnems3(entries)
    mnemonics["NOP", "NOP", 0, ""] = [EncodeVariant(Opcode.parse("90"), InstrDesc.parse("NP - - - - NOP"))]
    opcode_docs = load_opcode_docs(args.docs_inputfolder)

    get_feature = get_instruction_feature_set(entries)
    trait_feature = {}  # (base_mnem, nargs) -> feature name

    mnem_map = {}
    alt_table = [0]

    # Phase 1: Build opcode constants and collect trait data
    # trait_data: base_mnem -> list of (rust_types_tuple, opc_name, nargs, doc_info)
    trait_data = defaultdict(list)

    for (base_mnem, mnem, opsize, ots), variants in mnemonics.items():
        if "LOCK_" in mnem or "REP_" in mnem or "REPNZ_" in mnem or "REPZ_" in mnem:
            continue

        desc = variants[0].desc
        nargs = len(desc.operands)

        # Track feature for this (base_mnem, nargs) — first encountered wins
        feature_key = (base_mnem, nargs)
        if feature_key not in trait_feature:
            feature_flags = {flag[2:] for flag in desc.flags if flag.startswith("F=")}
            trait_feature[feature_key] = get_feature(feature_flags) if feature_flags else "BASE"

        supports_high_regs = []
        if desc.mnemonic in ("MOVSX", "MOVZX") or opsize == 8:
            for i, (ot, op) in enumerate(zip(ots, desc.operands)):
                if ot == "r" and op.kind == "GP" and op.abssize(opsize // 8) == 1:
                    supports_high_regs.append(i)

        alt_indices = [i + len(alt_table) for i in range(len(variants) - 1)] + [0]

        if desc.encoding == "D":
            if len(variants) > 1:
                variants = variants[:1]
                alt_indices = [0xff]

        enc_opcs = []
        for alt, variant in zip(alt_indices, variants):
            opcode, vdesc = variant.opcode, variant.desc
            encoding = ENCODINGS[vdesc.encoding]
            opc_i = opcode.opc
            if None not in opcode.modrm:
                opc_i |= 0xc000 | opcode.modrm[1] << 11 | opcode.modrm[2] << 8
            elif opcode.modrm[1] is not None:
                opc_i |= opcode.modrm[1] << 8
            if opcode.modrm == ("m", None, 4):
                opc_i |= 0x2000000000
            if not opcode.vex:
                assert opcode.escape < 4
                opc_i |= opcode.escape * 0x10000
                opc_i |= 0x80000 if opcode.prefix == "66" or opsize == 16 else 0
                opc_i |= 0x100000 if opcode.prefix == "F2" else 0
                opc_i |= 0x200000 if opcode.prefix == "F3" else 0
            else:
                assert opcode.escape < 8
                opc_i |= opcode.escape * 0x10000
                if opcode.prefix == "66" or opsize == 16:
                    assert opcode.prefix not in ("F2", "F3")
                    opc_i |= 0x100000
                if opcode.prefix == "F3":
                    opc_i |= 0x200000
                elif opcode.prefix == "F2":
                    opc_i |= 0x300000
            opc_i |= 0x400000 if opcode.rexw == "1" else 0
            if opcode.prefix == "LOCK":
                opc_i |= 0x800000
            elif opcode.vex == 1:
                opc_i |= 0x1000000 + 0x800000 * int(opcode.vexl or 0)
            elif opcode.vex == 2:
                opc_i |= 0x2000000
                if not variant.evexsae:
                    opc_i |= 0x800000 * int(opcode.vexl or 0)
            assert not (variant.evexsae and variant.evexbcst)
            opc_i |= 0x4000000 if variant.evexsae or variant.evexbcst else 0
            opc_i |= 0x8000000 if "VSIB" in vdesc.flags else 0
            opc_i |= 0x1000000000 if variant.evexmask == 2 else 0
            opc_i |= 0x4000000000 if variant.downgrade in (1, 2) else 0
            opc_i |= 0x40000000000 if variant.downgrade == 2 else 0
            opc_i |= 0x8000000000 * variant.evexdisp8scale
            if alt >= 0x100:
                raise Exception("encode alternate bits exhausted")
            opc_i |= sum(1 << i for i in supports_high_regs) << 45
            if encoding.imm_control >= 3:
                opc_i |= vdesc.imm_size(opsize // 8) << 47
            elif encoding.imm_control in (1, 2):
                opc_i |= 1 << 47

            enc_encoding = vdesc.encoding
            if vdesc.encoding != "I" and vdesc.encoding.endswith("I"):
                enc_encoding = vdesc.encoding[:-1]
            elif vdesc.encoding == "IA":
                enc_encoding = "A"
            opc_i |= ["NP", "M", "R", "M1", "MC", "MR", "RM", "RMA", "MRC",
                "AM", "MA", "I", "O", "OA", "S", "A", "D", "FD", "TD", "IM",
                "RVM", "RVMR", "RMV", "VM", "MVR", "MRV",
            ].index(enc_encoding) << 51
            opc_i |= alt << 56
            enc_opcs.append(opc_i)

        opc_name = mnem.upper()
        if opc_name.startswith('3'):
            opc_name = '_' + opc_name
        mnem_map[opc_name] = enc_opcs[0]
        alt_table += enc_opcs[1:]

        # Determine Rust types for each operand
        vecsize = 0
        # Re-derive vecsize from the variant
        v0 = variants[0]
        if v0.opcode.vex and v0.opcode.vexl:
            try:
                vecsize = 128 << int(v0.opcode.vexl)
            except ValueError:
                vecsize = 0

        rust_type_options = []  # list of lists: each position has possible types
        can_map = True
        for i in range(nargs):
            ot_char = ots[i] if i < len(ots) else ''
            op_kind = desc.operands[i] if i < len(desc.operands) else None
            if op_kind is None or not ot_char:
                can_map = False
                break
            try:
                rtype = _operand_rust_type(ot_char, op_kind, opsize, vecsize)
            except Exception:
                rtype = None
            if rtype is None:
                can_map = False
                break
            # Normalize to list
            if isinstance(rtype, list):
                rust_type_options.append(rtype)
            else:
                rust_type_options.append([rtype])

        if not can_map:
            print(f"Warning: cannot determine Rust types for {mnem} with ots={ots}, opsize={opsize}, desc={desc}")
            continue

        canonical_mnemonic = canonicalize_doc_mnemonic(desc.mnemonic)
        doc = opcode_docs.get(canonical_mnemonic)
        
        # Generate cartesian product of all type combinations
        for combo in product(*rust_type_options):
            rust_types_tuple = tuple(combo)
            trait_data[(base_mnem, nargs)].append({
                'rust_types': rust_types_tuple,
                'opc_name': opc_name,
                'nargs': nargs,
                'canonical_mnemonic': canonical_mnemonic,
                'doc': doc,
                'mnem': mnem,
            })

    # Phase 2: Build traits, impls, and inherent forwarding methods
    # Group by (base_mnem, nargs). Each group gets one trait.
    # Deduplicate: same rust_types keeps one entry.
    feature_code = defaultdict(list)  # feature_name -> [trait+impl code strings]
    feature_forwarders = defaultdict(list)  # feature_name -> [forwarder strings]

    # Track trait names to disambiguate when same base_mnem has different nargs
    base_mnem_nargs = defaultdict(set)
    for (base_mnem, nargs) in trait_data:
        base_mnem_nargs[base_mnem].add(nargs)


    for (base_mnem, nargs) in sorted(trait_data.keys()):
        entries_list = trait_data[(base_mnem, nargs)]
        if not entries_list:
            continue

        trait_base = _trait_name_for_mnemonic(base_mnem)
        fn_name = base_mnem.lower()
        # Ensure fn_name is a valid Rust identifier
        if fn_name and fn_name[0].isdigit():
            fn_name = '_' + fn_name
        # Sanitize fn_name for Rust keywords
        if fn_name in ('loop', 'in', 'out', 'move', 'mod', 'type', 'match', 'ref',
                        'return', 'break', 'continue', 'if', 'else', 'while', 'for',
                        'fn', 'let', 'mut', 'use', 'pub', 'self', 'super', 'crate',
                        'as', 'const', 'extern', 'impl', 'struct', 'enum', 'trait',
                        'where', 'async', 'await', 'dyn', 'static', 'union',
                        'unsafe', 'abstract', 'become', 'box', 'do', 'final',
                        'macro', 'override', 'priv', 'typeof', 'unsized',
                        'virtual', 'yield', 'try'):
            fn_name = f'r#{fn_name}'

        # Get doc from first entry
        first_entry = entries_list[0]
        canonical_mnemonic = first_entry['canonical_mnemonic']
        doc = first_entry['doc']

        # Disambiguate fn_name if same base_mnem has multiple nargs
        all_nargs = base_mnem_nargs[base_mnem]
        if len(all_nargs) > 1:
            if nargs == 0:
                fn_name_suffix = ''
            else: 
                fn_name_suffix = f'_{nargs}'
        else:
            fn_name_suffix = ''
        full_fn_name = fn_name + fn_name_suffix

        # Collect unique type combinations
        seen_types = {}
        unique_entries = []
        for entry in entries_list:
            key = entry['rust_types']
            if key not in seen_types:
                seen_types[key] = entry
                unique_entries.append(entry)

        if not unique_entries:
            continue

        # Generate generic type parameter names
        type_params = [chr(ord('A') + i) for i in range(nargs)]
        type_params_str = ', '.join(type_params)

        # Build trait definition
        trait_name = f"{trait_base}Emitter" + (fn_name_suffix.title() if fn_name_suffix else '')
        if nargs > 0:
            trait_decl = f"pub trait {trait_name}<{type_params_str}>"
        else:
            trait_decl = f"pub trait {trait_name}"

        # Build fn signature for trait
        args_str = ', '.join(f'op{i}: {type_params[i]}' for i in range(nargs))
        if nargs > 0:
            fn_sig = f"fn {full_fn_name}(&mut self, {args_str})"
        else:
            fn_sig = f"fn {full_fn_name}(&mut self)"

        # Build a readable list of concrete operand variants.
        variants_doc = []
        for entry in unique_entries:
            rust_types = entry['rust_types']
            if rust_types:
                sig = f"{full_fn_name}({', '.join(rust_types)})"
            else:
                sig = f"{full_fn_name}()"
            variants_doc.append(sig)
        variants_doc = sorted(set(variants_doc))

        variants_operands = []
        for entry in unique_entries:
            rust_types = entry['rust_types']
            variants_operands.append(', '.join(rust_types) if rust_types else '(none)')
        variants_operands = sorted(set(variants_operands))

        idx_width = max(1, len(str(len(variants_operands))))
        ops_width = max(len("Operands"), *(len(v) for v in variants_operands))
        sep = f"+-{'-' * idx_width}-+-{'-' * ops_width}-+"
        header = f"| {'#'.ljust(idx_width)} | {'Operands'.ljust(ops_width)} |"
        table_lines = [sep, header, sep]
        for i, ops in enumerate(variants_operands, start=1):
            table_lines.append(f"| {str(i).ljust(idx_width)} | {ops.ljust(ops_width)} |")
        table_lines.append(sep)

        # Doc for trait
        trait_doc = ''
        if doc:
            trait_doc = f"/// `{base_mnem.upper()}` ({canonical_mnemonic}). {doc['tooltip']}\n"
        else:
            trait_doc = f"/// `{base_mnem.upper()}`.\n"
        trait_doc += "///\n"
        trait_doc += "/// Supported operand variants:\n"
        trait_doc += "///\n"
        trait_doc += "/// ```text\n"
        for line in table_lines:
            trait_doc += f"/// {line}\n"
        trait_doc += "/// ```\n"

        trait_code = f"{trait_doc}{trait_decl} {{\n    {fn_sig};\n}}\n"

        # Build impl blocks
        impl_blocks = []
        rem = 4 - nargs
        noreg_args = ', '.join('&NOREG' for _ in range(rem))

        for entry in unique_entries:
            rust_types = entry['rust_types']
            opc_name = entry['opc_name']
               
            if nargs > 0:
                concrete_types_str = ', '.join(rust_types)
                impl_header = f"impl<'a> {trait_name}<{concrete_types_str}> for Assembler<'a>"
            else:
                impl_header = f"impl<'a> {trait_name} for Assembler<'a>"

            concrete_args = ', '.join(f'op{i}: {rust_types[i]}' for i in range(nargs))
            if nargs > 0:
                concrete_fn_sig = f"fn {full_fn_name}(&mut self, {concrete_args})"
            else:
                concrete_fn_sig = f"fn {full_fn_name}(&mut self)"

            op_args = ', '.join(f'op{i}.as_operand()' for i in range(nargs))
            if op_args and noreg_args:
                all_args = f'{op_args}, {noreg_args}'
            elif op_args:
                all_args = op_args
            else:
                all_args = noreg_args

            impl_block = (
                f"{impl_header} {{\n"
                f"    {concrete_fn_sig} {{\n"
                f"        self.emit({opc_name}, {all_args});\n"
                f"    }}\n"
                f"}}\n"
            )
            impl_blocks.append(impl_block)

        feature = trait_feature.get((base_mnem, nargs), "BASE")
        feature_code[feature].append(trait_code + '\n' + '\n'.join(impl_blocks))

        # Build inherent forwarding method so users can call self.mov(...)
        # while still dispatching through trait implementations.
        fwd_doc = trait_doc
        if nargs > 0:
            fwd_type_params = [chr(ord('A') + i) for i in range(nargs)]
            fwd_tp_str = ', '.join(fwd_type_params)
            fwd_args = ', '.join(f'op{i}: {fwd_type_params[i]}' for i in range(nargs))
            fwd_call_args = ', '.join(f'op{i}' for i in range(nargs))
            fwd_where = f"where Assembler<'a>: {trait_name}<{fwd_tp_str}>"
            forwarder = (
                f"{fwd_doc}"
                f"#[inline]\n"
                f"pub fn {full_fn_name}<{fwd_tp_str}>(&mut self, {fwd_args})\n"
                f"{fwd_where} {{\n"
                f"    <Self as {trait_name}<{fwd_tp_str}>>::{full_fn_name}(self, {fwd_call_args});\n"
                f"}}\n"
            )
        else:
            forwarder = (
                f"{fwd_doc}"
                f"#[inline]\n"
                f"pub fn {full_fn_name}(&mut self)\n"
                f"where Assembler<'a>: {trait_name} {{\n"
                f"    <Self as {trait_name}>::{full_fn_name}(self);\n"
                f"}}\n"
            )
        feature_forwarders[feature].append(forwarder)

    # Build opcode constants
    mnem_tab = "".join(
        f"pub const {m}: i64 = {v:#x}u64 as i64;\n"
        for m, v in mnem_map.items()
    )

    alt_tab = f"pub static ALT_TAB: [i64; {len(alt_table)}] = [\n"
    for v in alt_table:
        alt_tab += f"    {v:#x}u64 as i64,\n"
    alt_tab += "];\n"

    public_str = f"{mnem_tab}\n{alt_tab}"

    feature_preamble = (
        "use crate::x86::assembler::*;\n"
        "use crate::x86::operands::*;\n"
        "use super::super::opcodes::*;\n"
        "use crate::core::emitter::*;\n"
        "use crate::core::operand::*;\n"
        "\n"
        "/// A dummy operand that represents no register. Here just for simplicity.\n"
        "const NOREG: Operand = Operand::new();\n"
        "\n"
    )

    per_feature_str = {}
    for feature, code_list in feature_code.items():
        fwds = feature_forwarders.get(feature, [])
        inherent = "impl<'a> Assembler<'a> {\n" + ''.join(f"    {line}\n" if line else "\n" for fwd in fwds for line in fwd.splitlines()) + "}\n"
        per_feature_str[feature] = feature_preamble + '\n'.join(code_list) + "\n\n" + inherent

    # =========================================================================
    # Phase 3: Dynamic dispatch module (_DYN.rs) and text-assembler (_PARSER.rs)
    # =========================================================================
    _RUST_KW = {
        'loop', 'in', 'out', 'move', 'mod', 'type', 'match', 'ref',
        'return', 'break', 'continue', 'if', 'else', 'while', 'for',
        'fn', 'let', 'mut', 'use', 'pub', 'self', 'super', 'crate',
        'as', 'const', 'extern', 'impl', 'struct', 'enum', 'trait',
        'where', 'async', 'await', 'dyn', 'static', 'union',
        'unsafe', 'abstract', 'become', 'box', 'do', 'final',
        'macro', 'override', 'priv', 'typeof', 'unsized',
        'virtual', 'yield', 'try',
    }

    dyn_impl_methods = []      # List[str] — Instruction::xyz() methods
    mnem_dispatch_arms = []    # List[str] — arms for Instruction::from_mnem
    seen_dyn_keys = set()      # (dispatch_key, nargs) pairs already emitted

    for (base_mnem, nargs) in sorted(trait_data.keys()):
        entries_list = trait_data[(base_mnem, nargs)]
        if not entries_list:
            continue

        # Replicate fn_name derivation from Phase 2
        fn_name = base_mnem.lower()
        if fn_name and fn_name[0].isdigit():
            fn_name = '_' + fn_name
        # For dyn builders we allow reserved words with trailing underscore
        # (they live inside impl Instruction, so r# is not needed there)
        if fn_name in _RUST_KW:
            fn_name = fn_name + '_instr'

        all_nargs_d = base_mnem_nargs[base_mnem]
        fn_name_suffix = f'_{nargs}' if len(all_nargs_d) > 1 and nargs != 0 else ''
        full_fn_name = fn_name + fn_name_suffix

        # Collect unique type combinations (same dedupe as Phase 2)
        seen_types_d = {}
        unique_entries = []
        for entry in entries_list:
            key = entry['rust_types']
            if key not in seen_types_d:
                seen_types_d[key] = entry
                unique_entries.append(entry)

        if not unique_entries:
            continue

        # Build method body: runtime checks + Ok(Self{...}) or Err
        body_lines = []
        has_unconditional = False
        for entry in unique_entries:
            rust_types = entry['rust_types']
            opc_name = entry['opc_name']
            checks = []
            for i, t in enumerate(rust_types):
                tmpl = DYN_OPTYPE_CHECK.get(t)
                if tmpl:
                    checks.append(tmpl.replace('{i}', str(i)))

            op_fields = ', '.join(f'op{i}: *op{i}' for i in range(nargs))
            noreg_fields = ', '.join(f'op{i}: NOREG' for i in range(nargs, 4))
            all_parts = [f'opcode: {opc_name}']
            if op_fields:
                all_parts.append(op_fields)
            if noreg_fields:
                all_parts.append(noreg_fields)
            all_fields = ', '.join(all_parts)

            if checks:
                cond = ' && '.join(checks)
                body_lines.append(
                    f'        if {cond} {{\n'
                    f'            return Ok(Self {{ {all_fields} }});\n'
                    f'        }}'
                )
            else:
                body_lines.append(
                    f'        return Ok(Self {{ {all_fields} }});'
                )
                has_unconditional = True
                break

        if not has_unconditional:
            body_lines.append('        Err(AsmError::InvalidOperand)')

        body = '\n'.join(body_lines)

        if nargs > 0:
            params_str = ', '.join(f'op{i}: &Operand' for i in range(nargs))
        else:
            params_str = ''

        doc_base = base_mnem.upper()
        method = (
            f'    /// Construct a dynamically typed `{doc_base}` instruction.\n'
            f'    /// Checks operand types at runtime; returns `Err` if types do not match.\n'
            f'    pub fn {full_fn_name}({params_str}) -> Result<Self, AsmError> {{\n'
            f'{body}\n'
            f'    }}\n'
        )
        dyn_impl_methods.append(method)

        # dispatch key — lowercase mnemonic (without leading r#)
        dispatch_key = base_mnem.lower()
        if dispatch_key and dispatch_key[0].isdigit():
            dispatch_key = '_' + dispatch_key

        if (dispatch_key, nargs) not in seen_dyn_keys:
            seen_dyn_keys.add((dispatch_key, nargs))
            if nargs > 0:
                call_args = ', '.join(f'&ops[{i}]' for i in range(nargs))
                arm_body = (
                    f'if ops.len() == {nargs} {{\n'
                    f'                    Self::{full_fn_name}({call_args})\n'
                    f'                }} else {{\n'
                    f'                    Err(AsmError::InvalidOperand)\n'
                    f'                }}'
                )
            else:
                arm_body = (
                    f'if ops.is_empty() {{\n'
                    f'                    Self::{full_fn_name}()\n'
                    f'                }} else {{\n'
                    f'                    Err(AsmError::InvalidOperand)\n'
                    f'                }}'
                )
            mnem_dispatch_arms.append(
                f'            "{dispatch_key}" => {{\n'
                f'                {arm_body}\n'
                f'            }},\n'
            )

    # ------------------------------------------------------------------
    # Generate _DYN.rs
    # ------------------------------------------------------------------
    dyn_methods_block = '\n'.join(dyn_impl_methods)
    dispatch_block = ''.join(mnem_dispatch_arms)

    dyn_str = (
        '#![allow(unused_imports, dead_code, non_snake_case, clippy::all)]\n'
        'extern crate alloc;\n'
        'use crate::x86::assembler::Assembler;\n'
        'use crate::x86::operands::*;\n'
        'use super::super::opcodes::*;\n'
        'use crate::core::emitter::*;\n'
        'use crate::core::operand::*;\n'
        'use crate::AsmError;\n'
        '\n'
        'const NOREG: Operand = Operand::new();\n'
        '\n'
        '/// A dynamically typed x86 instruction.\n'
        '///\n'
        '/// Build instructions with runtime operand-type checking via the associated\n'
        '/// constructor methods (e.g. [`Instruction::mov`], [`Instruction::add`]),\n'
        '/// or build from a mnemonic string with [`Instruction::from_mnem`].\n'
        'pub struct Instruction {\n'
        '    /// Encoded opcode constant (matches those in `opcodes.rs`).\n'
        '    pub opcode: i64,\n'
        '    pub op0: Operand,\n'
        '    pub op1: Operand,\n'
        '    pub op2: Operand,\n'
        '    pub op3: Operand,\n'
        '}\n'
        '\n'
        'impl Instruction {\n'
        '    /// Emit this instruction into the given assembler.\n'
        '    #[inline]\n'
        '    pub fn emit(&self, asm: &mut Assembler<\'_>) {\n'
        '        asm.emit(self.opcode, &self.op0, &self.op1, &self.op2, &self.op3);\n'
        '    }\n'
        '\n'
        + dyn_methods_block + '\n'
        '\n'
        '    /// Build an instruction from a mnemonic string and a slice of operands,\n'
        '    /// performing runtime operand-type checking.\n'
        '    ///\n'
        '    /// # Errors\n'
        '    /// - [`AsmError::InvalidInstruction`] — unknown mnemonic.\n'
        '    /// - [`AsmError::InvalidOperand`] — operand count or types do not match.\n'
        '    pub fn from_mnem(mnem: &str, ops: &[Operand]) -> Result<Self, AsmError> {\n'
        '        let mnem_l = mnem.to_ascii_lowercase();\n'
        '        match mnem_l.as_str() {\n'
        + dispatch_block
        + '            _ => Err(AsmError::InvalidInstruction),\n'
        '        }\n'
        '    }\n'
        '}\n'
        '\n'
        'impl<\'a> Assembler<\'a> {\n'
        '    /// Emit an instruction identified by mnemonic string and operand slice.\n'
        '    ///\n'
        '    /// This performs runtime operand-type checking to select the correct encoding.\n'
        '    /// Requires the `x86-dyn` Cargo feature.\n'
        '    pub fn emit_dyn(&mut self, mnem: &str, ops: &[Operand]) -> Result<(), AsmError> {\n'
        '        Instruction::from_mnem(mnem, ops)?.emit(self);\n'
        '        Ok(())\n'
        '    }\n'
        '}\n'
    )

    

    return public_str, per_feature_str, dyn_str


def get_instruction_feature_set(entries):
    """Determine the feature set for an instruction based on its variants."""
    # Priority order for feature sets (most specific first)
    FEATURE_PRIORITY = [
        "AMX", "AVX512", "AVX512_BF16", "AVX512_BITALG", "AVX512BW", "AVX512CD",
        "AVX512DQ", "AVX512F", "AVX512_IFMA", "AVX512_VBMI", "AVX512_VBMI2",
        "AVX512_VNNI", "AVX512_VP2INTERSECT", "AVX512_VPOPCNTDQ", "AVX10", "AVX2",
        "AVX", "VNNI", "FMA", "F16C", "GFNI", "VAES", "VPCLMULQDQ",
        "SSE42", "SSE41", "SSSE3", "SSE3", "SSE2", "SSE", "SSE4A",
        "AESNI", "AESKLE", "PCLMULQDQ", "SHA",
        "BMI2", "BMI1", "ADX", "ABM",
        "RDRAND", "RDSEED", "CLFLUSHOPT", "CLWB", "CLDEMOTE",
        "MOVBE", "POPCNT", "LZCNT", "TZCNT",
        "FSGSBASE", "INVPCID", "MPX", "CLZERO", "MCOMMIT",
        "PREFETCH", "PREFETCHW", "PREFETCHWT1", "PREFETCHI",
        "3DNOW", "MMX", "387",
        "HLERTM", "SGX", "SMX", "SEAM", "TDX",
        "CET", "UINTR", "SERIALIZE", "WAITPKG", "PTWRITE",
        "ENQCMD", "MOVDIRI", "MOVDIR64B", "PBNDKB",
        "RAO", "CMPCCXADD", "FRED", "INVLPGB", "RMPQUERY", "RMPREAD", "SNP",
        "SEVES", "HRESET", "TSXLDTRK", "UINTR", "USER_MSR", "MSRLIST",
        "WRMSRNS", "RDPRU", "SKINIT", "SVM", "VMX", "XSAVE", "XSAVEC",
        "XSAVEOPT", "XSS", "OSPKE", "PCONFIG", "WBNOINVD", "RDPID",
        "MONITOR", "MONITORX", "SMAP", "RDTSCP", "MSR_IMM", "PADLOCK",
        "486", "586", "686", "CMOV", "FXSR",
    ]
    
    def get_primary_feature(flags):
        """Extract the primary feature from a set of flags."""
        for feature in FEATURE_PRIORITY:
            if feature in flags:
                return feature
        return "BASE"
    
    return get_primary_feature

def rust_operand_check(ot_char, kind, idx):
    """Generate a Rust expression that checks if op{idx} is the correct operand type."""
    if ot_char == 'm':
        return f"op{idx}.is_mem()"
    elif ot_char == 'b':
        return f"op{idx}.is_mem()"
    elif ot_char == 'i':
        return f"op{idx}.is_imm()"
    elif ot_char == 'k':
        return f"op{idx}.is_mask()"
    elif ot_char in ('o', 'a'):
        return None  # no condition for label/address operands
    elif ot_char == 'r':
        # Use specific register group checks based on operand kind
        checks = {
            "GP":   f"op{idx}.is_gp()",
            "XMM":  f"op{idx}.is_vec()",
            "MMX":  f"op{idx}.is_reg_group_of(RegGroup::X86MM)",
            "MASK": f"op{idx}.is_mask()",
            "FPU":  f"op{idx}.is_reg_group_of(RegGroup::X86St)",
            "SEG":  f"op{idx}.is_reg_group_of(RegGroup::X86SReg)",
            "CR":   f"op{idx}.is_reg_group_of(RegGroup::X86CReg)",
            "DR":   f"op{idx}.is_reg_group_of(RegGroup::X86DReg)",
            "BND":  f"op{idx}.is_reg_group_of(RegGroup::X86Bnd)",
            "TMM":  f"op{idx}.is_reg_group_of(RegGroup::X86Tmm)",
        }
        return checks.get(kind, f"op{idx}.is_reg()")
    return None

def encode_table(entries, args):
    mnemonics = encode_mnems(entries)
    mnemonics["NOP", 0, ""] = [EncodeVariant(Opcode.parse("90"), InstrDesc.parse("NP - - - - NOP"))]
    opcode_docs = load_opcode_docs(args.docs_inputfolder)
    mnem_map = {}
    alt_table = [0] # first entry is unused

    # Group instructions by feature set
    feature_sets = defaultdict(lambda: Trait(""))
    feature_set_names = set()
    
    # Determine feature set for each instruction
    get_feature = get_instruction_feature_set(entries)

    # Collect function entries for grouping
    fn_entries = []

    for (mnem, opsize, ots), variants in mnemonics.items():
        supports_high_regs = []
        if variants[0][1].mnemonic in ("MOVSX", "MOVZX") or opsize == 8:
            # Should be the same for all variants
            desc = variants[0][1]
            for i, (ot, op) in enumerate(zip(ots, desc.operands)):
                if ot == "r" and op.kind == "GP" and op.abssize(opsize//8) == 1:
                    supports_high_regs.append(i)

        alt_indices = [i + len(alt_table) for i in range(len(variants) - 1)] + [0]

        if variants[0][1].encoding == "D":
            assert 1 <= len(variants) <= 2
            # We handle jump (jmp/jcc) alternatives in code to support Jcc.
            if len(variants) > 1:
                assert variants[0][1].mnemonic[:1] == "J"
                variants = variants[:1]
                alt_indices = [0xff]
        
        enc_opcs = []
        for alt, variant in zip(alt_indices, variants):
            opcode, desc = variant.opcode, variant.desc
            encoding = ENCODINGS[desc.encoding]
            opc_i = opcode.opc
            if None not in opcode.modrm:
                opc_i |= 0xc000 | opcode.modrm[1] << 11 | opcode.modrm[2] << 8
            elif opcode.modrm[1] is not None:
                opc_i |= opcode.modrm[1] << 8
            if opcode.modrm == ("m", None, 4):
                opc_i |= 0x2000000000 # FORCE_SIB
            if not opcode.vex:
                assert opcode.escape < 4
                opc_i |= opcode.escape * 0x10000
                opc_i |= 0x80000 if opcode.prefix == "66" or opsize == 16 else 0
                opc_i |= 0x100000 if opcode.prefix == "F2" else 0
                opc_i |= 0x200000 if opcode.prefix == "F3" else 0
            else:
                assert opcode.escape < 8
                opc_i |= opcode.escape * 0x10000
                if opcode.prefix == "66" or opsize == 16:
                    assert opcode.prefix not in ("F2", "F3")
                    opc_i |= 0x100000
                if opcode.prefix == "F3":
                    opc_i |= 0x200000
                elif opcode.prefix == "F2":
                    opc_i |= 0x300000
            opc_i |= 0x400000 if opcode.rexw == "1" else 0
            if opcode.prefix == "LOCK":
                opc_i |= 0x800000
            elif opcode.vex == 1:
                opc_i |= 0x1000000 + 0x800000 * int(opcode.vexl or 0)
            elif opcode.vex == 2:
                opc_i |= 0x2000000
                # L'L encodes SAE rounding mode otherwise
                if not variant.evexsae:
                    opc_i |= 0x800000 * int(opcode.vexl or 0)
            assert not (variant.evexsae and variant.evexbcst)
            opc_i |= 0x4000000 if variant.evexsae or variant.evexbcst else 0
            opc_i |= 0x8000000 if "VSIB" in desc.flags else 0
            opc_i |= 0x1000000000 if variant.evexmask == 2 else 0
            opc_i |= 0x4000000000 if variant.downgrade in (1, 2) else 0
            opc_i |= 0x40000000000 if variant.downgrade == 2 else 0
            opc_i |= 0x8000000000 * variant.evexdisp8scale
            if alt >= 0x100:
                raise Exception("encode alternate bits exhausted")
            opc_i |= sum(1 << i for i in supports_high_regs) << 45
            if encoding.imm_control >= 3:
                opc_i |= desc.imm_size(opsize//8) << 47
            elif encoding.imm_control in (1, 2):
                # Must be an arbitrary non-zero value, replaced by address size
                # for imm_ctl=2 and zero for imm_ctl=1 (constant 1).
                opc_i |= 1 << 47

            enc_encoding = desc.encoding
            if desc.encoding != "I" and desc.encoding.endswith("I"):
                enc_encoding = desc.encoding[:-1]
            elif desc.encoding == "IA":
                enc_encoding = "A"
            opc_i |= ["NP", "M", "R", "M1", "MC", "MR", "RM", "RMA", "MRC",
                "AM", "MA", "I", "O", "OA", "S", "A", "D", "FD", "TD", "IM",
                "RVM", "RVMR", "RMV", "VM", "MVR", "MRV",
            ].index(enc_encoding) << 51
            opc_i |= alt << 56
            enc_opcs.append(opc_i)
        mnem_map[f"{mnem.upper()}"] = enc_opcs[0]
        if mnem.startswith('3'):
            mnem = "_" + mnem

        nargs = len(desc.operands)
        rem = 4 - nargs
        if "LOCK_" in mnem or "REP_" in mnem or "REPNZ_" in mnem or "REPZ_" in mnem:
            pass
        else:
            # Determine feature set for this instruction
            variant_desc = variants[0].desc
            feature_flags = {flag for flag in variant_desc.flags if flag.startswith("F=")}
            feature_flags = {flag[2:] for flag in feature_flags}  # Remove "F=" prefix
            feature = get_feature(feature_flags) if feature_flags else "BASE"

            # Create trait name based on feature
            trait_name = f"X86{feature}Emitter" if feature != "BASE" else "X86BaseEmitter"
            trait_name = trait_name.replace("_", "")

            has_separate_opsize = "ENC_SEPSZ" in variant_desc.flags

            # Strip encoding suffixes (_mask, _maskz, _sae, _er) before
            # computing the unified name, so that ots stripping doesn't
            # corrupt them and variants are grouped correctly.
            extra_suffix = ""
            clean_mnem = mnem
            for s in ("_sae", "_er"):
                if clean_mnem.endswith(s):
                    extra_suffix = s + extra_suffix
                    clean_mnem = clean_mnem[:-len(s)]
                    break
            for s in ("_maskz", "_mask"):
                if clean_mnem.endswith(s):
                    extra_suffix = s + extra_suffix
                    clean_mnem = clean_mnem[:-len(s)]
                    break

            # Compute unified name by stripping r/m/i from the operand-type suffix
            ots_suffix = ots.replace("o", "")
            if ots_suffix and not has_separate_opsize:
                # Residual: non-dispatchable characters (k, b, a, etc.)
                residual = "".join(c for c in ots_suffix if c not in ("r", "m", "i"))
                base_name = clean_mnem[:-len(ots_suffix)]
                unified_name = base_name + residual + extra_suffix
            else:
                unified_name = clean_mnem + extra_suffix

            canonical_mnemonic = canonicalize_doc_mnemonic(variants[0].desc.mnemonic)

            # Capture operand kind info for richer type checks
            op_kinds = []
            for i in range(nargs):
                ot_char = ots[i] if i < len(ots) else ''
                if ot_char in ('r', 'k'):
                    op_kinds.append(variant_desc.operands[i].kind)
                elif ot_char in ('m', 'b'):
                    op_kinds.append('MEM')
                elif ot_char == 'i':
                    op_kinds.append('IMM')
                else:
                    op_kinds.append('UNKNOWN')

            fn_entries.append({
                'mnem': mnem,
                'unified_name': unified_name,
                'ots': ots,
                'nargs': nargs,
                'trait_name': trait_name,
                'canonical_mnemonic': canonical_mnemonic,
                'op_kinds': op_kinds,
            })

        alt_table += enc_opcs[1:]

    # Phase 2: Group function entries by (unified_name, nargs, trait_name)
    fn_groups = defaultdict(list)
    for entry in fn_entries:
        key = (entry['unified_name'], entry['nargs'], entry['trait_name'])
        fn_groups[key].append(entry)

    # Phase 3: Detect nargs collisions within same (unified_name, trait_name)
    base_nargs_map = defaultdict(set)
    for (unified_name, nargs, trait_name) in fn_groups:
        base_nargs_map[(unified_name, trait_name)].add(nargs)

    # Phase 4: Generate grouped dispatch functions
    for (unified_name, nargs, trait_name), group_entries in sorted(fn_groups.items()):
        nargs_options = base_nargs_map[(unified_name, trait_name)]

        # Determine function name
        if len(nargs_options) > 1:
            # Multiple arg counts for same unified name - disambiguate
            # Keep the group with the most entries as the base name
            group_sizes = {n: len(fn_groups[(unified_name, n, trait_name)]) for n in nargs_options}
            primary_nargs = max(nargs_options, key=lambda n: (group_sizes[n], -n))
            if nargs == primary_nargs:
                fn_name = unified_name.lower()
            else:
                fn_name = f"{unified_name}_{nargs}".lower() if nargs > 0 else unified_name.lower()
                # Avoid collisions with 0-arg primary
                if nargs == 0 and primary_nargs != 0:
                    fn_name = f"{unified_name}_0".lower()
        else:
            fn_name = unified_name.lower()

        # Get or create the trait
        if trait_name not in feature_sets:
            feature_sets[trait_name] = Trait(f"{trait_name}: Emitter")
            feature_set_names.add(trait_name)

        # Deduplicate: if multiple entries have the same ots pattern, keep one
        seen_ots = {}
        deduped_entries = []
        for entry in group_entries:
            if entry['ots'] not in seen_ots:
                seen_ots[entry['ots']] = entry
                deduped_entries.append(entry)
        group_entries = deduped_entries

        rem = 4 - nargs
        noreg_args = ",".join(f"&NOREG" for _ in range(rem))

        # Build the function
        func = Function(fn_name)
        func.self_mut()
        func.ret("()")
        

        # Use doc from the first entry's canonical mnemonic
        canonical_mnemonic = group_entries[0]['canonical_mnemonic']
        doc = opcode_docs.get(canonical_mnemonic)
        if len(group_entries) == 1:
            opc_name = group_entries[0]['mnem'].upper()
            doc_name = opc_name
        else:
            doc_name = unified_name.upper()
        if doc:
            func.doc(f"Emits `{doc_name}` (`{canonical_mnemonic}`). {doc['tooltip']}")
            func.doc(f"Reference: [Intel x86 docs for {canonical_mnemonic}]({doc['url']})")
        else:
            func.doc(f"Emits `{doc_name}`.")

        for i in range(nargs):
            func.arg(Field(f"op{i}", "impl OperandCast"))

        block = Block()

        # Escape the doc_name for use in error messages
        err_mnem = doc_name.replace('"', '\\"')

        if len(group_entries) == 1:
            # Single variant - emit directly, no dispatch needed
            entry = group_entries[0]
            opc = entry['mnem'].upper()
            op_args = "".join(f"op{i}.as_operand()," for i in range(nargs))
            block.line(f"self.emit({opc}, {op_args}{noreg_args});")
        else:
            # Multiple variants - generate dispatch based on operand types
            # Convert operands to Operand references first
            for i in range(nargs):
                block.line(f"let op{i} = op{i}.as_operand();")

            # Sort entries: most specific first (more conditions), fallback last
            def entry_specificity(e):
                return sum(1 for c in e['ots'] if c not in ("o", " ", "a") and c != "")
            sorted_entries = sorted(group_entries, key=entry_specificity, reverse=True)

            first_cond = True
            fallback_entry = None
            for entry in sorted_entries:
                ots = entry['ots']
                opc = entry['mnem'].upper()
                op_kinds = entry.get('op_kinds', [])

                conditions = []
                for i, c in enumerate(ots):
                    kind = op_kinds[i] if i < len(op_kinds) else 'UNKNOWN'
                    check = rust_operand_check(c, kind, i)
                    if check is not None:
                        conditions.append(check)

                if not conditions:
                    # No conditions - this is a fallback (e.g., label/symbol operand)
                    fallback_entry = entry
                    continue

                cond_str = " && ".join(conditions)
                if first_cond:
                    block.line(f"if {cond_str} {{")
                    first_cond = False
                else:
                    block.line(f"}} else if {cond_str} {{")
                op_args = ",".join(f"op{i}" for i in range(nargs))
                block.line(f"    self.emit({opc}, {op_args},{noreg_args});")

            if fallback_entry:
                opc = fallback_entry['mnem'].upper()
                op_args = ",".join(f"op{i}" for i in range(nargs))
                if not first_cond:
                    block.line("} else {")
                    block.line(f"    self.emit({opc}, {op_args},{noreg_args});")
                else:
                    block.line(f"self.emit({opc}, {op_args},{noreg_args});")
            else:
                if not first_cond:
                    block.line("} else {")
                    block.line(f'    unreachable!("invalid operand types for {err_mnem}");')

            if not first_cond:
                block.line("}")

        func.body(block)
        feature_sets[trait_name].items.append(func)

    mnem_tab = "".join(f"pub const {m if not m.startswith('3') else '_' + m}: i64 = {v:#x}u64 as i64;\n" for m, v in mnem_map.items())

    alt_tab = f"pub static ALT_TAB: [i64; {len(alt_table)}] = [\n";
    for v in alt_table:
        alt_tab += f"{v:#x}u64 as i64,\n"
    alt_tab += "];"

    # Generate all traits
    all_traits = []
    for trait_name in sorted(feature_set_names):
        trait = feature_sets[trait_name]

        all_traits.append(trait)

    return f"{mnem_tab}\n{alt_tab}", all_traits

def unique(it):
    vals = set(it)
    if len(vals) != 1:
        raise Exception(f"multiple values: {vals}")
    return next(iter(vals))


def canonicalize_doc_mnemonic(mnemonic):
    mnemonic = mnemonic.replace("EVX_", "V")
    mnemonic = mnemonic.replace("SSE_", "")
    mnemonic = mnemonic.replace("MMX_", "")
    mnemonic = mnemonic.replace("MOVABS", "MOV")
    mnemonic = mnemonic.replace("XCHG_NOP", "XCHG")
    mnemonic = mnemonic.replace("_CR", "")
    mnemonic = mnemonic.replace("_DR", "")
    mnemonic = mnemonic.replace("_S2G", "")
    mnemonic = mnemonic.replace("_G2S", "")
    mnemonic = mnemonic.replace("_X2G", "")
    mnemonic = mnemonic.replace("_G2X", "")
    mnemonic = mnemonic.replace("REP_", "")
    mnemonic = mnemonic.replace("REPNZ_", "")
    return mnemonic.upper()


def load_opcode_docs(inputfolder):
    try:
        return collect_instruction_docs(inputfolder)
    except Exception as exc:
        print(f"Warning: failed to load asm docs from {inputfolder}: {exc}")
        return {}

def encode2_gen_legacy(variant: EncodeVariant, opsize: int, supports_high_regs: list[int], imm_expr: str, imm_size_expr: str, has_idx: bool) -> list[str]:
    opcode = variant.opcode
    desc = variant.desc
    flags = ENCODINGS[variant.desc.encoding]
    code = []

    rex_expr = "0" if opcode.rexw != "1" else "0x48"
    for i in supports_high_regs:
        rex_expr += f"|(if op_reg_idx(op{i}) >= 4 && op_reg_idx(op{i}) <= 15 {{ 0x40 }} else {{ 0 }})"
    if flags.modrm_idx:
        if opcode.modrm[0] == "m":
            rex_expr += f"|(if op_mem_base(op{flags.modrm_idx^3}) & 8 != 0 {{ 0x41 }} else {{ 0 }})"
            rex_expr += f"|(if op_mem_idx(op{flags.modrm_idx^3}) & 8 != 0 {{ 0x42 }} else {{ 0 }})"
        elif desc.operands[flags.modrm_idx^3].kind in ("GP", "XMM"):
            rex_expr += f"|(if op_reg_idx(op{flags.modrm_idx^3}) & 8 != 0 {{ 0x41 }} else {{ 0 }})"
        if flags.modreg_idx:
            if desc.operands[flags.modreg_idx^3].kind in ("GP", "XMM", "CR", "DR"):
                rex_expr += f"|(if op_reg_idx(op{flags.modreg_idx^3}) & 8 != 0 {{ 0x44 }} else {{ 0 }})"
    elif flags.modreg_idx: # O encoding
        if desc.operands[flags.modreg_idx^3].kind in ("GP", "XMM"):
            rex_expr += f"|(if op_reg_idx(op{flags.modreg_idx^3}) & 8 != 0 {{ 0x41 }} else {{ 0 }})"

    if rex_expr != "0":
        code.append(f"let rex = {rex_expr};")
    for i in supports_high_regs:
        code.append(f"if rex != 0 && op_reg_gph(op{i}) {{ return 0; }}")

    if not has_idx:
        code.append("let mut idx: usize = 0;")
    if opcode.prefix == "LOCK":
        code.append("buf[idx] = 0xF0; idx += 1;")
    if opsize == 16 or opcode.prefix == "66":
        code.append("buf[idx] = 0x66; idx += 1;")
    if opcode.prefix in ("F2", "F3"):
        code.append(f"buf[idx] = 0x{opcode.prefix}; idx += 1;")
    if opcode.rexw == "1":
        code.append("buf[idx] = rex as u8; idx += 1;")
    elif rex_expr != "0":
        code.append("if rex != 0 { buf[idx] = rex as u8; idx += 1; }")
    if opcode.escape:
        code.append("buf[idx] = 0x0F; idx += 1;")
        if opcode.escape == 2:
            code.append("buf[idx] = 0x38; idx += 1;")
        elif opcode.escape == 3:
            code.append("buf[idx] = 0x3A; idx += 1;")
    opcodestr = f"{opcode.opc:#x}" + ("|(flags>>16)" if variant.flexcc else "")
    code.append(f"buf[idx] = ({opcodestr}) as u8; idx += 1;")
    if None not in opcode.modrm:
        opcext = 0xc0 | opcode.modrm[1] << 3 | opcode.modrm[2]
        code.append(f"buf[idx] = {opcext:#x}; idx += 1;")

    if flags.modrm:
        if flags.modreg_idx:
            modreg = f"op_reg_idx(op{flags.modreg_idx^3})"
        else:
            modreg = opcode.modrm[1] or 0
        if opcode.modrm[0] == "m":
            assert "VSIB" not in desc.flags
            assert opcode.modrm[2] is None
            modrm = f"op{flags.modrm_idx^3}"
            code.append(f"let memoff = enc_mem(&mut buf[idx..], idx + ({imm_size_expr}) as usize, {modrm}, {modreg}, 0, 0);")
            code.append("if memoff == 0 { return 0; }")
            code.append("idx += memoff;")
        else:
            if flags.modrm_idx:
                modrm = f"op_reg_idx(op{flags.modrm_idx^3})"
            else:
                modrm = f"{opcode.modrm[2] or 0}"
            code.append(f"buf[idx] = (0xC0 | (({modreg}) << 3) | (({modrm}) & 7)) as u8; idx += 1;")
    elif flags.modrm_idx:
        code.append(f"buf[idx - 1] |= (op_reg_idx(op{flags.modrm_idx^3}) & 7) as u8;")
    if flags.imm_control >= 2:
        if flags.imm_control == 6:
            imm_expr += " - idx as i64"
        code.append(f"enc_imm(&mut buf[idx..], {imm_expr}, ({imm_size_expr}) as usize);")
        code.append(f"return idx + ({imm_size_expr}) as usize;")
    else:
        code.append("return idx;")
    return code

def encode2_gen_vex(variant: EncodeVariant, imm_expr: str, imm_size_expr: str, has_idx: bool) -> list[str]:
    opcode = variant.opcode
    flags = ENCODINGS[variant.desc.encoding]
    code = []

    helperopc = opcode.opc << 16
    helperopc |= ["NP", "66", "F3", "F2"].index(opcode.prefix) << 8
    helperopc |= 0x8000 if opcode.rexw == "1" else 0
    if not variant.evexsae:
        # ER: L'L encodes rounding mode for SAE
        helperopc |= 0x0020 * int(opcode.vexl or 0) # EVEX.L'L
    helperopc |= opcode.escape << 10
    helperopc |= 0x10 if variant.evexsae or variant.evexbcst else 0 # EVEX.b
    helperopc |= 0x80 if variant.evexmask == 2 else 0 # EVEX.z
    helperopc |= 0x1000000 if variant.downgrade in (1, 2) else 0
    helperopc |= 0x2000000 if variant.downgrade == 2 else 0
    helperopc = f"{helperopc:#x}"
    if variant.flexcc:
        helperopc += "|(flags&FE_CC_MASK)"
    if variant.evexsae == 2:
        helperopc += "|(flags&FE_RC_MASK)"
    if variant.evexmask:
        code.append("if op_reg_idx(opmask) == 0 { return 0; }")
        helperopc += "|(op_reg_idx(opmask)&7)"

    if flags.modreg_idx:
        modreg = f"op_reg_idx(op{flags.modreg_idx^3})"
    else:
        modreg = opcode.modrm[1] or 0
    vexop = f"op_reg_idx(op{flags.vexreg_idx^3})" if flags.vexreg_idx else 0
    if not flags.modrm and opcode.modrm == (None, None, None):
        # No ModRM, prefix only (VZEROUPPER/VZEROALL)
        assert opcode.vex == 1
        helperfn, helperargs = "enc_vex_common", f"0, 0, 0, 0"
    elif opcode.modrm[0] == "m":
        vsib = "VSIB" in variant.desc.flags
        helperfn = "enc" + ["", "_vex", "_evex"][opcode.vex] + ["_mem", "_vsib"][vsib]
        assert opcode.modrm[2] in (None, 4)
        forcesib = 1 if opcode.modrm[2] == 4 else 0 # AMX
        modrm = f"op{flags.modrm_idx^3}"
        ripoff = imm_size_expr + ("" if not has_idx else "+idx")
        helperargs = (f"{modrm}, {modreg}, {vexop}, {ripoff}, " +
                      f"{forcesib}, {variant.evexdisp8scale}")
    else:
        if flags.modrm_idx:
            modrm = f"op_reg_idx(op{flags.modrm_idx^3})"
        else:
            modrm = f"{opcode.modrm[2] or 0}"
        suffix = "_reg"
        if (opcode.vex == 2 and flags.modrm_idx and
            variant.desc.operands[flags.modrm_idx^3].kind == "XMM"):
            suffix = "_xmm"
        helperfn = "enc" + ["", "_vex", "_evex"][opcode.vex] + suffix
        helperargs = f"{modrm}, {modreg}, {vexop}"
    bufidx = "buf" if not has_idx else "&mut buf[idx..]"
    helpercall = f"{helperfn}({bufidx}, {helperopc}, {helperargs})"
    if flags.imm_control >= 2:
        assert flags.imm_control < 6, "jmp with VEX/EVEX?"
        code.append(f"let vexoff = {helpercall};")
        if has_idx:
            code.append(f"enc_imm(&mut buf[idx + vexoff..], {imm_expr}, ({imm_size_expr}) as usize);")
            code.append(f"return if vexoff != 0 {{ vexoff + ({imm_size_expr}) as usize + idx }} else {{ 0 }};")
        else:
            code.append(f"enc_imm(&mut buf[vexoff..], {imm_expr}, ({imm_size_expr}) as usize);")
            code.append(f"return if vexoff != 0 {{ vexoff + ({imm_size_expr}) as usize }} else {{ 0 }};")
    elif has_idx:
        code.append(f"let vexoff = {helpercall};")
        code.append("return if vexoff != 0 { vexoff + idx } else { 0 };")
    else:
        code.append(f"return {helpercall};")
    return code

def encode2_table(entries, args):
    mnemonics = encode_mnems(entries)

    enc_decls = ["// Rust signatures generated by encode2."]
    
    # Group functions by feature set
    feature_functions = defaultdict(TopCode)
    get_feature = get_instruction_feature_set(entries)

    for (mnem, opsize, ots), variants in mnemonics.items():
        max_imm_size = max(v.desc.imm_size(opsize//8) for v in variants)

        supports_high_regs = []
        if variants[0].desc.mnemonic in ("MOVSX", "MOVZX") or opsize == 8:
            # Should be the same for all variants
            for i, (ot, op) in enumerate(zip(ots, variants[0].desc.operands)):
                if ot == "r" and op.kind == "GP" and op.abssize(opsize//8) == 1:
                    supports_high_regs.append(i)
        supports_vsib = unique("VSIB" in v.desc.flags for v in variants)
        opkinds = unique(tuple(op.kind for op in v.desc.operands) for v in variants)
        evexmask = unique(v.evexmask for v in variants)
        evexsae = unique(v.evexsae for v in variants)

        OPKIND_LUT = {"FPU": "ST", "SEG": "SREG", "MMX": "MM"}
        reg_tys = [OPKIND_LUT.get(opkind, opkind) for opkind in opkinds]

        fnname = f"fe64_{mnem}"
        op_tys = [{
            "i": f"i{max_imm_size*8 if max_imm_size != 3 else 32}",
            "a": "usize",
            "r": f"FeReg{reg_ty if i not in supports_high_regs else 'GPLH'}",
            "k": "FeRegMASK",
            "m": "FeMem" if not supports_vsib else "FeMemV",
            "b": "FeMem",
            "o": "*const core::ffi::c_void",
        }[ot] for i, (ot, reg_ty) in enumerate(zip(ots, reg_tys))]
        fn_args = [Field("buf", "&mut [u8]"), Field("flags", "u64")]
        if evexmask:
            fn_args.append(Field("opmask", "FeRegMASK"))
        for i, ty in enumerate(op_tys):
            fn_args.append(Field(f"op{i}", ty))

        enc_decls.append(
            "// pub fn " + fnname + "(" + ", ".join(f"{a.name}: {a.ty}" for a in fn_args) + ") -> usize;"
        )

        fn = Function(fnname)
        fn.pub()
        fn.ret("usize")
        for arg in fn_args:
            fn.arg(arg)
        body = Block()

        has_memory = unique(v.opcode.modrm[0] == "m" for v in variants)
        has_useg = unique("USEG" in v.desc.flags for v in variants)
        has_u67 = unique("U67" in v.desc.flags for v in variants)
        if has_memory or has_useg:
            # segment override without addrsize override shouldn't happen
            assert has_memory or has_u67
            body.line("let mut idx = if unlikely((flags & (FE_SEG_MASK | FE_ADDR32)) != 0) { enc_seg67(buf, flags) } else { 0 };")
        elif has_u67:
            # STOS, SCAS, JCXZ, LOOP, LOOPcc
            body.line("let mut idx = if unlikely((flags & FE_ADDR32) != 0) { buf[0] = 0x67; 1usize } else { 0usize };")
        else:
            body.line("let _ = flags;")

        # indicate whether an idx variable exists
        has_idx = has_memory or has_useg or has_u67

        for i, variant in enumerate(variants):
            opcode, desc = variant.opcode, variant.desc
            flags = ENCODINGS[desc.encoding]

            conds = []
            # Select usable encoding.
            if desc.encoding == "S":
                # Segment encoding is weird.
                conds.append(f"op_reg_idx(op0)=={(opcode.opc>>3)&0x7:#x}")
            if desc.mnemonic == "XCHG_NOP" and opsize == 32:
                # XCHG eax, eax must not be encoded as 90 -- that'd be NOP.
                conds.append(f"!(op_reg_idx(op0)==0&&op_reg_idx(op1)==0)")
            if flags.vexreg_idx and not opcode.vex: # vexreg w/o vex is zeroreg
                conds.append(f"op_reg_idx(op{flags.vexreg_idx^3})=={flags.zeroreg_val}")

            imm_size = desc.imm_size(opsize//8)
            imm_size_expr = f"{imm_size}"
            imm_expr = f"op{flags.imm_idx^3} as i64"
            if flags.imm_control == 1:
                conds.append(f"op{flags.imm_idx^3} == 1")
            elif flags.imm_control == 2:
                imm_size_expr = "if (flags & FE_ADDR32) != 0 { 4 } else { 8 }"
                imm_expr = f"if (flags & FE_ADDR32) != 0 {{ ({imm_expr}) as i32 as i64 }} else {{ {imm_expr} }}"
            elif flags.imm_control == 3:
                imm_expr = f"(op_reg_idx(op{flags.imm_idx^3}) << 4) as i64"
                body.line(f"if op_reg_idx(op{flags.imm_idx^3}) >= 16 {{ return 0; }}")
            elif flags.imm_control == 4 and imm_size == 3: # ENTER
                body.line(f"if (op{flags.imm_idx^3} as u32) >= 0x1000000 {{ return 0; }}")
            elif flags.imm_control == 4 and imm_size < max_imm_size:
                conds.append(f"op_imm_n({imm_expr}, {imm_size})")
            elif flags.imm_control == 6:
                imm_expr = f"{imm_expr} - buf.as_ptr() as i64 - {imm_size}"
                if i != len(variants) - 1: # only Jcc+JMP
                    conds.append(f"(flags & FE_JMPL) == 0")
                    # assume one-byte opcode without escape/prefixes
                    conds.append(f"op_imm_n({imm_expr}-1, {imm_size})")

            if conds:
                body.line(f"if {' && '.join(conds)} {{")

            if opcode.vex:
                gen_lines = encode2_gen_vex(variant, imm_expr, imm_size_expr, has_idx)
            else:
                gen_lines = encode2_gen_legacy(variant, opsize, supports_high_regs, imm_expr, imm_size_expr, has_idx)
            for line in gen_lines:
                body.line(("    " if conds else "") + line)

            if conds:
                body.line("}")
            else:
                break
        else:
            body.line("return 0;")

        fn.body(body)
        
        # Determine feature set for this instruction
        variant_desc = variants[0].desc
        feature_flags = {flag for flag in variant_desc.flags if flag.startswith("F=")}
        feature_flags = {flag[2:] for flag in feature_flags}  # Remove "F=" prefix
        feature = get_feature(feature_flags) if feature_flags else "BASE"
        
        # Add to appropriate feature group
        feature_functions[feature].items.append(fn)

    # Generate separate outputs for each feature set
    all_outputs = []
    for feature in sorted(feature_functions.keys()):
        fmt = Formatter()
        feature_functions[feature].fmt(fmt)
        all_outputs.append(f"// === {feature} ===\n{fmt.dst}")
    
    return "\n".join(enc_decls) + "\n", "\n\n".join(all_outputs)


if __name__ == "__main__":
    generators = {
        "decode": decode_table,
        "encode": encode_table,
        "encode2": encode2_table,
        "encode3": encode3,
    }

    parser = argparse.ArgumentParser()
    parser.add_argument("--32", dest="modes", action="append_const", const=32)
    parser.add_argument("--64", dest="modes", action="append_const", const=64)
    parser.add_argument("--with-undoc", action="store_true")
    parser.add_argument("--stats", action="store_true")
    parser.add_argument("--docs-inputfolder", type=str, default="asm-docs")
    parser.add_argument("mode", choices=generators.keys())
    parser.add_argument("table", type=str)
    parser.add_argument("out_public", type=str)
    parser.add_argument("out_private", type=str)
    args = parser.parse_args()

    out_public = open(args.out_public, "w")
    table = open(args.table, "r")


    entries = []
    for line in table.read().splitlines():
        if not line or line[0] == "#": continue
        line, weak = (line, False) if line[0] != "*" else (line[1:], True)
        opcode_string, desc_string = tuple(line.split(maxsplit=1))
        opcode, desc = Opcode.parse(opcode_string), InstrDesc.parse(desc_string)
        verifyOpcodeDesc(opcode, desc)
        if "UNDOC" not in desc.flags or args.with_undoc:
            entries.append((weak, opcode, desc))

    if args.mode == "encode":
        res_public, traits = generators[args.mode](entries, args)
        out_public.write(res_public)
        # Handle traits if needed
        for trait in traits: 
            # Extract feature name from trait name (e.g., "X86AVX512Emitter" -> "AVX512")
            trait_name = trait.name
            if trait_name.startswith("X86") and trait_name.endswith("Emitter"):
                feature_name = trait_name[3:-7]  # Remove "X86" prefix and "Emitter" suffix
            else:
                feature_name = trait_name
            feature_name = feature_name.replace("Emitter: ", "")
            with open(f"{args.out_private}/{feature_name}.rs", "w") as trait_file:
                fmt = Formatter()
                trait.fmt(fmt)
                trait_file.write(fmt.dst)
        
        with open(f"{args.out_private}/mod.rs", "w") as mod_file:
            mod_file.write("use crate::x86::assembler::*;\n")
            mod_file.write("use crate::{AsmError, X86Error};\n")
            mod_file.write("use super::opcodes::*;\n")
            mod_file.write("use crate::core::emitter::*;\n")
            mod_file.write("use crate::core::operand::*;\n\n")
            mod_file.write("/// A dummy operand that represents no register. Here just for simplicity.\n")
            mod_file.write("const NOREG: Operand = Operand::new();\n")
            for trait in traits:
                trait_name = trait.name
                if trait_name.startswith("X86") and trait_name.endswith("Emitter"):
                    feature_name = trait_name[3:-7]  # Remove "X86" prefix and "Emitter" suffix
                else:
                    feature_name = trait_name
                feature_name = feature_name.replace("Emitter: ", "")
                mod_file.write(f"include!(\"{feature_name}.rs\");\n")

            mod_file.write("\n")
            for trait in traits: 
                trait_name = trait.name.replace(": Emitter", "")
                mod_file.write(f"impl<'a> {trait_name} for Assembler<'a> {{}}\n")
    elif args.mode == "encode3":
        import os
        res_public, feature_code, dyn_str = generators[args.mode](entries, args)
        out_public.write(res_public)
        os.makedirs(args.out_private, exist_ok=True)
        for feature_name, code in sorted(feature_code.items()):
            with open(os.path.join(args.out_private, f"{feature_name}.rs"), "w") as f:
                f.write(code)
        # Write dynamic-dispatch module (_DYN.rs) and text-assembler module (_PARSER.rs)
        with open(os.path.join(args.out_private, "_DYN.rs"), "w") as f:
            f.write(dyn_str)
        with open(os.path.join(args.out_private, "mod.rs"), "w") as mod_file:
            mod_file.write("#![allow(unused, non_camel_case_types)]\n")
            for feature_name in sorted(feature_code.keys()):
                # Rust module names must be valid identifiers (no digit prefix)
                mod_name = feature_name.lower()
                if mod_name[0].isdigit():
                    mod_name = '_' + mod_name
                if mod_name != feature_name:
                    mod_file.write(f'#[path = "{feature_name}.rs"] pub mod {mod_name};\n')
                else:
                    mod_file.write(f'pub mod {mod_name};\n')
            # Dynamic dispatch module — requires the `x86-dyn` Cargo feature
            mod_file.write('\n')
            mod_file.write('#[cfg(feature = "x86-dyn")]\n')
            mod_file.write('#[path = "_DYN.rs"]\n')
            mod_file.write('pub mod dyn_emit;\n')
            # Text assembler module — requires the `x86-asm` Cargo feature (implies x86-dyn)
            mod_file.write('\n')
            mod_file.write('#[cfg(feature = "x86-asm")]\n')
            mod_file.write('#[path = "_PARSER.rs"]\n')
            mod_file.write('pub mod text_asm;\n')
    else: 
        out_private = open(args.out_private, "w")
        res_public, res_private = generators[args.mode](entries, args)
        out_public.write(res_public)
        out_private.write(res_private)
        out_private.close()
    out_public.close()
    table.close()
