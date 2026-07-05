use std::fs;
use crate::raw::elf32::header::Elf32Ehdr;
use crate::raw::elf32::section::section_header_table::*;
use crate::raw::elf32::program::program_header_table::*;
use crate::raw::elf32::error::*;

#[derive(Debug)]
pub struct Elf32 {
    pub raw_bytes : Vec<u8>,
    pub header : Elf32Ehdr,
    pub sht : Elf32Sht ,
    pub pht : Elf32Pht ,
}

impl Elf32 {
    pub fn from_file(path : impl AsRef<std::path::Path>) -> Result<Self,Error>
    {
        use crate::raw::elf32::error::*;
        let raw_bytes : Vec<u8> = match fs::read(&path) {
            Err(e) =>{
                println!("error : {}",e);
                return Err(Error::FileReadError);
            }
            Ok(f) => f,
        };
        let header_bytes : &[u8;size_of::<Elf32Ehdr>()] =
            &raw_bytes[0..size_of::<Elf32Ehdr>()].try_into().unwrap();

        let header = match Elf32Ehdr::from_bytes(header_bytes) {
            Ok(val) => val,
            Err(e) => {
                println!("error : {}",e);
                return Err(Error::HeaderParsingError);
            }
        };
        let sht = Elf32Sht::new(header.e_shnum()?);
        let pht = Elf32Pht::new(header.e_phnum()?);
        Ok(Self { raw_bytes, header,sht ,pht})

    }
    
    //pub fn section(&self,idx:usize) -> &Elf32Shdr

}
