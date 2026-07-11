use crate::global::error::Error;
use crate::raw::elf64::header::constants::*;
use crate::raw::elf64::program::constants::*;
use crate::raw::elf64::section::constants::*;
use crate::raw::elf64::types::*;

#[derive(Debug)]
#[repr(C)]
pub struct Elf64Ehdr {
    e_ident: [u8; 16],    //file identifier
    e_type: Elf64Half,    //object file type
    e_machine: Elf64Half, //architecture
    e_version: Elf64Word, //ELF version
    e_entry: Elf64Addr,   //entry point if executable, 0 if not
    //file offset of PROGRAM header table,0 if none
    e_phoff: Elf64Off,
    //file offset of SECTION header table,0 if none
    e_shoff: Elf64Off,
    e_flags: Elf64Word,  //processor specific flags
    e_ehsize: Elf64Half, //ELF header size
    //size of entries in PROGRAM header table
    //all entries are the same size
    e_phentsize: Elf64Half,
    //number of entries in program header table
    e_phnum: Elf64Half,
    //size of entries in SECTION header table
    //all entries are the same size
    e_shentsize: Elf64Half,
    //number of entries in section header table
    e_shnum: Elf64Half,
    //index in the SECTION header table of the
    //section associated with a string table
    //the string table has section names
    e_shstrndx: Elf64Half,
}

impl Elf64Ehdr {
    pub fn from_bytes(raw_bytes: &[u8]) -> Result<Self, Error> {
        if raw_bytes.len() < ELF64EHDRSIZE {
            return Err(Error::BufferTooShort);
        }
        let mut e_ident: [u8; 16] = [0; 16];
        e_ident.copy_from_slice(&raw_bytes[0..16]);
        if e_ident[EI_MAG0 as usize] != 0x7f
            || e_ident[EI_MAG1 as usize] != b'E'
            || e_ident[EI_MAG2 as usize] != b'L'
            || e_ident[EI_MAG3 as usize] != b'F'
        {
            return Err(Error::InvalidMagic);
        }

        if !VALID_EI_CLASS.contains(&e_ident[EI_CLASS as usize]) {
            return Err(Error::InvalidElfClass);
        }

        if !VALID_EI_DATA.contains(&e_ident[EI_DATA as usize]) {
            return Err(Error::InvalidElfEndian);
        }
        let endianness: u8 = e_ident[EI_DATA as usize];

        let e_type =
            match Elf64Half::from_bytes(&raw_bytes[16..18], endianness) {
                Ok(value) => value,
                Err(_) => return Err(Error::FieldBuildingError),
            };
        let e_machine =
            match Elf64Half::from_bytes(&raw_bytes[18..20], endianness) {
                Ok(value) => {
                    if !VALID_EM.contains(&value) {
                        return Err(Error::InvalidFieldValue);
                    }
                    value
                }
                Err(_) => {
                    return Err(Error::FieldBuildingError);
                }
            };
        let e_version =
            match Elf64Word::from_bytes(&raw_bytes[20..24], endianness) {
                Ok(value) => {
                    if !VALID_EV.contains(&value) {
                        return Err(Error::InvalidFieldValue);
                    }
                    value
                }
                Err(_) => {
                    return Err(Error::FieldBuildingError);
                }
            };
        let e_entry =
            match Elf64Addr::from_bytes(&raw_bytes[24..32], endianness) {
                Ok(value) => value,
                Err(_) => {
                    return Err(Error::FieldBuildingError);
                }
            };
        let e_phoff =
            match Elf64Off::from_bytes(&raw_bytes[32..40], endianness) {
                Ok(value) => value,
                Err(_) => {
                    return Err(Error::FieldBuildingError);
                }
            };
        let e_shoff =
            match Elf64Off::from_bytes(&raw_bytes[40..48], endianness) {
                Ok(value) => value,
                Err(_) => {
                    return Err(Error::FieldBuildingError);
                }
            };
        let e_flags =
            match Elf64Word::from_bytes(&raw_bytes[48..52], endianness) {
                Ok(value) => value,
                Err(_) => {
                    return Err(Error::FieldBuildingError);
                }
            };
        let e_ehsize =
            match Elf64Half::from_bytes(&raw_bytes[52..54], endianness) {
                Ok(value) => value,
                Err(_) => {
                    return Err(Error::FieldBuildingError);
                }
            };
        let e_phentsize =
            match Elf64Half::from_bytes(&raw_bytes[54..56], endianness) {
                Ok(value) => {
                    if u16::from(value) as usize != ELF64PHDRSIZE {
                        return Err(Error::InvalidPhEntSize);
                    }
                    value
                }
                Err(_) => {
                    return Err(Error::FieldBuildingError);
                }
            };
        let e_phnum =
            match Elf64Half::from_bytes(&raw_bytes[56..58], endianness) {
                Ok(value) => value,
                Err(_) => {
                    return Err(Error::FieldBuildingError);
                }
            };
        let e_shentsize =
            match Elf64Half::from_bytes(&raw_bytes[58..60], endianness) {
                Ok(value) => {
                    if u16::from(value) as usize != ELF64SHDRSIZE {
                        return Err(Error::InvalidShEntSize);
                    }
                    value
                }
                Err(_) => {
                    return Err(Error::FieldBuildingError);
                }
            };
        let e_shnum =
            match Elf64Half::from_bytes(&raw_bytes[60..62], endianness) {
                Ok(value) => value,
                Err(_) => {
                    return Err(Error::FieldBuildingError);
                }
            };
        let e_shstrndx =
            match Elf64Half::from_bytes(&raw_bytes[62..64], endianness) {
                Ok(value) => value,
                Err(_) => {
                    return Err(Error::FieldBuildingError);
                }
            };

        Ok(Self {
            e_ident: e_ident,
            e_type: e_type,
            e_machine: e_machine,
            e_version: e_version,
            e_entry: e_entry,
            e_phoff: e_phoff,
            e_shoff: e_shoff,
            e_flags: e_flags,
            e_ehsize: e_ehsize,
            e_phentsize: e_phentsize,
            e_phnum: e_phnum,
            e_shentsize: e_shentsize,
            e_shnum: e_shnum,
            e_shstrndx: e_shstrndx,
        })
    }

    pub fn e_ident(&self) -> &[u8] {
        &self.e_ident
    }
    pub fn e_type(&self) -> Elf64Half {
        self.e_type
    }
    pub fn e_machine(&self) -> Elf64Half {
        self.e_machine
    }
    pub fn e_version(&self) -> Elf64Word {
        self.e_version
    }
    pub fn e_entry(&self) -> Elf64Addr {
        self.e_entry
    }
    pub fn e_phoff(&self) -> Elf64Off {
        self.e_phoff
    }
    pub fn e_shoff(&self) -> Elf64Off {
        self.e_shoff
    }
    pub fn e_flags(&self) -> Elf64Word {
        self.e_flags
    }
    pub fn e_ehsize(&self) -> Elf64Half {
        self.e_ehsize
    }
    pub fn e_phentsize(&self) -> Elf64Half {
        self.e_phentsize
    }
    pub fn e_phnum(&self) -> Elf64Half {
        self.e_phnum
    }
    pub fn e_shentsize(&self) -> Elf64Half {
        self.e_shentsize
    }
    pub fn e_shnum(&self) -> Elf64Half {
        self.e_shnum
    }
    pub fn e_shstrndx(&self) -> Elf64Half {
        self.e_shstrndx
    }
    pub fn endianness(&self) -> u8 {
        self.e_ident[EI_DATA as usize]
    }
    pub fn section_header_size(&self) -> Elf64Word {
        Elf64Word::from(
            (u16::from(self.e_shentsize) * u16::from(self.e_shnum)) as u32,
        )
    }
    pub fn program_header_size(&self) -> Elf64Word {
        Elf64Word::from(
            (u16::from(self.e_phentsize) * u16::from(self.e_phnum)) as u32,
        )
    }
}
