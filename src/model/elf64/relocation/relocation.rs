use crate::global::error::Error;
use crate::raw::elf64::relocation::rel::*;
use crate::raw::elf64::relocation::rela::*;
use crate::model::elf64::section::section::*;
use crate::model::elf64::symbol::symbol::*;
#[derive(Debug)]
pub enum Elf64RelocationHeader {
    Rel(Elf64Rel),
    Rela(Elf64Rela),
}

#[derive(Debug)]
pub struct Elf64Relocation<'a> {
    header: Elf64RelocationHeader,
    //caching symbols somewhere wasn't worth it
    //so i violated the layer just for this because it
    //is really better to parse symbols from a table lazily
    //and not cache them
    //than to allocate OnceCells for 10000 symbols that won't be used
    symbol_table: &'a Elf64Section<'a>,
    //this holds a reference to the section that the relocation applies 
    //to , this way this relocation object has every info it could need
    relocation_target: &'a Elf64Section<'a>,
}

impl<'a> Elf64Relocation<'a> {
    pub fn new(
        header: Elf64RelocationHeader,
        symbol_table: &'a Elf64Section,
        relocation_target: &'a Elf64Section,
    ) -> Self {
        Self {
            header,
            symbol_table,
            relocation_target,
        }
    }
    pub fn offset(&self) -> u64 {
        match &self.header {
            Elf64RelocationHeader::Rel(header) => {
                return u64::from(header.r_offset());
            }
            Elf64RelocationHeader::Rela(header) => {
                return u64::from(header.r_offset());
            }
        }
    }
    pub fn info(&self) -> u64 {
        match &self.header {
            Elf64RelocationHeader::Rel(header) => {
                return u64::from(header.r_info());
            }
            Elf64RelocationHeader::Rela(header) => {
                return u64::from(header.r_info());
            }
        }
    }
    pub fn addend(&self) -> Option<i64> {
        match &self.header {
            Elf64RelocationHeader::Rel(_) => return None,
            Elf64RelocationHeader::Rela(header) => {
                return Some(i64::from(header.r_addend()));
            }
        }
    }
    pub fn target_section(&'a self) -> &'a Elf64Section<'a> {
        self.relocation_target
    }
    pub fn symbol(&'a self) -> Result<Elf64Symbol<'a>, Error> {
        match &self.header {
            Elf64RelocationHeader::Rel(header) => {
                let symbol_idx = header.relocated_symbol_idx();
                let symbol = match self.symbol_table.symbol(symbol_idx) {
                    Ok(value) => value,
                    Err(_) => return Err(Error::SymbolFetchingError),
                };
                return Ok(symbol);
            }
            Elf64RelocationHeader::Rela(header) => {
                let symbol_idx = header.relocated_symbol_idx();
                let symbol = match self.symbol_table.symbol(symbol_idx) {
                    Ok(value) => value,
                    Err(_) => return Err(Error::SymbolFetchingError),
                };
                return Ok(symbol);
            }
        }
    }
}
