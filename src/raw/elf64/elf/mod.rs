use crate::global::error::Error;
use crate::model::elf64::section::section_table::*;
use crate::raw::elf64::elf::section_header_iter::Elf64ShdrIter;
use crate::raw::elf64::elf::segment_header_iter::Elf64PhdrIter;
use crate::raw::elf64::header::constants::*;
use crate::raw::elf64::header::header::Elf64Ehdr;
use crate::raw::elf64::program::program_header::*;
use crate::raw::elf64::program::program_header_table::*;
use crate::raw::elf64::section::section_header::*;
use crate::raw::elf64::section::section_header_table::*;
use crate::raw::elf64::types::*;

pub mod section_header_iter;
pub mod segment_header_iter;
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
            return Err(Error::BufferTooShort);
        }
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
    pub fn raw_bytes(&'a self) -> &'a [u8] {
        self.raw_bytes
    }
    pub(crate) fn st(&'a self) -> &'a Elf64SectionTable<'a> {
        &self.st
    }
    pub fn endianness(&self) -> u8 {
        self.header.endianness()
    }
    pub fn sht_entry_count(&self) -> usize {
        u16::from(self.header.e_shnum()) as usize
    }

    fn calc_shdr_offset(&self, idx: usize) -> Result<Elf64Off, Error> {
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

            let shdr =
                match Elf64Shdr::from_bytes(shdr_bytes, self.endianness())
                {
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

    fn calc_phdr_offset(&self, idx: usize) -> Result<Elf64Off, Error> {
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

            let phdr =
                match Elf64Phdr::from_bytes(phdr_bytes, self.endianness())
                {
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
    pub fn shdr_iter(&'a self) -> Elf64ShdrIter<'a> {
        Elf64ShdrIter::new(&self)
    }
    pub fn phdr_iter(&'a self) -> Elf64PhdrIter<'a> {
        Elf64PhdrIter::new(&self)
    }
}
