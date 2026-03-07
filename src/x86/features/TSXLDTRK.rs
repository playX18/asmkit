pub trait X86TSXLDTRKEmitter: Emitter {
    /// Emits `XRESLDTRK`.
    fn xresldtrk(&mut self,) -> () {
        self.emit(XRESLDTRK, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `XSUSLDTRK`.
    fn xsusldtrk(&mut self,) -> () {
        self.emit(XSUSLDTRK, &NOREG,&NOREG,&NOREG,&NOREG);
    }
}
