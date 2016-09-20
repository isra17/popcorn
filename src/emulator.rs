use error::Error;
use std::collections::HashMap;
use unicorn;

#[derive(Clone)]
pub struct Arch(pub unicorn::unicorn_const::Arch, pub unicorn::unicorn_const::Mode);

#[derive(Debug)]
pub struct MemMap {
    pub addr: u64,
    pub size: u64,
    pub flags: unicorn::unicorn_const::Protection,
    pub name: Option<String>,
}

pub type MemMaps = HashMap<String, MemMap>;

pub struct Emulator {
    mappings: MemMaps,
    arch: Arch,
    uc: unicorn::Unicorn,
}

impl Emulator {
    pub fn new(arch_info: Arch) -> Result<Emulator, Error> {
        let Arch(arch, mode) = arch_info;
        Ok(Emulator {
            mappings: Default::default(),
            arch: arch_info,
            uc: try!(unicorn::Unicorn::new(arch, mode)),
        })
    }

    pub fn mem_map(&mut self, mapping: MemMap) -> Result<(), Error> {
        let map_key = match mapping.name {
            Some(ref name) => name.clone(),
            None => format!(".anonymous[0x{:x}]", mapping.addr),
        };
        try!(self.uc.mem_map(mapping.addr, mapping.size as usize, mapping.flags));
        self.mappings.insert(map_key, mapping);
        Ok(())
    }

    pub fn mem_write(&mut self, addr: u64, data: &[u8]) -> Result<(), Error> {
        try!(self.uc.mem_write(addr, data));
        Ok(())
    }

    pub fn mappings(&self) -> &MemMaps {
        &self.mappings
    }

    pub fn arch(&self) -> Arch {
        self.arch.clone()
    }

    pub fn call_addr(&mut self, _addr: u64, _args: &[u64]) -> Result<u64, Error> {
        Ok(0)
    }
}
