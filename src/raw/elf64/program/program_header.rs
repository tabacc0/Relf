use crate::global::error::Error;
use crate::raw::elf64::program::constants::*;
use crate::raw::elf64::types::*;

#[derive(Debug)]
#[repr(C)]
pub struct Elf64Phdr {
    p_type: Elf64Word, //type of segment , see ./constants.rs file
    //specifies flags relevant to the segment , see ./constants.rs file
    p_flags: Elf64Word,
    //the offset from the begining of the file to the segment
    //this can be this way bcs section of the same segment are always
    //contiguous in an elf file
    p_offset: Elf64Off,
    //the virtual address of the start of the segment in virtual memory
    p_vaddr: Elf64Addr,
    //in systems that support it , the pysical address of the start of segment
    p_paddr: Elf64Addr,
    //size of the segment on file , may be 0
    p_filesz: Elf64Xword,
    //size of the segment in the process memory, may not be less than p_filesz
    p_memsz: Elf64Xword,
    //segment alignment , a muliple of two or (0 or 1) for no alignment reqs
    p_align: Elf64Xword,
}

impl Elf64Phdr {
    pub fn from_bytes(
        raw_bytes: &[u8],
        endianness: u8,
    ) -> Result<Self, Error> {
        if raw_bytes.len() < ELF64PHDRSIZE {
            return Err(Error::BufferTooShort);
        }
        let p_type =
            match Elf64Word::from_bytes(&raw_bytes[0..4], endianness) {
                Ok(value) => value,
                Err(_) => return Err(Error::FieldBuildingError),
            };

        let p_flags =
            match Elf64Word::from_bytes(&raw_bytes[4..8], endianness) {
                Ok(value) => value,
                Err(_) => return Err(Error::FieldBuildingError),
            };
        let p_offset =
            match Elf64Off::from_bytes(&raw_bytes[8..16], endianness) {
                Ok(value) => value,
                Err(_) => return Err(Error::FieldBuildingError),
            };

        let p_vaddr =
            match Elf64Addr::from_bytes(&raw_bytes[16..24], endianness) {
                Ok(value) => value,
                Err(_) => return Err(Error::FieldBuildingError),
            };

        let p_paddr =
            match Elf64Addr::from_bytes(&raw_bytes[24..32], endianness) {
                Ok(value) => value,
                Err(_) => return Err(Error::FieldBuildingError),
            };

        let p_filesz =
            match Elf64Xword::from_bytes(&raw_bytes[32..40], endianness) {
                Ok(value) => value,
                Err(_) => return Err(Error::FieldBuildingError),
            };

        let p_memsz =
            match Elf64Xword::from_bytes(&raw_bytes[40..48], endianness) {
                Ok(value) => value,
                Err(_) => return Err(Error::FieldBuildingError),
            };

        let p_align =
            match Elf64Xword::from_bytes(&raw_bytes[48..56], endianness) {
                Ok(value) => value,
                Err(_) => return Err(Error::FieldBuildingError),
            };

        Ok(Self {
            p_type,
            p_offset,
            p_vaddr,
            p_paddr,
            p_filesz,
            p_memsz,
            p_flags,
            p_align,
        })
    }

    pub fn p_type(&self) -> Elf64Word {
        self.p_type
    }
    pub fn p_flags(&self) -> Elf64Word {
        self.p_flags
    }
    pub fn p_offset(&self) -> Elf64Off {
        self.p_offset
    }
    pub fn p_vaddr(&self) -> Elf64Addr {
        self.p_vaddr
    }
    pub fn p_paddr(&self) -> Elf64Addr {
        self.p_paddr
    }
    pub fn p_filesz(&self) -> Elf64Xword {
        self.p_filesz
    }
    pub fn p_memsz(&self) -> Elf64Xword {
        self.p_memsz
    }
    pub fn p_align(&self) -> Elf64Xword {
        self.p_align
    }
}
