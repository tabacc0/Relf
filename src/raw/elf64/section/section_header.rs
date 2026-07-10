use crate::global::error::Error;
use crate::raw::elf64::section::constants::*;
use crate::raw::elf64::types::*;

#[derive(Debug)]
#[repr(C)]
pub struct Elf64Shdr {
    sh_name: Elf64Word, //index into section header string table
    sh_type: Elf64Word, //contents and semantics, see ./constants.rs file
    sh_flags: Elf64Xword, //misc ,see ./constants.rs file
    sh_addr: Elf64Addr, //runtime address in the process, or 0
    sh_offset: Elf64Off, //file offset to the section
    sh_size: Elf64Xword, //size of the section
    sh_link: Elf64Word, //see ./constants.rs file
    sh_info: Elf64Word, //section info see ./constants.rs file
    sh_addralign: Elf64Xword, //alignment , or 0
    //size of entries in section that are tables or 0
    sh_entsize: Elf64Xword,
}

impl Elf64Shdr {
    pub fn from_bytes(
        raw_bytes: &[u8],
        endianness: u8,
    ) -> Result<Self, Error> {
        if raw_bytes.len() < ELF64SHDRSIZE {
            return Err(Error::BufferTooShort);
        }
        let sh_name =
            match Elf64Word::from_bytes(&raw_bytes[0..4], endianness) {
                Ok(value) => value,
                Err(_) => return Err(Error::FieldBuildingError),
            };

        let sh_type =
            match Elf64Word::from_bytes(&raw_bytes[4..8], endianness) {
                Ok(value) => value,
                Err(_) => return Err(Error::FieldBuildingError),
            };

        let sh_flags =
            match Elf64Xword::from_bytes(&raw_bytes[8..16], endianness) {
                Ok(value) => value,
                Err(_) => return Err(Error::FieldBuildingError),
            };

        let sh_addr =
            match Elf64Addr::from_bytes(&raw_bytes[16..24], endianness) {
                Ok(value) => value,
                Err(_) => return Err(Error::FieldBuildingError),
            };

        let sh_offset =
            match Elf64Off::from_bytes(&raw_bytes[24..32], endianness) {
                Ok(value) => value,
                Err(_) => return Err(Error::FieldBuildingError),
            };

        let sh_size =
            match Elf64Xword::from_bytes(&raw_bytes[32..40], endianness) {
                Ok(value) => value,
                Err(_) => return Err(Error::FieldBuildingError),
            };

        let sh_link =
            match Elf64Word::from_bytes(&raw_bytes[40..44], endianness) {
                Ok(value) => value,
                Err(_) => return Err(Error::FieldBuildingError),
            };

        let sh_info =
            match Elf64Word::from_bytes(&raw_bytes[44..48], endianness) {
                Ok(value) => value,
                Err(_) => return Err(Error::FieldBuildingError),
            };

        let sh_addralign =
            match Elf64Xword::from_bytes(&raw_bytes[48..56], endianness) {
                Ok(value) => value,
                Err(_) => return Err(Error::FieldBuildingError),
            };
        let sh_entsize =
            match Elf64Xword::from_bytes(&raw_bytes[56..64], endianness) {
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
    pub fn sh_name(&self) -> Elf64Word {
        self.sh_name
    }
    pub fn sh_type(&self) -> Elf64Word {
        self.sh_type
    }
    pub fn sh_flags(&self) -> Elf64Xword {
        self.sh_flags
    }
    pub fn sh_addr(&self) -> Elf64Addr {
        self.sh_addr
    }
    pub fn sh_offset(&self) -> Elf64Off {
        self.sh_offset
    }
    pub fn sh_size(&self) -> Elf64Xword {
        self.sh_size
    }
    pub fn sh_link(&self) -> Elf64Word {
        self.sh_link
    }
    pub fn sh_info(&self) -> Elf64Word {
        self.sh_info
    }
    pub fn sh_addralign(&self) -> Elf64Xword {
        self.sh_addralign
    }
    pub fn sh_entsize(&self) -> Elf64Xword {
        self.sh_entsize
    }
}
