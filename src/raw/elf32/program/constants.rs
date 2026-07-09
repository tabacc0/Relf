use crate::raw::elf32::types::*;

pub const ELF32PHDRSIZE: usize = 32;

//values of p_type and their interpretation
//all these values are optional by the spec
//a section program header may contain only fields that are relevant to it
//
//
//unused/invalid array member , to be ignored
pub const PT_NULL: Elf32Word = Elf32Word { value: 0 };
//loadable segment, the bytes from the file are put into memory and padded with 0s if p_memsz is greater than p_filesz
pub const PT_LOAD: Elf32Word = Elf32Word { value: 1 };
//entry with dynamic linking info
pub const PT_DYNAMIC: Elf32Word = Elf32Word { value: 2 };
//entry has size and location of the path of the interpreter (null term str)
pub const PT_INTERP: Elf32Word = Elf32Word { value: 3 };
//entry has auxiliary info
pub const PT_NOTE: Elf32Word = Elf32Word { value: 4 };
//unused
pub const PT_SHLIB: Elf32Word = Elf32Word { value: 5 };
//entry holding the location and size of the ph table itself
pub const PT_PHDR: Elf32Word = Elf32Word { value: 6 };
//this entry implements TLS(thread local storage)
//see : https://refspecs.linuxbase.org/elf/gabi4+/ch5.pheader.html
pub const PT_TLS: Elf32Word = Elf32Word { value: 7 };
//these two define the bounds of the range reserved for operating system-specific semantics
pub const PT_LOOS: Elf32Word = Elf32Word { value: 0x60000000 };
pub const PT_HIOS: Elf32Word = Elf32Word { value: 0x6fffffff };
//these two define the bounds of the range reserved for cpu-specific semantics
pub const PT_LOPROC: Elf32Word = Elf32Word { value: 0x70000000 };
pub const PT_HIPROC: Elf32Word = Elf32Word { value: 0x7fffffff };

pub const VALID_PT: &[Elf32Word] = &[
    PT_NULL, PT_LOAD, PT_DYNAMIC, PT_INTERP, PT_NOTE, PT_SHLIB, PT_PHDR,
    PT_TLS,
];

//masks of p_flags and their interpretation
//
//
pub const PF_X: Elf32Word = Elf32Word { value: 1 }; //this segment is executable
pub const PF_W: Elf32Word = Elf32Word { value: 2 }; //writable at runtime
pub const PF_R: Elf32Word = Elf32Word { value: 4 }; //readable at runtime
//mask bit for os-specific semantics
pub const PF_MASKOS: Elf32Word = Elf32Word { value: 0x0ff00000 };
//mask bit for cpu-specific semantics
pub const PF_MASKPROC: Elf32Word = Elf32Word { value: 0xf0000000 };
