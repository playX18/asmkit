pub trait X86SNPEmitter: Emitter {
    /// Emits `PSMASH`.
    fn psmash(&mut self,) -> () {
        self.emit(PSMASH, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `PVALIDATE`.
    fn pvalidate(&mut self,) -> () {
        self.emit(PVALIDATE, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `RMPADJUST`.
    fn rmpadjust(&mut self,) -> () {
        self.emit(RMPADJUST, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `RMPUPDATE`.
    fn rmpupdate(&mut self,) -> () {
        self.emit(RMPUPDATE, &NOREG,&NOREG,&NOREG,&NOREG);
    }
}
