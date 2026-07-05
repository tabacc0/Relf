use crate::raw::elf32::program::program_header::Elf32Phdr;
use crate::raw::elf32::types::Elf32Half;
#[derive(Debug)]
#[repr(C)]
pub struct Elf32Pht {
    pht :  Vec<Option<Elf32Phdr>>,
}
impl Elf32Pht {
    pub fn get_phdr(&self,idx : usize) -> &Option<Elf32Phdr>{
        &self.pht[idx]
    }
    pub fn set_ph(&mut self,ph:Option<Elf32Phdr>,idx:usize) {
            self.pht[idx] = ph;
    }
    pub fn new(e_phnum :&Elf32Half) -> Self{
        let e_phnum : usize = u16::from(e_phnum) as usize;
        let mut pht :  Vec<Option<Elf32Phdr>> =  Vec::new();
        for i in 0..e_phnum {
            pht.push(None);
        } 
        Self {pht}
    }
}
