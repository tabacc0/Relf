use crate::raw::elf32::types::*;
use crate::raw::elf32::error::*;
use crate::raw::elf32::symbol::symbol_entry::*;


#[derive(Debug)]
pub struct Elf32Symbol<'a> {
    name : &'a[u8],
    header : Elf32Sym,
}


impl<'a> Elf32Symbol<'a>{
    pub fn new( name : &'a[u8], header : Elf32Sym)
        -> Self{
            Self{name,header}
    }
    pub fn name(&self) -> &'a[u8] {
        self.name
    }
    pub fn header(&self) -> &Elf32Sym {
        &self.header
    }
}
