use crate::model::elf32::section::section::*;
use crate::model::elf32::symbol::symbol::Elf32Symbol;
pub struct Elf32SymbolIter<'a> {
    idx: usize,
    section: &'a Elf32Section<'a>,
}

impl<'a> Elf32SymbolIter<'a> {
    pub fn new(section: &'a Elf32Section<'a>) -> Self {
        let idx: usize = 0;
        Self { idx, section }
    }
}

impl<'a> Iterator for Elf32SymbolIter<'a> {
    type Item = Elf32Symbol<'a>;
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
