use crate::raw::elf32::relocation::*;
use crate::raw::elf32::types::*;
use crate::raw::elf32::error::*;
use crate::raw::elf32::symbol::symbol_entry::*;
use crate::raw::elf32::section::section_header::*;
use crate::raw::elf32::section::constants::*;

#[derive(Debug)]
pub struct Elf32Section<'a> {
    raw_bytes : &'a[u8],
    header : &'a Elf32Shdr,
    endianness : u8,
}

impl<'a> Elf32Section<'a>{
    pub fn new(raw_bytes : &'a[u8] , header : &'a Elf32Shdr,endianness : u8)
        -> Self{
        Self{raw_bytes,header,endianness}
    }
    pub fn raw_bytes(&self) -> &'a[u8] {
        self.raw_bytes
    }
    pub fn header(&self) -> &'a Elf32Shdr {
        &self.header
    }
    pub fn is_symtab(&self) -> bool {
        self.header.sh_type() == SHT_SYMTAB ||
            self.header.sh_type() == SHT_DYNSYM
    }
    pub fn is_strtab(&self) -> bool {
        self.header.sh_type() == SHT_STRTAB
    }
    pub fn is_reltab(&self) -> bool {
        self.header.sh_type() == SHT_REL
    }
    pub fn is_relatab(&self) -> bool {
        self.header.sh_type() == SHT_RELA
    }
    pub fn is_writable(&self) -> bool {
        if u32::from(self.header.sh_flags() & SHF_WRITE) == 0{
            return false
        }
        true
    }
    pub fn is_allocated(&self) -> bool {
        if u32::from(self.header.sh_flags() & SHF_ALLOC) == 0{
            return false
        }
        true
    }
    pub fn has_exec_instr(&self) -> bool {
        if u32::from(self.header.sh_flags() & SHF_EXECINSTR) == 0{
            return false
        }
        true
    }

    fn calc_symbol_offset(&self,idx:usize) -> 
        Result<Elf32Off,Error>
    {
        let symbol_entry_size = size_of::<Elf32Sym>();
        let entries_number =
            u32::from(self.header.sh_size()) as usize /
            symbol_entry_size;

        if idx >= entries_number as usize{
            return Err(Error::IndexOutOfBoundsError);
        }

        Ok(Elf32Off::from((idx*symbol_entry_size) as u32))
    }
    pub fn symbol(&self,idx:usize) -> Result<Elf32Sym,Error> {
        if !self.is_symtab() {
            return Err(Error::NotSymbolTable)
        }
        let symbol_offset = match self.calc_symbol_offset(idx){
            Ok(value) => u32::from(value) as usize,
            Err(_) => return Err(Error::CalcOffsetError),
        };
        let symbol_bytes : &[u8;size_of::<Elf32Sym>()] = 
            self.raw_bytes
            [symbol_offset..symbol_offset+size_of::<Elf32Sym>()]
            .try_into().unwrap();
        let symbol = 
            match Elf32Sym::from_bytes(symbol_bytes,self.endianness) {
                Ok(value) => Ok(value),
                Err(_) => Err(Error::SymbolConstructionError)
            };
        symbol
    }

    fn calc_rel_offset(&self,idx:usize) -> 
        Result<Elf32Off,Error>
    {
        let rel_entry_size = size_of::<Elf32Rel>();
        let entries_number =
            u32::from(self.header.sh_size()) as usize /
            rel_entry_size;

        if idx >= entries_number as usize{
            return Err(Error::IndexOutOfBoundsError);
        }

        Ok(Elf32Off::from((idx*rel_entry_size) as u32))
    }

    pub fn rel(&self,idx:usize) -> Result<Elf32Rel,Error> {
        if !self.is_reltab() {
            return Err(Error::NotRelTable)
        }
        let rel_offset = match self.calc_rel_offset(idx){
            Ok(value) => u32::from(value) as usize,
            Err(_) => return Err(Error::CalcOffsetError),
        };
        let rel_bytes : &[u8;size_of::<Elf32Rel>()] = 
            self.raw_bytes
            [rel_offset..rel_offset+size_of::<Elf32Rel>()]
            .try_into().unwrap();
        let rel = match Elf32Rel::from_bytes(rel_bytes,self.endianness) {
            Ok(value) => Ok(value),
            Err(_) => Err(Error::RelConstructionError)
        };
        rel
    }
    fn calc_rela_offset(&self,idx:usize) -> 
        Result<Elf32Off,Error>
    {
        let rela_entry_size = size_of::<Elf32Rela>();
        let entries_number =
            u32::from(self.header.sh_size()) as usize /
            rela_entry_size;

        if idx >= entries_number as usize{
            return Err(Error::IndexOutOfBoundsError);
        }

        Ok(Elf32Off::from((idx*rela_entry_size) as u32))
    }

    pub fn rela(&self,idx:usize) -> Result<Elf32Rela,Error> {
        if !self.is_relatab() {
            return Err(Error::NotRelaTable)//lol def not
        }
        let rela_offset = match self.calc_rela_offset(idx){
            Ok(value) => u32::from(value) as usize,
            Err(_) => return Err(Error::CalcOffsetError),
        };
        let rela_bytes : &[u8;size_of::<Elf32Rela>()] = 
            self.raw_bytes
            [rela_offset..rela_offset+size_of::<Elf32Rela>()]
            .try_into().unwrap();
        let rela = 
            match Elf32Rela::from_bytes(rela_bytes,self.endianness) {
                Ok(value) => Ok(value),
                Err(_) => Err(Error::RelaConstructionError)
            };
        rela
    }
}

