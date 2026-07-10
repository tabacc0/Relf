use crate::raw::elf32::types::*;
use crate::global::error::Error;
use crate::raw::elf32::relocation::constants::*;

#[derive(Debug)]
pub struct Elf32Rela {
    //same
    r_offset: Elf32Off,
    //same
    r_info: Elf32Word,
    //a constant value used to compute relocated value
    r_addend: Elf32Sword,
}

impl Elf32Rela {
    pub fn from_bytes(
        raw_bytes: &[u8],
        endianness: u8,
    ) -> Result<Self, Error> {
        if raw_bytes.len() < ELF32RELASIZE {
            return Err(Error::BufferTooShort);
        }
        let r_offset =
            match Elf32Off::from_bytes(&raw_bytes[0..4], endianness) {
                Ok(value) => value,
                Err(_) => return Err(Error::FieldBuildingError),
            };
        let r_info =
            match Elf32Word::from_bytes(&raw_bytes[4..8], endianness) {
                Ok(value) => value,
                Err(_) => return Err(Error::FieldBuildingError),
            };
        let r_addend =
            match Elf32Sword::from_bytes(&raw_bytes[8..12], endianness) {
                Ok(value) => value,
                Err(_) => return Err(Error::FieldBuildingError),
            };
        Ok(Self {
            r_offset,
            r_info,
            r_addend,
        })
    }

    pub fn r_offset(&self) -> Elf32Off {
        self.r_offset
    }
    pub fn r_info(&self) -> Elf32Word {
        self.r_info
    }
    pub fn r_addend(&self) -> Elf32Sword {
        self.r_addend
    }
    pub fn relocated_symbol_idx(&self) -> usize {
        let r_info = u32::from(self.r_info);
        let r_sym = (r_info >> 8) as usize;
        r_sym
    }
    pub fn rel_type(&self) -> usize {
        let r_info = u32::from(self.r_info);
        let r_type = (r_info & 0xff) as usize;
        r_type
    }
}
