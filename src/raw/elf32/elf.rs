use std::fs::File;
use std::io::prelude::*;
use crate::raw::elf32::header::elf32_header::Elf32Ehdr;
pub struct Elf32 {
    pub raw_bytes : Vec<u8>,
    pub header : Elf32Ehdr,
}

impl Elf32 {
    pub fn from_file(path : impl AsRef<std::path::Path>) -> result<Self> {
        let mut elf_file = match File::open(&path) {
            Err(e) => return Err(e),
            Ok(f) => f,
        };
        let mut raw_bytes = String::new():
        match elf_file.read_to_string(&mut raw_bytes){
            Ok() => (),
            Err(e) => return Err(e),
        }
        let mut raw_bytes = raw_bytes.into_bytes();
        let header = 
            Elf32Ehdr::from_bytes(&raw_bytes[0..size_of::<Elf32Ehdr>()]);
        Self {
            raw_bytes,
            header
        }

    }
    //pub fn from_file()
    //fn sections(&self) -> ElfSections{}
}
