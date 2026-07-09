use crate::raw::elf32::relocation::*;
use crate::raw::elf32::types::*;
use crate::raw::elf32::error::*;
use crate::raw::elf32::symbol::symbol_entry::*;
use crate::raw::elf32::symbol::symbol::*;
use crate::raw::elf32::section::section_header::*;
use crate::raw::elf32::section::constants::*;
 use crate::raw::elf32::relocation::relocation::*;

#[derive(Debug)]
pub struct Elf32Section<'a> {
    raw_bytes : &'a[u8],
    name : &'a[u8],
    header : &'a Elf32Shdr,
    //according to sh_link and sh_type (see ./constants.rs)
    //this can be a string table if this section is a symtab
    //or a  symbol table if this section is a relocation table
    associated_section : Option<&'a Elf32Section<'a>>,
    endianness : u8,
}


impl<'a> Elf32Section<'a>{
    pub fn new(raw_bytes : &'a[u8] ,
        name : &'a[u8],
        header : &'a Elf32Shdr,
        associated_section : Option<&'a Elf32Section>,
        endianness : u8) -> Self
    {
            Self{raw_bytes,name,header,associated_section,endianness}
    }
    pub fn raw_bytes(&self) -> &'a[u8] {
        self.raw_bytes
    }
    pub fn name(&self) -> &'a[u8] {
        self.name
    }
    pub fn header(&self) -> &'a Elf32Shdr {
        &self.header
    }



    pub fn name_idx(&self) -> u32 {
        u32::from(self.header.sh_name())
    }
    pub fn section_type(&self) -> u32 {
        u32::from(self.header.sh_type())
    }
    pub fn address(&self) -> u32 {
        u32::from(self.header.sh_addr())
    }
    pub fn file_offset(&self) -> u32 {
        u32::from(self.header.sh_offset())
    }
    pub fn size(&self) -> u32 {
        u32::from(self.header.sh_size())
    }
    pub fn link(&self) -> u32 {
        u32::from(self.header.sh_link())
    }
    pub fn info(&self) -> u32 {
        u32::from(self.header.sh_info())
    }
    pub fn alignement(&self) -> u32 {
        u32::from(self.header.sh_addralign())
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




    pub fn associated_section(&self) -> Option<&'a Elf32Section<'a>> {
        self.associated_section
    }


    fn calc_symbol_offset(&self,idx:usize) -> 
        Result<Elf32Off,Error>
    {
        let symbol_entry_size = ELF32SYMSIZE;
        let entries_number =
            u32::from(self.header.sh_size()) as usize /
            symbol_entry_size;

        if idx >= entries_number as usize{
            return Err(Error::IndexOutOfBoundsError);
        }

        Ok(Elf32Off::from((idx*symbol_entry_size) as u32))
    }
    pub fn sym(&self,idx:usize) -> Result<Elf32Sym,Error> {
        if !self.is_symtab() {
            return Err(Error::NotSymbolTable)
        }
        let symbol_offset = match self.calc_symbol_offset(idx){
            Ok(value) => u32::from(value) as usize,
            Err(_) => return Err(Error::CalcOffsetError),
        };
        let symbol_bytes : &[u8] = 
            &self.raw_bytes [symbol_offset..];
        let header = 
            match Elf32Sym::from_bytes(symbol_bytes,self.endianness) {
                Ok(value) => value,
                Err(_) => return Err(Error::SymbolConstructionError)
            };
        Ok(header)
    }
    pub fn symbol(&self,idx:usize) -> Result<Elf32Symbol,Error> {
        let header = match self.sym(idx) {
            Ok(value) => value,
            Err(_) => return Err(Error::SymFetchingError)
        };

        let assoc_strtab = match self.associated_section{
            Some(value) => value,
            None => return Err(Error::NoAssociatedSectionError),
        };
        let name_idx = u32::from(header.st_name()) as usize;
        let name = match assoc_strtab.str(name_idx){
            Ok(value) => value,
            Err(_) => return Err(Error::SymbolNameFetchingError),
        };
        let symbol = Elf32Symbol::new(name,header);
        Ok(symbol)
    }

    fn calc_rel_offset(&self,idx:usize) -> 
        Result<Elf32Off,Error>
    {
        let entries_number =
            u32::from(self.header.sh_size()) as usize /
            ELF32RELSIZE;

        if idx >= entries_number as usize{
            return Err(Error::IndexOutOfBoundsError);
        }

        Ok(Elf32Off::from((idx*ELF32RELSIZE) as u32))
    }

    pub fn rel(&self,idx:usize) -> Result<Elf32Rel,Error> {
        if !self.is_reltab() {
            return Err(Error::NotRelTable)
        }
        let rel_offset = match self.calc_rel_offset(idx){
            Ok(value) => u32::from(value) as usize,
            Err(_) => return Err(Error::CalcOffsetError),
        };
        let rel_bytes : &[u8] = 
            &self.raw_bytes [rel_offset..];
        let rel = match Elf32Rel::from_bytes(rel_bytes,self.endianness) {
            Ok(value) => value,
            Err(_) => return Err(Error::RelConstructionError)
        };
        Ok(rel)
    }
    fn calc_rela_offset(&self,idx:usize) -> 
        Result<Elf32Off,Error>
    {
        let rela_entry_size = ELF32RELASIZE;
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
        let rela_bytes : &[u8] = 
            &self.raw_bytes [rela_offset..];
        let rela = 
            match Elf32Rela::from_bytes(rela_bytes,self.endianness) {
                Ok(value) => value,
                Err(_) => return Err(Error::RelaConstructionError)
            };
        Ok(rela)
    }
    pub fn relocation(&self,idx:usize) -> Result<Elf32Relocation,Error> {
        let mut header : Elf32RelocationHeader ;
        if self.is_reltab(){
            header = match self.rel(idx) {
                Ok(value) => Elf32RelocationHeader::Rel(value),
                Err(_) => return Err(Error::RelFetchingError)
            };
        }
        else {
            header = match self.rela(idx) {
                Ok(value) => Elf32RelocationHeader::Rela(value),
                Err(_) => return Err(Error::RelaFetchingError)
            };
        }
        let symbol_table = match self.associated_section{
            Some(value) => value,
            None => return Err(Error::NoAssociatedSectionError),
        };
        let relocation = Elf32Relocation::new(header,symbol_table);
        Ok(relocation)

    }
    pub fn str(&self,idx:usize) -> Result<&'a[u8],Error> {
        if !self.is_strtab() {
            return Err(Error::NotStringTable)
        }
        //number of bytes in the section
        let table_size = u32::from(self.size()) as usize;
        if idx >= table_size {
            return Err(Error::IndexOutOfBoundsError);
        }
        let mut upper_bound = idx;
        while upper_bound < table_size {
            if self.raw_bytes[upper_bound] == 0 {
                break;
            }
            else {
                upper_bound += 1;
            }
        }

        let string = &self.raw_bytes()[idx..upper_bound];
        Ok(string)

    }
}

