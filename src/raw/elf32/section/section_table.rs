use crate::raw::elf32::error::Error;
use crate::raw::elf32::types::Elf32Half;
use std::cell::OnceCell;
use crate::raw::elf32::section::section::*;


#[derive(Debug)]
#[repr(C)]
pub struct Elf32SectionTable<'a>{
    st : Vec<OnceCell<Elf32Section<'a>>>,
}

impl<'a> Elf32SectionTable<'a>{
    pub fn get_sh(&self,idx:usize) -> 
        Result<&'a OnceCell<Elf32Section>,Error>{
        if idx >= self.st.len()  {
            return Err(Error::IndexOutOfBoundsError);
        }
         Ok(&self.st[idx])
    }
    pub fn new(e_shnum :Elf32Half) -> Self{
        let e_shnum : usize = u16::from(e_shnum) as usize;
        let mut st : Vec<OnceCell<Elf32Section>> =  Vec::new();
        for i in 0..e_shnum {
            st.push(OnceCell::new());
        } 
        Self {st}
    }
}
