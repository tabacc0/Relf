use crate::raw::elf32::error::Error;
use crate::raw::elf32::types::*;
use crate::raw::elf32::program::constants::*;


#[derive(Debug)]
#[repr(C)]
pub struct Elf32Phdr {
    p_type : Elf32Word,//type of segment , see above 
    //the offset from the begining of the file to the segment
    //this can be this way bcs section of the same segment are always
    //contiguous in an elf file
    p_offset : Elf32Off,
    //the virtual address of the start of the segment in virtual memory
    p_vaddr : Elf32Addr,
    //in systems that support it , the pysical address of the start of segment
    p_paddr : Elf32Addr,
    //size of the segment on file , may be 0
    p_filesz : Elf32Word,
    //size of the segment in the process memory, may not be less than p_filesz
    p_memsz : Elf32Word,
    //specifies flags relevant to the segment , see above
    p_flags : Elf32Word,
    //segment alignment , a muliple of two or (0 or 1) for no alignment reqs
    p_align : Elf32Word,
}

impl Elf32Phdr {
    pub fn from_bytes(raw_bytes : &[u8;ELF32PHDRSIZE],endianness:u8) 
        -> Result<Self,Error> {

            let p_type = 
        match Elf32Word::from_bytes(&raw_bytes[0..4],endianness){
                Ok(value) => value,
            Err(_) => return Err(Error::FieldBuildingError),
        };

    let p_offset = 
        match Elf32Off::from_bytes(&raw_bytes[4..8],endianness){
            Ok(value) => value,
            Err(_) => return Err(Error::FieldBuildingError),
        };

    let p_vaddr = 
        match Elf32Addr::from_bytes(&raw_bytes[8..12],endianness){
            Ok(value) => value,
            Err(_) => return Err(Error::FieldBuildingError),
        };

    let p_paddr = 
        match Elf32Addr::from_bytes(&raw_bytes[12..16],endianness){
            Ok(value) => value,
            Err(_) => return Err(Error::FieldBuildingError),
        };

    let p_filesz = 
        match Elf32Word::from_bytes(&raw_bytes[16..20],endianness){
            Ok(value) => value,
            Err(_) => return Err(Error::FieldBuildingError),
        };

    let p_memsz = 
        match Elf32Word::from_bytes(&raw_bytes[20..24],endianness){
            Ok(value) => value,
            Err(_) => return Err(Error::FieldBuildingError),
        };

    let p_flags = 
        match Elf32Word::from_bytes(&raw_bytes[24..28],endianness){
            Ok(value) => value,
            Err(_) => return Err(Error::FieldBuildingError),
        };

    let p_align = 
        match Elf32Word::from_bytes(&raw_bytes[28..32],endianness){
            Ok(value) => value,
            Err(_) => return Err(Error::FieldBuildingError),
        };


    Ok(Self {
        p_type , 
        p_offset , 
        p_vaddr , 
        p_paddr,
        p_filesz , 
        p_memsz , 
        p_flags , 
        p_align,
    })
}

pub fn p_type(&self) -> Elf32Word {
    self.p_type
}
pub fn p_offset(&self) -> Elf32Off {
    self.p_offset
}
pub fn p_vaddr(&self) -> Elf32Addr {
    self.p_vaddr
}
pub fn p_paddr(&self) -> Elf32Addr {
    self.p_paddr
}
pub fn p_filesz(&self) -> Elf32Word {
    self.p_filesz
}
pub fn p_memsz(&self) -> Elf32Word {
    self.p_memsz
}
pub fn p_flags(&self) -> Elf32Word {
    self.p_flags
}
pub fn p_align(&self) -> Elf32Word {
    self.p_align
}

}
