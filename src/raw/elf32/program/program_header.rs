use crate::raw::elf32::error::Error;
use crate::raw::elf32::types::*;


//values of p_type and their interpretation
//all these values are optional by the spec
//a section program header may contain only fields that are relevant to it
//
//
//unused/invalid array member , to be ignored
pub const PT_NULL : Elf32Word = Elf32Word{value:0};
//loadable segment, the bytes from the file are put into memory
//and padded with 0s if p_memsz is greater than p_filesz
pub const PT_LOAD : Elf32Word = Elf32Word{value:1};
//entry with dynamic linking info
pub const PT_DYNAMIC : Elf32Word = Elf32Word{value:2};
//entry has size and location of the path of the interpreter (null term str)
pub const PT_INTERP : Elf32Word = Elf32Word{value:3};
//entry has auxiliary info
pub const PT_NOTE : Elf32Word = Elf32Word{value:4};
//unused
pub const PT_SHLIB : Elf32Word = Elf32Word{value:5};
//entry holding the location and size of the ph table itself
pub const PT_PHDR : Elf32Word = Elf32Word{value:6};
//these two define the bounds of the range reserved for cpu-specific semantics
pub const PT_LOPROC : Elf32Word = Elf32Word{value:0x70000000};
pub const PT_HIPROC : Elf32Word = Elf32Word{value:0x7fffffff};


pub const VALID_PT : &[Elf32Word] = &[
    PT_NULL,
    PT_LOAD,
    PT_DYNAMIC,
    PT_INTERP,
    PT_NOTE,
    PT_SHLIB,
    PT_PHDR,
];


//masks of p_flags and their interpretation
//
//
pub const PF_X : Elf32Word = Elf32Word{value:1};//this segment is executable
pub const PF_W : Elf32Word = Elf32Word{value:2};//writable at runtime
pub const PF_R : Elf32Word = Elf32Word{value:4};//readable at runtime
//mask bit for os-specific semantics
pub const PF_MASKOS : Elf32Word = Elf32Word{value:0x0ff00000};
//mask bit for cpu-specific semantics
pub const PF_MASKPROC : Elf32Word = Elf32Word{value:0xf0000000};


#[derive(Debug)]
#[repr(C)]
pub struct Elf32Phdr {
    p_type : Elf32Word,//type of segment , see above 
    //the offset from the begining of the file to the segment
    //this can be this way bcs section of the same segment are always
    //contiguous in an elf file
    p_offset : Elf32Off,
    //the virtual address of the start of the segment in virtual memory
    p_vaddr : Elf32Addr,
    //in systems that support it , the pysical address of the start of segment
    p_paddr : Elf32Addr,
    //size of the segment on file , may be 0
    p_filesz : Elf32Word,
    //size of the segment in the process memory, may not be less than p_filesz
    p_memsz : Elf32Word,
    //specifies flags relevant to the segment , see above
    p_flags : Elf32Word,
    //segment alignment , a muliple of two or (0 or 1) for no alignment reqs
    p_align : Elf32Word,
}

impl Elf32Phdr {
    pub fn from_bytes(raw_bytes : &[u8;size_of::<Elf32Phdr>()],endianness:u8) 
        -> Result<Self,Error> {

        let p_type = 
            match Elf32Word::from_bytes(&raw_bytes[0..4],endianness){
                Ok(value) => {
                    if !VALID_PT.contains(&value) && 
                        (value < PT_LOPROC || value > PT_HIPROC)
                    {
                        return Err(Error::InvalidFieldValue);
                }
                value
            },
            Err(e) => return Err(Error::FieldBuildingError),
        };

        let p_offset = 
            match Elf32Off::from_bytes(&raw_bytes[4..8],endianness){
                Ok(value) => value,
                Err(e) => return Err(Error::FieldBuildingError),
            };

        let p_vaddr = 
            match Elf32Addr::from_bytes(&raw_bytes[8..12],endianness){
                Ok(value) => value,
                Err(e) => return Err(Error::FieldBuildingError),
            };

        let p_paddr = 
            match Elf32Addr::from_bytes(&raw_bytes[12..16],endianness){
                Ok(value) => value,
                Err(e) => return Err(Error::FieldBuildingError),
            };

        let p_filesz = 
            match Elf32Word::from_bytes(&raw_bytes[16..20],endianness){
                Ok(value) => value,
                Err(e) => return Err(Error::FieldBuildingError),
            };

        let p_memsz = 
            match Elf32Word::from_bytes(&raw_bytes[20..24],endianness){
                Ok(value) => {
                    if value < p_filesz {
                        return Err(Error::InvalidSegmentMemSz);
                    }
                    value
            },
            Err(e) => return Err(Error::FieldBuildingError),
        };

        let p_flags = 
            match Elf32Word::from_bytes(&raw_bytes[24..28],endianness){
                Ok(value) => value,
                Err(e) => return Err(Error::FieldBuildingError),
            };

        let p_align = 
            match Elf32Word::from_bytes(&raw_bytes[28..32],endianness){
                Ok(value) => {
                    //making sure alignment is sane
                    if u32::from(&value) != 1 && u32::from(&value) % 2 != 0{//
                        return Err(Error::InvalidFieldValue);
                    }
                value
            }
            Err(e) => return Err(Error::FieldBuildingError),
        };


        Ok(Self {
            p_type , 
            p_offset , 
            p_vaddr , 
            p_paddr,
            p_filesz , 
            p_memsz , 
            p_flags , 
            p_align,
        })
    }

    pub fn p_type(&self) -> &Elf32Word {
        &self.p_type
    }
    pub fn p_offset(&self) -> &Elf32Off {
        &self.p_offset
    }
    pub fn p_vaddr(&self) -> &Elf32Addr {
        &self.p_vaddr
    }
    pub fn p_paddr(&self) -> &Elf32Addr {
        &self.p_paddr
    }
    pub fn p_filesz(&self) -> &Elf32Word {
        &self.p_filesz
    }
    pub fn p_memsz(&self) -> &Elf32Word {
        &self.p_memsz
    }
    pub fn p_flags(&self) -> &Elf32Word {
        &self.p_flags
    }
    pub fn p_align(&self) -> &Elf32Word {
        &self.p_align
    }

}
