use std::{
    fs::File,
    io::{Read, Result},
};

use cave_core::ee::{interp::Interpreter, interp::RunMode, CpuState};
use memmap::MmapOptions;
use test_arch::{interp::TestArchInstructionExecutor, RegisterState};

fn main() -> Result<()> {
    let mut mem = MmapOptions::new().len(0x1000).map_anon().unwrap();

    let mut file = File::open("./fib.ta")?;
    let mut buf = Vec::new();
    let s = file.read_to_end(&mut buf)?;

    unsafe {
        std::ptr::copy(buf.as_ptr(), mem.as_mut_ptr(), s);
    }

    let mut state = CpuState::new(mem.as_mut_ptr(), RegisterState::new());
    Interpreter::<RegisterState, TestArchInstructionExecutor>::run(RunMode::Forever, &mut state)
        .expect("error");

    println!("{:?}", state);

    Ok(())
}
