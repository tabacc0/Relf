use crate::raw::elf32::types::*;
use crate::raw::elf32::error::Error;
use crate::raw::elf32::section::constants::*;

#[derive(Debug)]
#[repr(C)]
pub struct Elf32Shdr {
    sh_name : Elf32Word,//index into section header string table
    sh_type : Elf32Word,//contents and semantics, see above
    sh_flag : Elf32Word,//misc ,see above
    sh_addr : Elf32Addr,//runtime address in the process, or 0
    sh_offset : Elf32Off,//file offset to the section
    sh_size : Elf32Word,//size of the section
    sh_link : Elf32Word,//see above
    sh_info : Elf32Word,//section info see above
    sh_addralign : Elf32Word,//alignment , or 0
    //size of entries in section that are tables or 0
    sh_entsize : Elf32Word,
}


impl Elf32Shdr {
    pub fn from_bytes(raw_bytes:&[u8;size_of::<Self>()],endianness : u8)
        -> Result<Self,Error>
    {
        let sh_name = 
            match Elf32Word::from_bytes(&raw_bytes[0..4],endianness){
                Ok(value) => value,
                Err(e) => return Err(Error::FieldBuildingError),
            };

        let sh_type = 
            match Elf32Word::from_bytes(&raw_bytes[4..8],endianness){
                Ok(value) =>  value ,
            Err(e) => return Err(Error::FieldBuildingError),
        };

        let sh_flag = 
            match Elf32Word::from_bytes(&raw_bytes[8..12],endianness){
                Ok(value) => value,
                Err(e) => return Err(Error::FieldBuildingError),
            };

        let sh_addr = 
            match Elf32Addr::from_bytes(&raw_bytes[12..16],endianness){
                Ok(value) => value ,
                Err(e) => return Err(Error::FieldBuildingError),
            };

        let sh_offset = 
            match Elf32Off::from_bytes(&raw_bytes[16..20],endianness){
                Ok(value) => value ,
                Err(e) => return Err(Error::FieldBuildingError),
            };

        let sh_size = 
            match Elf32Word::from_bytes(&raw_bytes[20..24],endianness){
                Ok(value) => value ,
                Err(e) => return Err(Error::FieldBuildingError),
            };

        let sh_link = 
            match Elf32Word::from_bytes(&raw_bytes[24..28],endianness){
                Ok(value) => value ,
                Err(e) => return Err(Error::FieldBuildingError),
            };

        let sh_info = 
            match Elf32Word::from_bytes(&raw_bytes[28..32],endianness){
                Ok(value) => value ,
                Err(e) => return Err(Error::FieldBuildingError),
            };

        let sh_addralign = 
            match Elf32Word::from_bytes(&raw_bytes[32..36],endianness){
                Ok(value) =>  value ,
            Err(e) => return Err(Error::FieldBuildingError),
        };
        let sh_entsize = 
            match Elf32Word::from_bytes(&raw_bytes[36..40],endianness){
                Ok(value) => value,
                Err(e) => return Err(Error::FieldBuildingError),
            };

        Ok(Self {
            sh_name ,
            sh_type , 
            sh_flag , 
            sh_addr , 
            sh_offset,
            sh_size , 
            sh_link , 
            sh_info , 
            sh_addralign,
            sh_entsize,
        })
    }

    //reference access to fields , use only if needed
    pub fn section_name_idx(&self) -> &Elf32Word{
        &self.sh_name
    }
    pub fn section_type(&self) -> &Elf32Word{
        &self.sh_type
    }
    pub fn section_flag(&self) -> &Elf32Word{
        &self.sh_flag
    }
    pub fn section_addr(&self) -> &Elf32Addr{
        &self.sh_addr
    }
    pub fn section_offset(&self) -> &Elf32Off{
        &self.sh_offset
    }
    pub fn section_size(&self) -> &Elf32Word{
        &self.sh_size
    }
    pub fn section_link(&self) -> &Elf32Word{
        &self.sh_link
    }
    pub fn section_info(&self) -> &Elf32Word{
        &self.sh_info
    }
    pub fn section_alignment(&self) -> &Elf32Word{
        &self.sh_addralign
    }
    pub fn section_entry_size(&self) -> Result<&Elf32Word,Error>{
        if u32::from(&self.sh_entsize) == 0 {
            return Err(Error::NotTable);
        }
        Ok(&self.sh_entsize)
    }
}

