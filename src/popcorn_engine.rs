use binary;
use Error;

use emulator;
use std::path::Path;

pub struct PopcornEngine {
    emulator: Box<emulator::Emulator>,
}

pub struct ExecResult(u64);

impl PopcornEngine {
    pub fn new<PathRef: AsRef<Path>>(path: PathRef) -> Result<PopcornEngine, Error> {
        Ok(PopcornEngine { emulator: try!(binary::load(path)) })
    }

    pub fn setup_env(&self) {}

    pub fn run(&mut self, _args: &[&str]) -> Result<ExecResult, Error> {
        let result = try!(self.emulator.call_addr(0, &[]));
        Ok(ExecResult(result))
    }
}

impl ExecResult {
    pub fn stdout(&self) -> &str {
        ""
    }
}
