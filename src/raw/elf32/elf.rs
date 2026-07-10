use crate::global::error::Error;
use crate::raw::elf32::header::Elf32Ehdr;
use crate::raw::elf32::header::constants::*;
use crate::raw::elf32::program::program_header::*;
use crate::raw::elf32::program::program_header_table::*;
use crate::raw::elf32::section::section_header::*;
use crate::raw::elf32::section::section_header_table::*;
use crate::model::elf32::section::section_table::*;
use crate::raw::elf32::types::*;

#[derive(Debug)]
pub struct Elf32<'a> {
    raw_bytes: &'a [u8],
    header: Elf32Ehdr,
    sht: Elf32Sht,
    pht: Elf32Pht,
    //table of section abstraction
    st: Elf32SectionTable<'a>,
}

impl<'a> Elf32<'a> {
    pub fn from_bytes(raw_bytes: &'a [u8]) -> Result<Self, Error> {
        if raw_bytes.len() < ELF32EHDRSIZE {
            return Err(Error::BufferTooShort); }
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
    pub fn raw_bytes(&'a self) -> &'a [u8] {
        self.raw_bytes
    }
    pub fn header(&self) -> &Elf32Ehdr {
        &self.header
    }
    pub (crate) fn st(&'a self) -> &'a Elf32SectionTable<'a> {
        &self.st
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

    pub fn shdr(&self, idx: usize) -> Result<&Elf32Shdr, Error> {
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

            let shdr = match Elf32Shdr::from_bytes(
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


    fn calc_phdr_offset(
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

    pub fn phdr(&self, idx: usize) -> Result<&Elf32Phdr, Error> {
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

            let phdr = match Elf32Phdr::from_bytes(
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

}
