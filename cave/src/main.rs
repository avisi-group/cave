use std::{
    fs::File,
    io::{Read, Result},
};

use cave_core::ee::interp::Interpreter;
use cave_core::isa::Isa;
use memmap::MmapOptions;
use test_arch::{interp::TestArchInterpreter, CpuState, TestIsa};

fn main() -> Result<()> {
    let isa = TestIsa::new();

    let mut mem = MmapOptions::new().len(0x1000).map_anon().unwrap();

    let mut file = File::open("./fib.ta")?;
    let mut buf = Vec::new();
    let s = file.read_to_end(&mut buf)?;

    unsafe {
        std::ptr::copy(buf.as_ptr(), mem.as_mut_ptr(), s);
    }

    let mut state = CpuState::new(mem.as_mut_ptr());
    let mut interp = TestArchInterpreter::new(&mut state);

    interp
        .step(cave_core::ee::interp::StepMode::Forever)
        .expect("step failed");

    println!("{:?}", state);

    Ok(())
}
