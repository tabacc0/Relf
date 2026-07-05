use crate::raw::elf32::types::*;

//unused/invalid array member , to be ignored
const PT_NULL : Elf32Word = Elf32Word{value:0};
//loadable segment, the bytes from the file are put into memory
//and padded with 0s if p_memsz is greater than p_filesz
const PT_LOAD : Elf32Word = Elf32Word{value:1};
//entry with dynamic linking info
const PT_DYNAMIC : Elf32Word = Elf32Word{value:2};
//entry has size and location of the path of the interpreter (null term str)
const PT_INTERP : Elf32Word = Elf32Word{value:3};
//entry has auxiliary info
const PT_NOTE : Elf32Word = Elf32Word{value:4};
//unused
const PT_SHLIB : Elf32Word = Elf32Word{value:5};
//entry holding the location and size of the ph table itself
const PT_PHDR : Elf32Word = Elf32Word{value:6};
//these two define the bound of values reserved for cpu-specific semantics
const PT_LOPROC : Elf32Word = Elf32Word{value:0x70000000};
const PT_HIPROC : Elf32Word = Elf32Word{value:0x7fffffff};

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
