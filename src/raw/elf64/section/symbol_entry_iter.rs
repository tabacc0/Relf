use crate::raw::elf64::symbol::symbol_entry::Elf64Sym;
use crate::model::elf64::section::section::Elf64Section;
pub struct Elf64SymIter<'a> {
    idx:usize,
    section:&'a Elf64Section<'a>,
}

impl<'a> Elf64SymIter<'a>{
    pub fn new(section:&'a Elf64Section<'a>) -> Self{
        let idx : usize = 0;
        Self {idx,section}
    }
}

impl<'a> Iterator for Elf64SymIter<'a> {

    type Item = Elf64Sym;
    fn next(&mut self) -> Option<Self::Item> {
        match self.section.sym(self.idx){
            Ok(sym) => {
                self.idx += 1;
                Some(sym)
            },
            Err(_) => None,
        }
    }
}
