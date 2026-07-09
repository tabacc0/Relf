use crate::raw::elf32::error::Error;
use crate::raw::elf32::section::constants::*;
use crate::raw::elf32::types::*;

#[derive(Debug)]
#[repr(C)]
pub struct Elf32Shdr {
    sh_name: Elf32Word, //index into section header string table
    sh_type: Elf32Word, //contents and semantics, see ./constants.rs file
    sh_flags: Elf32Word, //misc ,see ./constants.rs file
    sh_addr: Elf32Addr, //runtime address in the process, or 0
    sh_offset: Elf32Off, //file offset to the section
    sh_size: Elf32Word, //size of the section
    sh_link: Elf32Word, //see ./constants.rs file
    sh_info: Elf32Word, //section info see ./constants.rs file
    sh_addralign: Elf32Word, //alignment , or 0
    //size of entries in section that are tables or 0
    sh_entsize: Elf32Word,
}

impl Elf32Shdr {
    pub fn from_bytes(
        raw_bytes: &[u8],
        endianness: u8,
    ) -> Result<Self, Error> {
        if raw_bytes.len() < ELF32SHDRSIZE {
            return Err(Error::BufferTooShort);
        }
        let sh_name =
            match Elf32Word::from_bytes(&raw_bytes[0..4], endianness) {
                Ok(value) => value,
                Err(_) => return Err(Error::FieldBuildingError),
            };

        let sh_type =
            match Elf32Word::from_bytes(&raw_bytes[4..8], endianness) {
                Ok(value) => value,
                Err(_) => return Err(Error::FieldBuildingError),
            };

        let sh_flags =
            match Elf32Word::from_bytes(&raw_bytes[8..12], endianness) {
                Ok(value) => value,
                Err(_) => return Err(Error::FieldBuildingError),
            };

        let sh_addr =
            match Elf32Addr::from_bytes(&raw_bytes[12..16], endianness) {
                Ok(value) => value,
                Err(_) => return Err(Error::FieldBuildingError),
            };

        let sh_offset =
            match Elf32Off::from_bytes(&raw_bytes[16..20], endianness) {
                Ok(value) => value,
                Err(_) => return Err(Error::FieldBuildingError),
            };

        let sh_size =
            match Elf32Word::from_bytes(&raw_bytes[20..24], endianness) {
                Ok(value) => value,
                Err(_) => return Err(Error::FieldBuildingError),
            };

        let sh_link =
            match Elf32Word::from_bytes(&raw_bytes[24..28], endianness) {
                Ok(value) => value,
                Err(_) => return Err(Error::FieldBuildingError),
            };

        let sh_info =
            match Elf32Word::from_bytes(&raw_bytes[28..32], endianness) {
                Ok(value) => value,
                Err(_) => return Err(Error::FieldBuildingError),
            };

        let sh_addralign =
            match Elf32Word::from_bytes(&raw_bytes[32..36], endianness) {
                Ok(value) => value,
                Err(_) => return Err(Error::FieldBuildingError),
            };
        let sh_entsize =
            match Elf32Word::from_bytes(&raw_bytes[36..40], endianness) {
                Ok(value) => value,
                Err(_) => return Err(Error::FieldBuildingError),
            };

        Ok(Self {
            sh_name,
            sh_type,
            sh_flags,
            sh_addr,
            sh_offset,
            sh_size,
            sh_link,
            sh_info,
            sh_addralign,
            sh_entsize,
        })
    }

    //reference access to fields , use only if needed
    pub fn sh_name(&self) -> Elf32Word {
        self.sh_name
    }
    pub fn sh_type(&self) -> Elf32Word {
        self.sh_type
    }
    pub fn sh_flags(&self) -> Elf32Word {
        self.sh_flags
    }
    pub fn sh_addr(&self) -> Elf32Addr {
        self.sh_addr
    }
    pub fn sh_offset(&self) -> Elf32Off {
        self.sh_offset
    }
    pub fn sh_size(&self) -> Elf32Word {
        self.sh_size
    }
    pub fn sh_link(&self) -> Elf32Word {
        self.sh_link
    }
    pub fn sh_info(&self) -> Elf32Word {
        self.sh_info
    }
    pub fn sh_addralign(&self) -> Elf32Word {
        self.sh_addralign
    }
    pub fn sh_entsize(&self) -> Elf32Word {
        self.sh_entsize
    }
}
