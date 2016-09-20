use binary;
use std;
use std::io;
use std::io::Write;


#[derive(Debug)]
pub enum Error {
    LoadError(binary::Error),
}

impl std::convert::From<binary::Error> for Error {
    fn from(e: binary::Error) -> Self {
        Error::LoadError(e)
    }
}

pub trait LogError<E> {
    fn log_err<'a, F>(self, f: F) -> Self where F: FnOnce(&E) -> String;
}

impl<T, E: std::fmt::Debug> LogError<E> for Result<T, E> {
    fn log_err<'a, F>(self, f: F) -> Self
        where F: FnOnce(&E) -> String
    {
        match self {
            Ok(_) => (),
            Err(ref e) => {
                let mut stderr = io::stderr();
                stderr.write(format!("{}: {:?}", f(e), e).as_bytes()).unwrap();
                stderr.write(&[0xa]).unwrap();
            }
        };
        return self;
    }
}
