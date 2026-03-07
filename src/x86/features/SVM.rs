pub trait X86SVMEmitter: Emitter {
    /// Emits `INVLPGA`.
    fn invlpga(&mut self,) -> () {
        self.emit(INVLPGA, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `VMLOAD`.
    fn vmload(&mut self,) -> () {
        self.emit(VMLOAD, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `VMMCALL`.
    fn vmmcall(&mut self,) -> () {
        self.emit(VMMCALL, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `VMRUN`.
    fn vmrun(&mut self,) -> () {
        self.emit(VMRUN, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `VMSAVE`.
    fn vmsave(&mut self,) -> () {
        self.emit(VMSAVE, &NOREG,&NOREG,&NOREG,&NOREG);
    }
}
