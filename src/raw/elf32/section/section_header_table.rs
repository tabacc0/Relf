use std::cell::OnceCell;
use crate::raw::elf32::types::Elf32Half;

//aside from normal indexing special indexes of this table
//are used in the ELF spec to signify other things
//these special indexes typically don't reference the table
//but are only signifiers for structures that hold them :
//here they are along whith their signification:
//
//undefined entry , missing or irrelevant 
const SHN_UNDEF : Elf32Half = Elf32Half{value:0} ;
//lower bound of reserved indexes
const SHN_LORESERVE : Elf32Half = Elf32Half{value:0xff00} ;
//lower bound of indexes reserved for processor specific sematics
const SHN_LOPROC : Elf32Half = Elf32Half{value:0xff00} ;
//higer bound of indexes reserved for processor specific sematics
const SHN_HIPROC : Elf32Half = Elf32Half{value:0xff1f} ;
//specifies abosolute values for corresponding references
//symbols relative to this have absolute values and need not
//be relocated
const SHN_ABS : Elf32Half = Elf32Half{value:0xfff1} ;
//reserved for common symbols , FORTAN common and C external vars
//relate to this
const SHN_COMMON : Elf32Half = Elf32Half{value:0xfff2} ;
//higher bound of reserved indexes
const SHN_HIRESERVE : Elf32Half = Elf32Half{value:0xffff} ;
use super::section_header::Elf32Shdr;
#[derive(Debug)]
#[repr(C)]
pub struct Elf32Sht{
    pub sht : Vec<OnceCell<Elf32Shdr>>,
}

impl Elf32Sht{
    pub fn get_sh(&self,idx:usize) -> &OnceCell<Elf32Shdr>{
         &self.sht[idx]
    }
    pub fn new(e_shnum :&Elf32Half) -> Self{
        let e_shnum : usize = u16::from(e_shnum) as usize;
        let mut sht : Vec<OnceCell<Elf32Shdr>> =  Vec::new();
        for i in 0..e_shnum {
            sht.push(OnceCell::new());
        } 
        Self {sht}
    }
}
