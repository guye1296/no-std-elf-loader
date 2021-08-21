use super::types as types;

#[repr(C)]
#[repr(packed)]
pub struct ElfHeader64 {
    magic: [u8; 4],
    class: types::ElfClass,
    endianess: types::ElfEndianess,
    version: types::ElfVersion,
    abi_type: types::ElfOsAbi,
    abi_version: u8,
    _padding0: [u8; 7],
    object_file_type: types::ElfObjectType,
    machine: types::ElfMachine,

    // Address-width dependant part of the ELF header
    e_version: u32,
    e_entry: u64,
    e_phoff: u64,
    e_shoff: u64,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16
}

impl ElfHeader64 {
    pub fn verify(self) -> Result<(), &'static str> {
        self.class.verify()?;
        self.endianess.verify()?;
        self.version.verify()?;
        self.abi_type.verify()?;
        self.object_file_type.verify()?;
        self.machine.verify()?;

        Ok(())
    }
}