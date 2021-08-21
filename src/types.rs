macro_rules! define_verified_type {
    ($name:ident, $type:ty, $($supported:literal),+) => {

        #[derive(Clone, Copy, Eq, PartialEq)]
        pub struct $name($type);

        impl $name {
            pub fn verify(self) -> Result<(), &'static str> {
                match self.0 {
                    $(
                        $supported => Ok(()),
                    )*
                    _ => Err(stringify!($name)),
                }
            }
        }
    };
}

// TODO: Elf magic
//define_verified_type!(ElfMagic, [u8; 4], [0x7f, b'E', b'L', b'F']);

// Only supporting 64-bit ELF
define_verified_type!(ElfClass, u8, 2);

// Only supporting little endian
define_verified_type!(ElfEndianess, u8, 1);

// ELF version
define_verified_type!(ElfVersion, u8, 1);

// OS ABI
define_verified_type!(ElfOsAbi, u8, 0);

// Supporting ET_NONE, ET_EXEC
define_verified_type!(ElfObjectType, u16, 0, 2);

// Only IA-64
define_verified_type!(ElfMachine, u16, 0x32);