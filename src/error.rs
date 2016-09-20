use std;
use std::io;
use unicorn;
use emulator;


#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    UnknownFormat,
    UnsupportedArch(String),
    ParserError(String),
    EmuError(unicorn::Error),
    MapAlreadyExists(emulator::MemMap),
}

impl std::convert::From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IoError(e)
    }
}

impl std::convert::From<unicorn::Error> for Error {
    fn from(e: unicorn::Error) -> Self {
        Error::EmuError(e)
    }
}
