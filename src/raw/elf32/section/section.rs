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
    link_section : Option<&'a Elf32Section<'a>>,
    //in case of relocation , sh_info holds the section header 
    //table index to which the relocation applies
    //and in case of a group section this has the symbol entry 
    //index in the link section whose name serves as the group
    //signature
    info_section : Option<&'a Elf32Section<'a>>,
    endianness : u8,
}


impl<'a> Elf32Section<'a>{
    pub fn new(raw_bytes : &'a[u8] ,
        name : &'a[u8],
        header : &'a Elf32Shdr,
        link_section : Option<&'a Elf32Section>,
        info_section : Option<&'a Elf32Section>,
        endianness : u8) -> Self
    {
            Self{raw_bytes,
            name,
            header,
            link_section,
            info_section,
            endianness}
    }
    pub fn raw_bytes(&self) -> &'a[u8] {
        self.raw_bytes
    }
    pub fn name(&self) -> &'a[u8] {
        self.name
    }
    pub fn header(&self) -> &'a Elf32Shdr {
        self.header
    }

    pub fn link_section(&self) -> Option<&'a Elf32Section<'a>> {
        self.link_section
    }

    pub fn info_section(&self) -> Option<&'a Elf32Section<'a>> {
        self.info_section
    }

    pub fn group_symbol(&self) -> Option<Elf32Symbol> {
        if self.is_group_section(){
            return match self.symbol(self.info() as usize){
                Ok(value) => Some(value),
                Err(_) => None,
            };
        }
        None
    }
    pub fn group_name(&self) -> Option<&[u8]> {
        if self.is_group_section(){
            return match self.symbol(self.info() as usize){
                Ok(value) => Some(value.name()),
                Err(_) => None,
            };
        }
        None
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
    pub fn entry_size(&self) -> u32 {
        u32::from(self.header.sh_entsize())
    }
    pub fn entry_count(&self) -> u32 {
        u32::from(self.header.sh_size() / self.header.sh_entsize())
    }


//type-related properties
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
    pub fn is_group_section(&self) -> bool {
        self.header.sh_type() == SHT_GROUP
    }
    pub fn is_note_section(&self) -> bool {
        self.header.sh_type() == SHT_NOTE
    }
    pub fn is_empty(&self) -> bool {
        self.header.sh_type() == SHT_NOBITS
    }
    pub fn is_fini_array(&self) -> bool {
        self.header.sh_type() == SHT_FINI_ARRAY
    }
    pub fn is_init_array(&self) -> bool {
        self.header.sh_type() == SHT_INIT_ARRAY
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

    pub fn to_be_merged(&self) -> bool {
        if u32::from(self.header.sh_flags() & SHF_MERGE) == 0{
            return false
        }
        true
    }

    pub fn has_strings(&self) -> bool {
        if u32::from(self.header.sh_flags() & SHF_STRINGS) == 0{
            return false
        }
        true
    }

    pub fn sh_info_is_sht_index(&self) -> bool {
        if u32::from(self.header.sh_flags() & SHF_INFO_LINK) == 0{
            return false
        }
        true
    }

    pub fn has_ordering_requirement(&self) -> bool {
        if u32::from(self.header.sh_flags() & SHF_LINK_ORDER) == 0{
            return false
        }
        true
    }

    pub fn os_specific(&self) -> bool {
        if u32::from(self.header.sh_flags() & SHF_OS_NONCONFORMING) == 0{
            return false
        }
        true
    }

    pub fn is_group_member(&self) -> bool {
        if u32::from(self.header.sh_flags() & SHF_GROUP) == 0{
            return false
        }
        true
    }

    pub fn is_tls(&self) -> bool {
        if u32::from(self.header.sh_flags() & SHF_TLS) == 0{
            return false
        }
        true
    }


    pub fn is_comdat_group(&self) -> bool {
        if self.is_group_section(){
            let flag_entry = match
                Elf32Word::
                from_bytes(&self.raw_bytes[0..4],self.endianness){
                    Ok(value) => value,
                    //this should never happen
                    //as endianness is validated before here
                    Err(_) => return false,
                };
            if u32::from(flag_entry & GRP_COMDAT) == 0{
                return false
            }
            return true;
        }
        false
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

        let assoc_strtab = match self.link_section{
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
        else if self.is_relatab() {
            header = match self.rela(idx) {
                Ok(value) => Elf32RelocationHeader::Rela(value),
                Err(_) => return Err(Error::RelaFetchingError)
            };
        }
        else {
            return Err(Error::NotRelaTable);
        }
        let symbol_table = match self.link_section{
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

