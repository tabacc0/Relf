use crate::raw::elf64::elf::Elf64;
use crate::raw::elf64::program::program_header::Elf64Phdr;
pub struct Elf64PhdrIter<'a> {
    idx:usize,
    elf:&'a Elf64<'a>,
}

impl<'a> Elf64PhdrIter<'a>{
    pub fn new(elf:&'a Elf64<'a>) -> Self{
        let idx : usize = 0;
        Self {idx,elf}
    }
}

impl<'a> Iterator for Elf64PhdrIter<'a> {

    type Item = &'a Elf64Phdr;
    fn next(&mut self) -> Option<Self::Item> {
        match self.elf.phdr(self.idx){
            Ok(phdr) => {
                self.idx += 1;
                Some(phdr)
            },
            Err(_) => None,
        }
    }
}
