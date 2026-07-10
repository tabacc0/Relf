use crate::raw::elf64::types::*;
use crate::global::error::Error;
use crate::raw::elf64::relocation::constants::*;

#[derive(Debug)]
pub struct Elf64Rel {
    //location where the relocation applies
    //it is an offset in object files and
    //a virtual address in executable/shared files
    r_offset: Elf64Off,
    //see above
    r_info: Elf64Xword,
    //the addent here is impicit , stored in the location to be modified
}

impl Elf64Rel {
    pub fn from_bytes(
        raw_bytes: &[u8],
        endianness: u8,
    ) -> Result<Self, Error> {
        if raw_bytes.len() < ELF64RELSIZE {
            return Err(Error::BufferTooShort);
        }
        let r_offset =
            match Elf64Off::from_bytes(&raw_bytes[0..8], endianness) {
                Ok(value) => value,
                Err(_) => return Err(Error::FieldBuildingError),
            };
        let r_info =
            match Elf64Xword::from_bytes(&raw_bytes[8..16], endianness) {
                Ok(value) => value,
                Err(_) => return Err(Error::FieldBuildingError),
            };
        Ok(Self { r_offset, r_info })
    }
    pub fn r_offset(&self) -> Elf64Off {
        self.r_offset
    }
    pub fn r_info(&self) -> Elf64Xword {
        self.r_info
    }
    pub fn relocated_symbol_idx(&self) -> usize {
        let r_info = u64::from(self.r_info);
        let r_sym = (r_info >> 8) as usize;
        r_sym
    }
    pub fn rel_type(&self) -> usize {
        let r_info = u64::from(self.r_info);
        let r_type = (r_info & 0xff) as usize;
        r_type
    }
}

