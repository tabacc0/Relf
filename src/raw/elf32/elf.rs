use std::fs;
use crate::raw::elf32::types::*;
use crate::raw::elf32::header::Elf32Ehdr;
use crate::raw::elf32::section::section_header_table::*;
use crate::raw::elf32::program::program_header_table::*;
use crate::raw::elf32::section::section_header::*;
//use crate::raw::elf32::section::program_header::*;
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
    
    fn calc_section_header_offset(&self,idx:usize) -> Result<Elf32Off,Error>{
        let e_shnum = self.header.e_shnum()?;
        if idx >  u16::from(e_shnum) as usize{
            return Err(Error::IndexOutOfBoundsError);
        }
        let sh_offset = self.header.section_header_offset()?.value as usize;
        let sh_entsize = self.header.e_shentsize()?.value as usize;
        Ok(Elf32Off::from((sh_offset + idx*sh_entsize) as u32))
    }

    pub fn section_header(&self,idx:usize) -> Result<&Elf32Shdr,Error>{
        let sh_cell = self.sht.get_sh(idx);
        if  sh_cell.get().is_none() {
            let sh_offset = match self.calc_section_header_offset(idx){
                Ok(off) => off.value as usize,
                Err(_) => return Err(Error::CalcOffsetError),
                };


            let section_header_bytes : &[u8;size_of::<Elf32Shdr>()] = 
                &self.raw_bytes[sh_offset..sh_offset+size_of::<Elf32Shdr>()]
                .try_into().unwrap();

            let section_header = match 
                Elf32Shdr::from_bytes(section_header_bytes){
                    Ok(value) => value,
                    Err(_) => 
                        return Err(Error::SectionHeaderConstructionError),
                };
            sh_cell.set(section_header);
            return Ok(sh_cell.get().unwrap());
        }
        else {
            return Ok(sh_cell.get().unwrap());
        }
    }

}
