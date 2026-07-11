use crate::model::elf64::section::section::*;
use crate::model::elf64::symbol::symbol::Elf64Symbol;
pub struct Elf64SymbolIter<'a> {
    idx: usize,
    section: &'a Elf64Section<'a>,
}

impl<'a> Elf64SymbolIter<'a> {
    pub fn new(section: &'a Elf64Section<'a>) -> Self {
        let idx: usize = 0;
        Self { idx, section }
    }
}

impl<'a> Iterator for Elf64SymbolIter<'a> {
    type Item = Elf64Symbol<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.section.symbol(self.idx) {
            Ok(sym) => {
                self.idx += 1;
                Some(sym)
            }
            Err(_) => None,
        }
    }
}
