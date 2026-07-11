use crate::raw::elf64::relocation::rel::Elf64Rel;
use crate::model::elf64::section::section::Elf64Section;
pub struct Elf64RelIter<'a> {
    idx:usize,
    section:&'a Elf64Section<'a>,
}

impl<'a> Elf64RelIter<'a>{
    pub fn new(section:&'a Elf64Section<'a>) -> Self{
        let idx : usize = 0;
        Self {idx,section}
    }
}

impl<'a> Iterator for Elf64RelIter<'a> {

    type Item = Elf64Rel;
    fn next(&mut self) -> Option<Self::Item> {
        match self.section.rel(self.idx){
            Ok(rel) => {
                self.idx += 1;
                Some(rel)
            },
            Err(_) => None,
        }
    }
}
