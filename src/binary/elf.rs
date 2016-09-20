use elf;
use std;
use unicorn;
use emulator;
use binary::{aligned_addr, aligned_size};
use error::Error;

const ELF_MAGIC: [u8; 4] = [0x7f, 0x45, 0x4c, 0x46];

fn prot_from(flag: elf::types::ProgFlag) -> unicorn::unicorn_const::Protection {
    let flag = flag.0;
    let mut prot = unicorn::unicorn_const::PROT_NONE;
    if flag & elf::types::PF_X.0 != 0 {
        prot |= unicorn::unicorn_const::PROT_EXEC;
    }
    if flag & elf::types::PF_R.0 != 0 {
        prot |= unicorn::unicorn_const::PROT_READ;
    }
    if flag & elf::types::PF_W.0 != 0 {
        prot |= unicorn::unicorn_const::PROT_WRITE;
    }
    return prot;
}

fn arch_from(arch: elf::types::Machine) -> Result<emulator::Arch, Error> {
    use unicorn::unicorn_const::{Arch, Mode};
    match arch {
        // elf::types::EM_386 => Ok(Arch(Arch::X86, Mode::MODE_32)),
        elf::types::EM_X86_64 => Ok(emulator::Arch(Arch::X86, Mode::MODE_64)),
        _ => Err(Error::UnsupportedArch(format!("{:?}", arch))),
    }
}

pub fn check_magic(file_magic: &[u8; 4]) -> bool {
    file_magic == &ELF_MAGIC
}

pub fn load(file: &mut std::fs::File) -> Result<emulator::Emulator, Error> {
    use std::io::{Read, Seek};
    let elf_file = try!(elf::File::open_stream(file)
        .map_err(|e| Error::ParserError(format!("{:?}", e))));
    let mut emu = try!(arch_from(elf_file.ehdr.machine)
        .and_then(|arch| emulator::Emulator::new(arch)));

    // Load segment in emulator.
    let loadable_segments = elf_file.phdrs.iter().filter(|s| s.progtype == elf::types::PT_LOAD);
    for phdr in loadable_segments {
        let page_addr = aligned_addr(phdr.vaddr, 0x1000);
        let offset = (phdr.vaddr - page_addr) as usize;
        let page_size = aligned_size(phdr.memsz as usize + offset, 0x1000);
        let flags: unicorn::unicorn_const::Protection = prot_from(phdr.flags);

        try!(emu.mem_map(emulator::MemMap {
            addr: page_addr,
            size: page_size as u64,
            flags: flags,
            name: None,
        }));

        try!(file.seek(std::io::SeekFrom::Start(phdr.offset)));

        let mut data_buf = Vec::with_capacity(phdr.filesz as usize);
        data_buf.resize(phdr.filesz as usize, 0);
        try!(file.read_exact(data_buf.as_mut_slice()));

        try!(emu.engine().mem_write(phdr.vaddr, data_buf.as_slice()));
    }
    Ok(emu)
}


#[cfg(test)]
mod tests {
    use binary::elf;

    const SAMPLE1_PATH: &'static str = "./tests/samples/cat";

    #[test]
    fn test_load() {
        let mut file = ::std::fs::File::open(SAMPLE1_PATH).expect("Cannot open test file");
        let emu = elf::load(&mut file).expect("Failed to load test file");
        assert_eq!(emu.mappings().len(), 2);
    }
}
