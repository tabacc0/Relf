use crate::raw::elf32::error::Error;
use crate::raw::elf32::types::*;

#[derive(Debug)]
pub struct Elf32Sym {
    st_name: Elf32Word, //index into the object's string table
    //depending on the symbol this may be an address or absolute value(above)
    st_value: Elf32Addr,
    //size of the symbol (ie. some data structure), 0 if no size or unknown
    st_size: Elf32Word,
    //specifies the symbol types and binding attribute , details above
    st_info: u8,
    //some bits specify symbol visibility other are 0(unused) ,see above.
    st_other: u8,
    //index of the section header of the section to which
    //the symbol entry relates
    //or an index with special meaning see section_header_table.rs for details
    st_shndx: Elf32Half,
}

pub const ELF32SYMSIZE: usize = 16;
impl Elf32Sym {
    pub fn from_bytes(
        raw_bytes: &[u8],
        endianness: u8,
    ) -> Result<Self, Error> {
        if raw_bytes.len() < ELF32SYMSIZE {
            return Err(Error::BufferTooShort);
        }
        let st_name =
            match Elf32Word::from_bytes(&raw_bytes[0..4], endianness) {
                Ok(value) => value,
                Err(_) => return Err(Error::FieldBuildingError),
            };

        let st_value =
            match Elf32Addr::from_bytes(&raw_bytes[4..8], endianness) {
                Ok(value) => value,
                Err(_) => return Err(Error::FieldBuildingError),
            };

        let st_size =
            match Elf32Word::from_bytes(&raw_bytes[8..12], endianness) {
                Ok(value) => value,
                Err(_) => return Err(Error::FieldBuildingError),
            };

        let st_info = raw_bytes[12];
        let st_other = raw_bytes[13];

        let st_shndx =
            match Elf32Half::from_bytes(&raw_bytes[14..16], endianness) {
                Ok(value) => value,
                Err(_) => return Err(Error::FieldBuildingError),
            };

        Ok(Self {
            st_name,
            st_value,
            st_size,
            st_info,
            st_other,
            st_shndx,
        })
    }

    pub fn st_name(&self) -> Elf32Word {
        self.st_name
    }
    pub fn st_value(&self) -> Elf32Addr {
        self.st_value
    }
    pub fn st_size(&self) -> Elf32Word {
        self.st_size
    }
    pub fn st_info(&self) -> u8 {
        self.st_info
    }
    pub fn st_other(&self) -> u8 {
        self.st_other
    }
    pub fn st_shndx(&self) -> Elf32Half {
        self.st_shndx
    }

    pub fn st_type(&self) -> u8 {
        let st_info = self.st_info;
        let st_type = st_info & 0xf;
        st_type
    }
    pub fn st_bind(&self) -> u8 {
        let st_info = self.st_info;
        let st_bind = st_info >> 4;
        st_bind
    }
    pub fn st_visibility(&self) -> u8 {
        let st_other = self.st_other;
        let st_visibility = st_other & 0x3;
        st_visibility
    }
}
