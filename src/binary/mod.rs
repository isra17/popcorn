pub mod elf;

use emulator;
use error::Error;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

pub fn load<P: AsRef<Path>>(path: P) -> Result<Box<emulator::Emulator>, Error> {
    let mut file = try!(File::open(path).map_err(|e| Error::IoError(e)));
    let mut file_magic = [0; 4];
    try!(file.read_exact(&mut file_magic));
    try!(file.seek(SeekFrom::Start(0)));

    let binary_file = try!(if elf::check_magic(&file_magic) {
        elf::load(&mut file)
    } else {
        Err(Error::UnknownFormat)
    });
    Ok(Box::new(binary_file))
}

// Helper functions.
/// Align a memory size.
fn aligned_size(size: usize, page_size: usize) -> usize {
    return (size / page_size + 1) * page_size;
}

/// Align a memory address.
fn aligned_addr(addr: u64, page_size: u64) -> u64 {
    return (addr / page_size) * page_size;
}

#[cfg(test)]
mod tests {
    use error::Error;
    use binary;

    #[test]
    fn test_load() {
        use std::path::PathBuf;
        assert!(matches!(binary::load(PathBuf::from("path/does/not/exists")),
                         Err(Error::IoError(_))));
        assert!(matches!(binary::load(PathBuf::from("./tests/samples/test.txt")),
                         Err(Error::UnknownFormat)));
        binary::load(PathBuf::from("./tests/samples/cat")).expect("Load sample");
    }

    #[test]
    fn test_aligned_size() {
        assert_eq!(binary::aligned_size(1024, 1024), 2048);
        assert_eq!(binary::aligned_size(1023, 1024), 1024);
        assert_eq!(binary::aligned_size(0, 1024), 1024);
        assert_eq!(binary::aligned_size(1025, 1024), 2048);
    }

    #[test]
    fn test_aligned_addr() {
        assert_eq!(binary::aligned_addr(1024, 1024), 1024);
        assert_eq!(binary::aligned_addr(1023, 1024), 0);
        assert_eq!(binary::aligned_addr(0, 1024), 0);
        assert_eq!(binary::aligned_addr(1025, 1024), 1024);
    }
}
