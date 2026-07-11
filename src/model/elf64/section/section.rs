use crate::global::error::Error;
use crate::model::elf64::relocation::relocation::*;
use crate::raw::elf64::relocation::rel::*;
use crate::raw::elf64::relocation::rela::*;
use crate::raw::elf64::relocation::constants::*;
use crate::raw::elf64::section::constants::*;
use crate::raw::elf64::section::section_header::*; 
use crate::model::elf64::symbol::symbol::*;
use crate::raw::elf64::symbol::symbol_entry::*;
use crate::raw::elf64::symbol::constants::*;
use crate::raw::elf64::types::*;
use crate::model::elf64::section::symbol_iter::Elf64SymbolIter;
use crate::model::elf64::section::relocation_iter::Elf64RelocationIter;
use crate::raw::elf64::section::symbol_entry_iter::*;
use crate::raw::elf64::section::rel_iter::*;
use crate::raw::elf64::section::rela_iter::*;

#[derive(Debug)]
pub struct Elf64Section<'a> {
    raw_bytes: &'a [u8],
    name: &'a [u8],
    header: &'a Elf64Shdr,
    //according to sh_link and sh_type (see ./constants.rs)
    //this can be a string table if this section is a symtab
    //or a  symbol table if this section is a relocation table
    link_section: Option<&'a Elf64Section<'a>>,
    //in case of relocation , sh_info holds the section header
    //table index to which the relocations applies
    //and in case of a group section this has the symbol entry
    //index in the link section whose name serves as the group
    //signature
    info_section: Option<&'a Elf64Section<'a>>,
    endianness: u8,
}

impl<'a> Elf64Section<'a> {
    pub fn new(
        raw_bytes: &'a [u8],
        name: &'a [u8],
        header: &'a Elf64Shdr,
        link_section: Option<&'a Elf64Section>,
        info_section: Option<&'a Elf64Section>,
        endianness: u8,
    ) -> Self {
        Self {
            raw_bytes,
            name,
            header,
            link_section,
            info_section,
            endianness,
        }
    }
    pub fn raw_bytes(&self) -> &'a [u8] {
        self.raw_bytes
    }
    pub fn name(&self) -> &'a [u8] {
        self.name
    }
    pub fn header(&self) -> &'a Elf64Shdr {
        self.header
    }

    pub fn link_section(&self) -> Option<&'a Elf64Section<'a>> {
        self.link_section
    }

    pub fn info_section(&self) -> Option<&'a Elf64Section<'a>> {
        self.info_section
    }

    pub fn group_symbol(&'a self) -> Option<Elf64Symbol<'a>> {
        if self.is_group_section() {
            return match self.symbol(self.info() as usize) {
                Ok(value) => Some(value),
                Err(_) => None,
            };
        }
        None
    }
    pub fn group_name(&self) -> Option<&[u8]> {
        if self.is_group_section() {
            return match self.symbol(self.info() as usize) {
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
    pub fn address(&self) -> u64 {
        u64::from(self.header.sh_addr())
    }
    pub fn file_offset(&self) -> u64 {
        u64::from(self.header.sh_offset())
    }
    pub fn size(&self) -> u64 {
        u64::from(self.header.sh_size())
    }
    pub fn link(&self) -> u32 {
        u32::from(self.header.sh_link())
    }
    pub fn info(&self) -> u32 {
        u32::from(self.header.sh_info())
    }
    pub fn alignement(&self) -> u64 {
        u64::from(self.header.sh_addralign())
    }
    pub fn entry_size(&self) -> u64 {
        u64::from(self.header.sh_entsize())
    }
    pub fn entry_count(&self) -> usize {
        u64::from(self.header.sh_size() / self.header.sh_entsize())
            as usize
    }

    //type-related properties
    pub fn is_symtab(&self) -> bool {
        self.header.sh_type() == SHT_SYMTAB
            || self.header.sh_type() == SHT_DYNSYM
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
        if u64::from(self.header.sh_flags() & SHF_WRITE) == 0 {
            return false;
        }
        true
    }
    pub fn is_allocated(&self) -> bool {
        if u64::from(self.header.sh_flags() & SHF_ALLOC) == 0 {
            return false;
        }
        true
    }
    pub fn has_exec_instr(&self) -> bool {
        if u64::from(self.header.sh_flags() & SHF_EXECINSTR) == 0 {
            return false;
        }
        true
    }

    pub fn to_be_merged(&self) -> bool {
        if u64::from(self.header.sh_flags() & SHF_MERGE) == 0 {
            return false;
        }
        true
    }

    pub fn has_strings(&self) -> bool {
        if u64::from(self.header.sh_flags() & SHF_STRINGS) == 0 {
            return false;
        }
        true
    }

    pub fn sh_info_is_sht_index(&self) -> bool {
        if u64::from(self.header.sh_flags() & SHF_INFO_LINK) == 0 {
            return false;
        }
        true
    }

    pub fn has_ordering_requirement(&self) -> bool {
        if u64::from(self.header.sh_flags() & SHF_LINK_ORDER) == 0 {
            return false;
        }
        true
    }

    pub fn os_specific(&self) -> bool {
        if u64::from(self.header.sh_flags() & SHF_OS_NONCONFORMING) == 0 {
            return false;
        }
        true
    }

    pub fn is_group_member(&self) -> bool {
        if u64::from(self.header.sh_flags() & SHF_GROUP) == 0 {
            return false;
        }
        true
    }

    pub fn is_tls(&self) -> bool {
        if u64::from(self.header.sh_flags() & SHF_TLS) == 0 {
            return false;
        }
        true
    }

    pub fn is_comdat_group(&self) -> bool {
        if self.is_group_section() {
            if self.raw_bytes.len() < 4 {
                return false;
            }
            let flag_entry = match Elf64Word::from_bytes(
                &self.raw_bytes[0..4],
                self.endianness,
            ) {
                Ok(value) => value,
                //this should never happen
                //as endianness is validated before here
                Err(_) => return false,
            };
            if u32::from(flag_entry & GRP_COMDAT) == 0 {
                return false;
            }
            return true;
        }
        false
    }

    fn calc_symbol_offset(&self, idx: usize) -> Result<Elf64Off, Error> {
        let symbol_entry_size = ELF64SYMSIZE;
        let entries_number =
            u64::from(self.header.sh_size()) as usize / symbol_entry_size;

        if idx >= entries_number as usize {
            return Err(Error::IndexOutOfBoundsError);
        }

        Ok(Elf64Off::from((idx * symbol_entry_size) as u64))
    }
    pub fn sym(&self, idx: usize) -> Result<Elf64Sym, Error> {
        if !self.is_symtab() {
            return Err(Error::NotSymbolTable);
        }
        let symbol_offset = match self.calc_symbol_offset(idx) {
            Ok(value) => u64::from(value) as usize,
            Err(_) => return Err(Error::CalcOffsetError),
        };
        let symbol_bytes: &[u8] = &self.raw_bytes[symbol_offset..];
        let header =
            match Elf64Sym::from_bytes(symbol_bytes, self.endianness) {
                Ok(value) => value,
                Err(_) => return Err(Error::SymbolConstructionError),
            };
        Ok(header)
    }
    pub fn symbol(&'a self, idx: usize) -> Result<Elf64Symbol<'a>, Error> {
        let header = match self.sym(idx) {
            Ok(value) => value,
            Err(_) => return Err(Error::SymFetchingError),
        };

        let assoc_strtab = match self.link_section {
            Some(value) => value,
            None => return Err(Error::NoAssociatedSectionError),
        };
        let name_idx = u32::from(header.st_name()) as usize;
        let name = match assoc_strtab.str(name_idx) {
            Ok(value) => value,
            Err(_) => return Err(Error::SymbolNameFetchingError),
        };
        let symbol = Elf64Symbol::new(name, header);
        Ok(symbol)
    }

    pub fn symbol_by_name(
        &'a self,
        name: &[u8],
    ) -> Result<Option<Elf64Symbol<'a>>, Error> {

        for idx in 0..self.entry_count(){
            let symbol = match self.symbol(idx) {
                Ok(value) => value,
                Err(_) => return Err(Error::SymbolFetchingError),
            };

            if name == symbol.name() {
                let symbol = match self.symbol(idx){
                    Ok(value) => value,
                    Err(_) => return Err(Error::SymbolFetchingError),
                };

                return Ok(Some(symbol));
            }

        }
        return Ok(None);
    }

    fn calc_rel_offset(&self, idx: usize) -> Result<Elf64Off, Error> {
        let entries_number =
            u64::from(self.header.sh_size()) as usize / ELF64RELSIZE;

        if idx >= entries_number as usize {
            return Err(Error::IndexOutOfBoundsError);
        }

        Ok(Elf64Off::from((idx * ELF64RELSIZE) as u64))
    }

    pub fn rel(&self, idx: usize) -> Result<Elf64Rel, Error> {
        if !self.is_reltab() {
            return Err(Error::NotRelTable);
        }
        let rel_offset = match self.calc_rel_offset(idx) {
            Ok(value) => u64::from(value) as usize,
            Err(_) => return Err(Error::CalcOffsetError),
        };
        let rel_bytes: &[u8] = &self.raw_bytes[rel_offset..];
        let rel = match Elf64Rel::from_bytes(rel_bytes, self.endianness) {
            Ok(value) => value,
            Err(_) => return Err(Error::RelConstructionError),
        };
        Ok(rel)
    }
    fn calc_rela_offset(&self, idx: usize) -> Result<Elf64Off, Error> {
        let rela_entry_size = ELF64RELASIZE;
        let entries_number =
            u64::from(self.header.sh_size()) as usize / rela_entry_size;

        if idx >= entries_number as usize {
            return Err(Error::IndexOutOfBoundsError);
        }

        Ok(Elf64Off::from((idx * rela_entry_size) as u64))
    }

    pub fn rela(&self, idx: usize) -> Result<Elf64Rela, Error> {
        if !self.is_relatab() {
            return Err(Error::NotRelaTable); //lol def not
        }
        let rela_offset = match self.calc_rela_offset(idx) {
            Ok(value) => u64::from(value) as usize,
            Err(_) => return Err(Error::CalcOffsetError),
        };
        let rela_bytes: &[u8] = &self.raw_bytes[rela_offset..];
        let rela = match Elf64Rela::from_bytes(rela_bytes, self.endianness)
        {
            Ok(value) => value,
            Err(_) => return Err(Error::RelaConstructionError),
        };
        Ok(rela)
    }
    pub fn relocation(
        &'a self,
        idx: usize,
    ) -> Result<Elf64Relocation<'a>, Error> {
        let header: Elf64RelocationHeader;
        if self.is_reltab() {
            header = match self.rel(idx) {
                Ok(value) => Elf64RelocationHeader::Rel(value),
                Err(_) => return Err(Error::RelFetchingError),
            };
        } else if self.is_relatab() {
            header = match self.rela(idx) {
                Ok(value) => Elf64RelocationHeader::Rela(value),
                Err(_) => return Err(Error::RelaFetchingError),
            };
        } else {
            return Err(Error::NotRelaTable);
        }
        let symbol_table = match self.link_section {
            Some(value) => value,
            None => return Err(Error::NoAssociatedSectionError),
        };
        let relocation_target = match self.info_section {
            Some(value) => value,
            None => return Err(Error::NoAssociatedSectionError),
        };
        let relocation =
            Elf64Relocation::new(header, symbol_table,relocation_target);
        Ok(relocation)
    }
    pub fn str(&self, idx: usize) -> Result<&'a [u8], Error> {
        if !self.is_strtab() {
            return Err(Error::NotStringTable);
        }
        //number of bytes in the section
        let table_size = u64::from(self.size()) as usize;
        if idx >= table_size {
            return Err(Error::IndexOutOfBoundsError);
        }
        if self.raw_bytes.len() < table_size {
            return Err(Error::BufferTooShort);
        }
        let mut upper_bound = idx;
        while upper_bound < table_size {
            if self.raw_bytes[upper_bound] == 0 {
                break;
            } else {
                upper_bound += 1;
            }
        }

        let string = &self.raw_bytes()[idx..upper_bound];
        Ok(string)
    }

    pub fn sym_iter(&'a self) -> Elf64SymIter<'a> {
        Elf64SymIter::new(&self)
    }
    pub fn rel_iter(&'a self) -> Elf64RelIter<'a> {
        Elf64RelIter::new(&self)
    }
    pub fn rela_iter(&'a self) -> Elf64RelaIter<'a> {
        Elf64RelaIter::new(&self)
    }

    pub fn symbol_iter(&'a self) -> Elf64SymbolIter<'a> {
        Elf64SymbolIter::new(&self)
    }
    pub fn relocation_iter(&'a self) -> Elf64RelocationIter<'a> {
        Elf64RelocationIter::new(&self)
    }
}
