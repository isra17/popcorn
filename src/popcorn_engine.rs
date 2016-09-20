use binary;
use Error;

use emulator;
use std::path::Path;

pub struct PopcornEngine {
    emulator: Box<emulator::Emulator>,
}

pub struct ExecResult {}

impl PopcornEngine {
    pub fn new<PathRef: AsRef<Path>>(path: PathRef) -> Result<PopcornEngine, Error> {
        Ok(PopcornEngine { emulator: try!(binary::load(path)) })
    }

    pub fn setup_env(&self) {}

    pub fn run(&self, args: &[&str]) -> Result<ExecResult, ()> {
        Ok(ExecResult {})
    }
}

impl ExecResult {
    pub fn stdout(&self) -> &str {
        ""
    }
}
