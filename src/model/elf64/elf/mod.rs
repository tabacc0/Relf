use crate::global::error::Error;
use crate::model::elf64::elf::section_iter::Elf64SectionIter;
use crate::model::elf64::elf::segment_iter::Elf64SegmentIter;
use crate::model::elf64::program::segment::*;
use crate::model::elf64::section::section::*;
use crate::model::elf64::symbol::symbol::Elf64Symbol;
use crate::raw::elf64::elf::Elf64;
use crate::raw::elf64::program::program_header::*;
use crate::raw::elf64::section::constants::*;
use crate::raw::elf64::section::section_header::*;

pub mod section_iter;
pub mod segment_iter;
impl<'a> Elf64<'a> {
    pub fn get_section_name(
        &'a self,
        idx: usize,
    ) -> Result<&'a [u8], Error> {
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

        let shstrndx = u16::from(self.header().e_shstrndx()) as usize;

        //special treatment for the string table
        //that has the sections names
        //bootstraping its name
        if idx == shstrndx {
            if self.raw_bytes().len() < sh_offset + sh_size {
                return Err(Error::BufferTooShort);
            }
            let raw_bytes =
                &self.raw_bytes()[sh_offset..sh_offset + sh_size];
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
        let section_cell = match self.st().get_sh(idx) {
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
            let sh_type = header.sh_type();

            let file_size = match sh_type {
                SHT_NOBITS => 0usize,
                _ => u64::from(header.sh_size()) as usize,
            };

            if sh_offset + file_size > self.raw_bytes().len() {
                println!("{sh_offset}");
                println!("{file_size}");
                println!("{}",self.raw_bytes().len());
                return Err(Error::BufferTooShort);
            }
            let raw_bytes: &[u8] =
                &self.raw_bytes()[sh_offset..sh_offset + file_size];
            let name: &[u8];
            let mut link_section: Option<&Elf64Section> = None;
            let mut info_section: Option<&Elf64Section> = None;

            name = match self.get_section_name(idx) {
                Ok(value) => value,
                Err(_) => return Err(Error::SectionNameFetchingError),
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
        for idx in 0..self.sht_entry_count() {
            let section_name = match self.get_section_name(idx) {
                Ok(value) => value,
                Err(_) => return Err(Error::SectionNameFetchingError),
            };
            if name == section_name {
                let section = match self.section(idx) {
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
        for idx in 0..self.sht_entry_count() {
            let section_header = match self.shdr(idx) {
                Ok(value) => value,
                Err(_) => return Err(Error::SectionNameFetchingError),
            };
            if section_header.sh_type() == SHT_SYMTAB
                || section_header.sh_type() == SHT_DYNSYM
            {
                let section = match self.section(idx) {
                    Ok(value) => value,
                    Err(_) => return Err(Error::SectionbuildingError),
                };
                match section.symbol_by_name(name) {
                    Ok(value) => return Ok(value),
                    Err(_) => return Err(Error::SymbolLookupError),
                };
            }
        }
        return Ok(None);
    }

    pub fn segment(
        &'a self,
        idx: usize,
    ) -> Result<Elf64Segment<'a>, Error> {
        let header: &Elf64Phdr = match self.phdr(idx) {
            Ok(value) => value,
            Err(_) => return Err(Error::ProgramHeaderRetrievalError),
        };
        let ph_offset = u64::from(header.p_offset()) as usize;
        let ph_size = u64::from(header.p_filesz()) as usize;
        if ph_offset + ph_size > self.raw_bytes().len() {
            return Err(Error::BufferTooShort);
        }
        let raw_bytes: &[u8] =
            &self.raw_bytes()[ph_offset..ph_offset + ph_size];
        Ok(Elf64Segment::new(raw_bytes, header))
    }
    pub fn section_iter(&'a self) -> Elf64SectionIter<'a> {
        Elf64SectionIter::new(&self)
    }
    pub fn segment_iter(&'a self) -> Elf64SegmentIter<'a> {
        Elf64SegmentIter::new(&self)
    }
}
