use crate::raw::elf32::types::*;
use crate::global::error::Error;
use crate::raw::elf32::relocation::constants::*;

#[derive(Debug)]
pub struct Elf32Rel {
    //location where the relocation applies
    //it is an offset in object files and
    //a virtual address in executable/shared files
    r_offset: Elf32Off,
    //see above
    r_info: Elf32Word,
    //the addent here is impicit , stored in the location to be modified
}

impl Elf32Rel {
    pub fn from_bytes(
        raw_bytes: &[u8],
        endianness: u8,
    ) -> Result<Self, Error> {
        if raw_bytes.len() < ELF32RELSIZE {
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
        Ok(Self { r_offset, r_info })
    }
    pub fn r_offset(&self) -> Elf32Off {
        self.r_offset
    }
    pub fn r_info(&self) -> Elf32Word {
        self.r_info
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

