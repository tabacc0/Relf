use crate::model::elf32::section::section::Elf32Section;
use crate::raw::elf32::symbol::symbol_entry::Elf32Sym;
pub struct Elf32SymIter<'a> {
    idx: usize,
    section: &'a Elf32Section<'a>,
}

impl<'a> Elf32SymIter<'a> {
    pub fn new(section: &'a Elf32Section<'a>) -> Self {
        let idx: usize = 0;
        Self { idx, section }
    }
}

impl<'a> Iterator for Elf32SymIter<'a> {
    type Item = Elf32Sym;
    fn next(&mut self) -> Option<Self::Item> {
        match self.section.sym(self.idx) {
            Ok(sym) => {
                self.idx += 1;
                Some(sym)
            }
            Err(_) => None,
        }
    }
}
