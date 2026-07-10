use crate::global::error::Error;
use crate::raw::elf64::header::header::Elf64Ehdr;
use crate::raw::elf64::header::constants::*;
use crate::raw::elf64::program::program_header::*;
use crate::raw::elf64::program::program_header_table::*;
use crate::model::elf64::program::segment::*;
use crate::raw::elf64::section::constants::*;
use crate::model::elf64::section::section::*;
use crate::raw::elf64::section::section_header::*;
use crate::raw::elf64::section::section_header_table::*;
use crate::model::elf64::section::section_table::*;
use crate::raw::elf64::types::*;
use crate::model::elf64::symbol::symbol::Elf64Symbol;

#[derive(Debug)]
pub struct Elf64<'a> {
    raw_bytes: &'a [u8],
    header: Elf64Ehdr,
    sht: Elf64Sht,
    pht: Elf64Pht,
    //table of section abstraction
    st: Elf64SectionTable<'a>,
}

impl<'a> Elf64<'a> {
    pub fn from_bytes(raw_bytes: &'a [u8]) -> Result<Self, Error> {
        if raw_bytes.len() < ELF64EHDRSIZE {
            return Err(Error::BufferTooShort); }
        let header = match Elf64Ehdr::from_bytes(raw_bytes) {
            Ok(val) => val,
            Err(_) => return Err(Error::HeaderParsingError),
        };
        let sht = Elf64Sht::new(header.e_shnum());
        let pht = Elf64Pht::new(header.e_phnum());
        let st = Elf64SectionTable::new(header.e_shnum());
        Ok(Self {
            raw_bytes,
            header,
            sht,
            pht,
            st,
        })
    }
    pub fn header(&self) -> &Elf64Ehdr {
        &self.header
    }
    pub fn endianness(&self) -> u8 {
        self.header.endianness()
    }
    pub fn sht_entry_count(&self) -> usize {
        u16::from(self.header.e_shnum()) as usize
    }


    fn calc_shdr_offset(
        &self,
        idx: usize,
    ) -> Result<Elf64Off, Error> {
        let e_shnum = self.header.e_shnum();
        if idx >= u16::from(e_shnum) as usize {
            return Err(Error::IndexOutOfBoundsError);
        }
        //offset of the section header table
        let sht_offset = self.header.e_shoff().value as usize;
        //lenght of an entry
        let sh_entsize = self.header.e_shentsize().value as usize;
        //offset + product
        Ok(Elf64Off::from((sht_offset + idx * sh_entsize) as u64))
    }

    pub fn shdr(&self, idx: usize) -> Result<&Elf64Shdr, Error> {
        let sh_cell = match self.sht.get_sh(idx) {
            Ok(value) => value,
            Err(_) => return Err(Error::SectionHeaderRetrievalError),
        };
        if sh_cell.get().is_none() {
            let sh_offset = match self.calc_shdr_offset(idx) {
                Ok(off) => off.value as usize,
                Err(_) => return Err(Error::CalcOffsetError),
            };

            let shdr_bytes = &self.raw_bytes[sh_offset..];

            let shdr = match Elf64Shdr::from_bytes(
                shdr_bytes,
                self.endianness(),
            ) {
                Ok(value) => value,
                Err(_) => {
                    return Err(Error::SectionHeaderConstructionError);
                }
            };
            match sh_cell.set(shdr) {
                Ok(_) => (),
                Err(_) => return Err(Error::OnceCellFailureError),
            };
            let sh = match sh_cell.get() {
                Some(value) => value,
                None => return Err(Error::OnceCellFailureError),
            };
            return Ok(sh);
        } else {
            let sh = match sh_cell.get() {
                Some(value) => value,
                None => return Err(Error::OnceCellFailureError),
            };
            return Ok(sh);
        }
    }

    pub fn get_section_name(&'a self,idx : usize) -> Result<&'a [u8],Error>{
        let name: &[u8];
        let header: &Elf64Shdr = match self.shdr(idx) {
            Ok(value) => value,
            Err(_) => {
                return Err(Error::SectionHeaderRetrievalError);
            }
        };
        let name_idx = u32::from(header.sh_name()) as usize;
        let sh_size = u64::from(header.sh_size()) as usize;
        let sh_offset = u64::from(header.sh_offset()) as usize;

        let shstrndx = u16::from(self.header.e_shstrndx()) as usize;


        //special treatment for the string table
        //that has the sections names
        //bootstraping its name
        if idx == shstrndx {
            if self.raw_bytes.len() < sh_offset+sh_size {
                return Err(Error::BufferTooShort);
            } 
            let raw_bytes = &self.raw_bytes[sh_offset..sh_offset+sh_size];
            let table_size = sh_size;
            if name_idx >= table_size {
                return Err(Error::IndexOutOfBoundsError);
            }
            let mut upper_bound = name_idx;
            while upper_bound < table_size {
                if raw_bytes[upper_bound] == 0 {
                    break;
                } else {
                    upper_bound += 1;
                }
            }
            name = &raw_bytes[name_idx..upper_bound];
        } else {
            let strsh_section = match self.section(shstrndx) {
                Ok(value) => value,
                Err(_) => {
                    return Err(Error::SectionbuildingError);
                }
            };
            name = match strsh_section.str(name_idx) {
                Ok(value) => value,
                Err(_) => {
                    return Err(Error::SectionNameFetchingError);
                }
            };
        }
        Ok(name)
    } 

    pub fn section(
        &'a self,
        idx: usize,
    ) -> Result<&'a Elf64Section<'a>, Error> {
        let section_cell = match self.st.get_sh(idx) {
            Ok(value) => value,
            Err(_) => return Err(Error::SectionRetrievalError),
        };
        if section_cell.get().is_none() {
            let header: &Elf64Shdr = match self.shdr(idx) {
                Ok(value) => value,
                Err(_) => {
                    return Err(Error::SectionHeaderRetrievalError);
                }
            };
            let sh_offset = u64::from(header.sh_offset()) as usize;
            let sh_size = u64::from(header.sh_size()) as usize;
            let sh_type = header.sh_type();
            let raw_bytes: &[u8] =
                &self.raw_bytes[sh_offset..sh_offset + sh_size];
            let name: &[u8];
            let mut link_section: Option<&Elf64Section> = None;
            let mut info_section: Option<&Elf64Section> = None;


            name = match self.get_section_name(idx){
                Ok(value) => value,
                Err(_) => return Err(Error::SectionNameFetchingError)
            };
            if sh_type == SHT_REL
                || sh_type == SHT_RELA
                || sh_type == SHT_DYNSYM
                || sh_type == SHT_SYMTAB
                || sh_type == SHT_GROUP
                || sh_type == SHT_DYNAMIC
                || sh_type == SHT_HASH
            {
                let sh_link = u32::from(header.sh_link()) as usize;
                link_section = match self.section(sh_link) {
                    Ok(value) => Some(value),
                    Err(_) => {
                        return Err(Error::SectionbuildingError);
                    }
                };
            }
            if sh_type == SHT_REL
                || sh_type == SHT_RELA
                || sh_type == SHT_GROUP
            {
                let sh_info = u32::from(header.sh_info()) as usize;
                info_section = match self.section(sh_info) {
                    Ok(value) => Some(value),
                    Err(_) => {
                        return Err(Error::SectionbuildingError);
                    }
                };
            }

            let section = Elf64Section::new(
                raw_bytes,
                name,
                header,
                link_section,
                info_section,
                self.endianness(),
            );
            match section_cell.set(section) {
                Ok(_) => (),
                Err(_) => return Err(Error::OnceCellFailureError),
            };

            let section = match section_cell.get() {
                Some(value) => value,
                None => return Err(Error::OnceCellFailureError),
            };
            return Ok(section);
        } else {
            let section = match section_cell.get() {
                Some(value) => value,
                None => return Err(Error::OnceCellFailureError),
            };
            return Ok(section);
        }
    }


    pub fn section_by_name(
        &'a self,
        name: &[u8],
    ) -> Result<Option<&'a Elf64Section<'a>>, Error> {
        for idx in 0..self.sht_entry_count(){
            let section_name = match self.get_section_name(idx){
                Ok(value) => value,
                Err(_) => return Err(Error::SectionNameFetchingError),
            };
            if name == section_name {
                let section = match self.section(idx){
                    Ok(value) => value,
                    Err(_) => return Err(Error::SectionbuildingError),
                };
                return Ok(Some(section));
            }
        }
        return Ok(None);
    }

    pub fn symbol_by_name(
        &'a self,
        name: &[u8],
    ) -> Result<Option<Elf64Symbol<'a>>, Error> {
        for idx in 0..self.sht_entry_count(){
            let section_header = match self.shdr(idx){
                Ok(value) => value,
                Err(_) => return Err(Error::SectionNameFetchingError),
            };
            if section_header.sh_type() == SHT_SYMTAB || 
                section_header.sh_type() == SHT_DYNSYM{
                let section = match self.section(idx){
                    Ok(value) => value,
                    Err(_) => return Err(Error::SectionbuildingError),
                };
                 match section.symbol_by_name(name){
                    Ok(value) => return Ok(value),
                    Err(_) => return Err(Error::SymbolLookupError),
                };
            }
        }
        return Ok(None);
    }


    fn calc_phdr_offset(
        &self,
        idx: usize,
    ) -> Result<Elf64Off, Error> {
        let e_phnum = self.header.e_phnum();
        if idx >= u16::from(e_phnum) as usize {
            return Err(Error::IndexOutOfBoundsError);
        }
        let ph_offset = self.header.e_phoff().value as usize;
        let ph_entsize = self.header.e_phentsize().value as usize;
        Ok(Elf64Off::from((ph_offset + idx * ph_entsize) as u64))
    }

    pub fn phdr(&self, idx: usize) -> Result<&Elf64Phdr, Error> {
        let ph_cell = match self.pht.get_ph(idx) {
            Ok(value) => value,
            Err(_) => return Err(Error::ProgramHeaderRetrievalError),
        };
        if ph_cell.get().is_none() {
            let ph_offset = match self.calc_phdr_offset(idx) {
                Ok(off) => off.value as usize,
                Err(_) => return Err(Error::CalcOffsetError),
            };

            let phdr_bytes: &[u8] = &self.raw_bytes[ph_offset..];

            let phdr = match Elf64Phdr::from_bytes(
                phdr_bytes,
                self.endianness(),
            ) {
                Ok(value) => value,
                Err(_) => {
                    return Err(Error::ProgramHeaderConstructionError);
                }
            };
            match ph_cell.set(phdr) {
                Ok(_) => (),
                Err(_) => return Err(Error::OnceCellFailureError),
            };
            let ph = match ph_cell.get() {
                Some(value) => value,
                None => return Err(Error::OnceCellFailureError),
            };
            return Ok(ph);
        } else {
            let ph = match ph_cell.get() {
                Some(value) => value,
                None => return Err(Error::OnceCellFailureError),
            };
            return Ok(ph);
        }
    }

    pub fn segment(
        &'a self,
        idx: usize,
    ) -> Result<Elf64Segment<'a>, Error> {
        let header: &Elf64Phdr = match self.phdr(idx) {
            Ok(value) => value,
            Err(_) => return Err(Error::ProgramHeaderRetrievalError),
        };
        let sh_offset = u64::from(header.p_offset()) as usize;
        let sh_size = u64::from(header.p_filesz()) as usize;
        let raw_bytes: &[u8] =
            &self.raw_bytes[sh_offset..sh_offset + sh_size];
        Ok(Elf64Segment::new(raw_bytes, header))
    }
}
