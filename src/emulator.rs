use unicorn;

pub type Error = unicorn::Error;

#[derive(Clone)]
pub struct Arch(pub unicorn::unicorn_const::Arch, pub unicorn::unicorn_const::Mode);

pub struct MemMap {
    pub addr: u64,
    pub size: u64,
    pub flags: unicorn::unicorn_const::Protection,
    pub name: String,
}

pub struct Emulator {
    mapping: Vec<MemMap>,
    arch: Arch,
    uc: unicorn::Unicorn,
}

impl Emulator {
    pub fn new(arch_info: Arch) -> Result<Emulator, Error> {
        let Arch(arch, mode) = arch_info;
        Ok(Emulator {
            mapping: Default::default(),
            arch: arch_info,
            uc: try!(unicorn::Unicorn::new(arch, mode)),
        })
    }

    pub fn mem_map(&mut self, mapping: MemMap) -> Result<(), Error> {
        try!(self.uc.mem_map(mapping.addr, mapping.size as usize, mapping.flags));
        self.mapping.push(mapping);
        Ok(())
    }

    pub fn mem_write(&mut self, addr: u64, data: &[u8]) -> Result<(), Error> {
        Ok(())
    }

    pub fn arch(&self) -> Arch {
        self.arch.clone()
    }
}
