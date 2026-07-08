use crate::raw::elf32::error::Error;
use std::cell::OnceCell;
use crate::raw::elf32::types::Elf32Half;

//aside from normal indexing special indexes of this table
//are used in the ELF spec to signify other things
//these special indexes typically don't reference the table
//but are only signifiers for structures that hold them :
//here they are along whith their signification:
//
//undefined entry , missing or irrelevant , in case of symbols
//the definition will be resolved from another object file
pub const SHN_UNDEF : Elf32Half = Elf32Half{value:0} ;
//lower bound of reserved indexes
pub const SHN_LORESERVE : Elf32Half = Elf32Half{value:0xff00} ;
//lower bound of indexes reserved for processor specific sematics
pub const SHN_LOPROC : Elf32Half = Elf32Half{value:0xff00} ;
//higer bound of indexes reserved for processor specific sematics
pub const SHN_HIPROC : Elf32Half = Elf32Half{value:0xff1f} ;
//lower bound of indexes reserved for operating system specific sematics
pub const SHN_LOOS : Elf32Half = Elf32Half{value:0xff20} ;
//higer bound of indexes reserved for operating system specific sematics
pub const SHN_HIOS : Elf32Half = Elf32Half{value:0xff3f} ;


//specifies abosolute values for corresponding references
//symbols relative to this have absolute values and need not
//be relocated
pub const SHN_ABS : Elf32Half = Elf32Half{value:0xfff1} ;
//reserved for common symbols that have not been allocated,
//the symbol in this case defines alignment requirements
//FORTAN common and C external vars relate to this, 
pub const SHN_COMMON : Elf32Half = Elf32Half{value:0xfff2} ;
//escape index , means that the index is too large to fit in the
//structure (Efl32Half here) and is to be found somewhere else
//were exactly is specific to the structure
pub const SHN_XINDEX : Elf32Half = Elf32Half{value:0xffff} ;
//higher bound of reserved indexes
pub const SHN_HIRESERVE : Elf32Half = Elf32Half{value:0xffff} ;
use super::section_header::Elf32Shdr;
#[derive(Debug)]
#[repr(C)]
pub struct Elf32Sht{
    sht : Vec<OnceCell<Elf32Shdr>>,
    e_shnum : usize,
}

impl Elf32Sht{
    pub fn get_sh(&self,idx:usize) -> Result<&OnceCell<Elf32Shdr>,Error>{
        if idx > (self.e_shnum - 1) {
            return Err(Error::IndexOutOfBoundsError);
        }
         Ok(&self.sht[idx])
    }
    pub fn new(e_shnum :Elf32Half) -> Self{
        let e_shnum : usize = u16::from(e_shnum) as usize;
        let mut sht : Vec<OnceCell<Elf32Shdr>> =  Vec::new();
        for i in 0..e_shnum {
            sht.push(OnceCell::new());
        } 
        Self {sht,e_shnum}
    }
}
