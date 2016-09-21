use arch::Arch;
use error::Error;
use stack;
use std::collections::HashMap;
use unicorn;

#[derive(Debug)]
pub struct MemMap {
    pub addr: u64,
    pub size: u64,
    pub flags: unicorn::unicorn_const::Protection,
    pub name: Option<String>,
}

pub type MemMaps = HashMap<String, MemMap>;

/// Main struct to manage and run emulation.
pub struct Emulator {
    mappings: MemMaps,
    arch: Arch,
    uc: unicorn::Unicorn,
}

impl Emulator {
    /// Create a new emulator instance for a given architecture from
    /// `arch_info`.
    pub fn new(arch: Arch) -> Result<Emulator, Error> {
        let uc = try!(unicorn::Unicorn::new(arch.arch, arch.mode));
        Ok(Emulator {
            mappings: Default::default(),
            arch: arch,
            uc: uc,
        })
    }

    /// Return a reference to engine architecture information.
    pub fn arch(&self) -> &Arch {
        &self.arch
    }

    /// Return a reference to the emulation engine.
    pub fn engine(&self) -> &unicorn::Unicorn {
        &self.uc
    }

    /// Return an helper accessor to the emulator stack.
    pub fn stack(&mut self) -> stack::Stack {
        stack::Stack::new(self)
    }

    // Memory operation.
    /// Create a new memory map in the emulator from `mapping`. It will be
    /// possible to read and write the emulator memory afterward.
    pub fn mem_map(&mut self, mapping: MemMap) -> Result<(), Error> {
        let map_key = match mapping.name {
            Some(ref name) => name.clone(),
            None => format!(".anonymous[0x{:x}]", mapping.addr),
        };

        if self.mappings.contains_key(&map_key) {
            return Err(Error::MapAlreadyExists(mapping));
        }

        try!(self.uc.mem_map(mapping.addr, mapping.size as usize, mapping.flags));
        self.mappings.insert(map_key, mapping);
        Ok(())
    }

    /// Return a reference to the HashMap that contains the mappings
    /// information.
    pub fn mappings(&self) -> &MemMaps {
        &self.mappings
    }

    // Emulation helper.

    /// Call a function at a given address `addr` with the arguments `args`.
    pub fn call_addr(&mut self, _addr: u64, _args: &[u64]) -> Result<u64, Error> {
        Ok(0)
    }
}
