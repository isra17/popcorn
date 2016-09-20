#[macro_use]
extern crate bitflags;
extern crate elf;
#[macro_use]
extern crate matches;
extern crate unicorn;

pub mod binary;
pub mod emulator;
pub mod error;
pub mod popcorn_engine;

pub use emulator::Emulator;
pub use error::Error;

#[cfg(test)]
mod tests {
    use popcorn_engine::PopcornEngine;
    use std::path::PathBuf;

    #[test]
    fn happy_path_test() {
        // Load and parse a binary file. The loader detect the environment and
        // architecture targeted by the binary.
        let emulator = PopcornEngine::new(PathBuf::from("./tests/samples/cat"))
            .expect("Failed to load binary");

        // Setup a simulated environment to mock I/O done by the program.
        emulator.setup_env();

        // Run the binary file.
        let result = emulator.run(&["./tests/samples/foobar.txt"]).expect("Failed to run");

        // The program should have print the foobar.txt file content.
        assert_eq!(result.stdout(), "This is a text file.");
    }
}
