pub trait X86SVMEmitter: Emitter {
    /// Emits `INVLPGA`.
    fn invlpga(&mut self,) -> Result<(), AsmError> {
        self.emit(INVLPGA, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VMLOAD`.
    fn vmload(&mut self,) -> Result<(), AsmError> {
        self.emit(VMLOAD, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VMMCALL`.
    fn vmmcall(&mut self,) -> Result<(), AsmError> {
        self.emit(VMMCALL, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VMRUN`.
    fn vmrun(&mut self,) -> Result<(), AsmError> {
        self.emit(VMRUN, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VMSAVE`.
    fn vmsave(&mut self,) -> Result<(), AsmError> {
        self.emit(VMSAVE, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
