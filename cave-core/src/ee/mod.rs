pub mod interp;

#[derive(Debug)]
pub struct CpuState<R> {
    pub pc: usize,
    pub regstate: R,
    pub mem: *mut u8,
}

impl<R> CpuState<R> {
    pub fn new(mem: *mut u8, regstate: R) -> Self {
        Self {
            pc: 0,
            regstate,
            mem,
        }
    }
}
