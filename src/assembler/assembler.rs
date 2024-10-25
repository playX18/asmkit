use super::{
    binemit::{Label, LabelUse},
    codeholder::CodeBuffer,
};

pub struct BaseAssembler<'a> {
    pub buffer: &'a mut CodeBuffer,
}

impl<'a> BaseAssembler<'a> {
    pub fn offset(&self) -> u32 {
        self.buffer.cur_offset()
    }
    /// Use label at specific offset.
    pub fn use_label_at_offset(&mut self, offset: u32, label: Label, label_use: LabelUse) {
        self.buffer.use_label_at_offset(offset, label, label_use);
    }

    pub fn bind_label(&mut self, label: Label) {
        self.buffer.bind_label(label);
    }
}
