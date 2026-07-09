use crate::raw::elf32::error::*;
use crate::raw::elf32::header::Elf32Ehdr;
use crate::raw::elf32::header::constants::*;
use crate::raw::elf32::program::program_header::*;
use crate::raw::elf32::program::program_header_table::*;
use crate::raw::elf32::program::segment::*;
use crate::raw::elf32::section::constants::*;
use crate::raw::elf32::section::section::*;
use crate::raw::elf32::section::section_header::*;
use crate::raw::elf32::section::section_header_table::*;
use crate::raw::elf32::section::section_table::*;
use crate::raw::elf32::types::*;

#[derive(Debug)]
pub struct Elf32<'a> {
    raw_bytes: &'a [u8],
    header: Elf32Ehdr,
    sht: Elf32Sht,
    pht: Elf32Pht,
    //section_table
    st: Elf32SectionTable<'a>,
}

impl<'a> Elf32<'a> {
    pub fn from_bytes(raw_bytes: &'a [u8]) -> Result<Self, Error> {
        if raw_bytes.len() < ELF32EHDRSIZE {
            return Err(Error::BufferTooShort);
        }
        let header = match Elf32Ehdr::from_bytes(raw_bytes) {
            Ok(val) => val,
            Err(_) => return Err(Error::HeaderParsingError),
        };
        let sht = Elf32Sht::new(header.e_shnum());
        let pht = Elf32Pht::new(header.e_phnum());
        let st = Elf32SectionTable::new(header.e_shnum());
        Ok(Self {
            raw_bytes,
            header,
            sht,
            pht,
            st,
        })
    }
    pub fn header(&self) -> &Elf32Ehdr {
        &self.header
    }
    pub fn endianness(&self) -> u8 {
        self.header.endianness()
    }

    fn calc_section_header_offset(
        &self,
        idx: usize,
    ) -> Result<Elf32Off, Error> {
        let e_shnum = self.header.e_shnum();
        if idx >= u16::from(e_shnum) as usize {
            return Err(Error::IndexOutOfBoundsError);
        }
        //offset of the section header table
        let sht_offset = self.header.e_shoff().value as usize;
        //lenght of an entry
        let sh_entsize = self.header.e_shentsize().value as usize;
        //offset + product
        Ok(Elf32Off::from((sht_offset + idx * sh_entsize) as u32))
    }

    pub fn section_header(&self, idx: usize) -> Result<&Elf32Shdr, Error> {
        let sh_cell = match self.sht.get_sh(idx) {
            Ok(value) => value,
            Err(_) => return Err(Error::SectionHeaderRetrievalError),
        };
        if sh_cell.get().is_none() {
            let sh_offset = match self.calc_section_header_offset(idx) {
                Ok(off) => off.value as usize,
                Err(_) => return Err(Error::CalcOffsetError),
            };

            let section_header_bytes = &self.raw_bytes[sh_offset..];

            let section_header = match Elf32Shdr::from_bytes(
                section_header_bytes,
                self.endianness(),
            ) {
                Ok(value) => value,
                Err(_) => {
                    return Err(Error::SectionHeaderConstructionError);
                }
            };
            match sh_cell.set(section_header) {
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

    pub fn section(
        &'a self,
        idx: usize,
    ) -> Result<&'a Elf32Section<'a>, Error> {
        let section_cell = match self.st.get_sh(idx) {
            Ok(value) => value,
            Err(_) => return Err(Error::SectionRetrievalError),
        };
        if section_cell.get().is_none() {
            let header: &Elf32Shdr = match self.section_header(idx) {
                Ok(value) => value,
                Err(_) => {
                    return Err(Error::SectionHeaderRetrievalError);
                }
            };
            let sh_offset = u32::from(header.sh_offset()) as usize;
            let sh_size = u32::from(header.sh_size()) as usize;
            let sh_type = header.sh_type();
            let raw_bytes: &[u8] =
                &self.raw_bytes[sh_offset..sh_offset + sh_size];
            let name_idx = u32::from(header.sh_name()) as usize;
            let name: &[u8];
            let mut link_section: Option<&Elf32Section> = None;
            let mut info_section: Option<&Elf32Section> = None;

            //special treatment for the string table
            //that has the sections names
            //bootstraping its name
            let shstrndx = u16::from(self.header.e_shstrndx()) as usize;

            if idx == shstrndx {
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

            let section = Elf32Section::new(
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

    fn calc_program_header_offset(
        &self,
        idx: usize,
    ) -> Result<Elf32Off, Error> {
        let e_phnum = self.header.e_phnum();
        if idx >= u16::from(e_phnum) as usize {
            return Err(Error::IndexOutOfBoundsError);
        }
        let ph_offset = self.header.e_phoff().value as usize;
        let ph_entsize = self.header.e_phentsize().value as usize;
        Ok(Elf32Off::from((ph_offset + idx * ph_entsize) as u32))
    }

    pub fn program_header(&self, idx: usize) -> Result<&Elf32Phdr, Error> {
        let ph_cell = match self.pht.get_ph(idx) {
            Ok(value) => value,
            Err(_) => return Err(Error::ProgramHeaderRetrievalError),
        };
        if ph_cell.get().is_none() {
            let ph_offset = match self.calc_program_header_offset(idx) {
                Ok(off) => off.value as usize,
                Err(_) => return Err(Error::CalcOffsetError),
            };

            let program_header_bytes: &[u8] = &self.raw_bytes[ph_offset..];

            let program_header = match Elf32Phdr::from_bytes(
                program_header_bytes,
                self.endianness(),
            ) {
                Ok(value) => value,
                Err(_) => {
                    return Err(Error::ProgramHeaderConstructionError);
                }
            };
            match ph_cell.set(program_header) {
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
    ) -> Result<Elf32Segment<'a>, Error> {
        let header: &Elf32Phdr = match self.program_header(idx) {
            Ok(value) => value,
            Err(_) => return Err(Error::ProgramHeaderRetrievalError),
        };
        let sh_offset = u32::from(header.p_offset()) as usize;
        let sh_size = u32::from(header.p_filesz()) as usize;
        let raw_bytes: &[u8] =
            &self.raw_bytes[sh_offset..sh_offset + sh_size];
        Ok(Elf32Segment::new(raw_bytes, header))
    }
}
