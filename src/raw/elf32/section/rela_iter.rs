use crate::raw::elf32::relocation::rela::Elf32Rela;
use crate::model::elf32::section::section::Elf32Section;
pub struct Elf32RelaIter<'a> {
    idx:usize,
    section:&'a Elf32Section<'a>,
}

impl<'a> Elf32RelaIter<'a>{
    pub fn new(section:&'a Elf32Section<'a>) -> Self{
        let idx : usize = 0;
        Self {idx,section}
    }
}

impl<'a> Iterator for Elf32RelaIter<'a> {

    type Item = Elf32Rela;
    fn next(&mut self) -> Option<Self::Item> {
        match self.section.rela(self.idx){
            Ok(rela) => {
                self.idx += 1;
                Some(rela)
            },
            Err(_) => None,
        }
    }
}
