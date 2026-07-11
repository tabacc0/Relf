use crate::raw::elf64::relocation::rela::Elf64Rela;
use crate::model::elf64::section::section::Elf64Section;
pub struct Elf64RelaIter<'a> {
    idx:usize,
    section:&'a Elf64Section<'a>,
}

impl<'a> Elf64RelaIter<'a>{
    pub fn new(section:&'a Elf64Section<'a>) -> Self{
        let idx : usize = 0;
        Self {idx,section}
    }
}

impl<'a> Iterator for Elf64RelaIter<'a> {

    type Item = Elf64Rela;
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
