use crate::raw::elf32::error::*;
use crate::raw::elf32::relocation::*;
use crate::raw::elf32::section::section::*;
use crate::raw::elf32::symbol::symbol::*;
#[derive(Debug)]
pub enum Elf32RelocationHeader {
    Rel(Elf32Rel),
    Rela(Elf32Rela),
}

#[derive(Debug)]
pub struct Elf32Relocation<'a> {
    header: Elf32RelocationHeader,
    //caching symbols somewhere wasn't worth it
    //so i violated the layer just for this because it
    //is really better to parse symbols from a table lazily
    //than to allocate OnceCells for 10000 symbols that won't be used
    symbol_table: &'a Elf32Section<'a>,
}

impl<'a> Elf32Relocation<'a> {
    pub fn new(
        header: Elf32RelocationHeader,
        symbol_table: &'a Elf32Section,
    ) -> Self {
        Self {
            header,
            symbol_table,
        }
    }
    pub fn offset(&self) -> u32 {
        match &self.header {
            Elf32RelocationHeader::Rel(header) => {
                return u32::from(header.r_offset());
            }
            Elf32RelocationHeader::Rela(header) => {
                return u32::from(header.r_offset());
            }
        }
    }
    pub fn info(&self) -> u32 {
        match &self.header {
            Elf32RelocationHeader::Rel(header) => {
                return u32::from(header.r_info());
            }
            Elf32RelocationHeader::Rela(header) => {
                return u32::from(header.r_info());
            }
        }
    }
    pub fn addend(&self) -> Option<i32> {
        match &self.header {
            Elf32RelocationHeader::Rel(_) => return None,
            Elf32RelocationHeader::Rela(header) => {
                return Some(i32::from(header.r_addend()));
            }
        }
    }
    pub fn symbol(&'a self) -> Result<Elf32Symbol<'a>, Error> {
        match &self.header {
            Elf32RelocationHeader::Rel(header) => {
                let symbol_idx = header.relocated_symbol_idx();
                let symbol = match self.symbol_table.symbol(symbol_idx) {
                    Ok(value) => value,
                    Err(_) => return Err(Error::SymbolFetchingError),
                };
                return Ok(symbol);
            }
            Elf32RelocationHeader::Rela(header) => {
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
