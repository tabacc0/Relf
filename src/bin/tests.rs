use relf::raw::elf64::elf::*;
use relf::global::error::*;
fn main()  -> Result<(),Error> {
    let elf_bytes = match std::fs::read("./test") {
        Ok(f) => f,
        Err(_) => return Ok(()),
    };
    let elf_file = Elf64::from_bytes(&elf_bytes)?;
    let section = elf_file.section_by_name(b".dynsym")?.unwrap();
    let section_name = section.name();
    println!(" section : {}  ", str::from_utf8(section_name).unwrap());
    
    let mut i = 0;
    for symbol in section.symbol_iter(){
            let symbol_name = &symbol.name();
            let symbol_name = &symbol_name[0..symbol_name.len().min(30)];
            println!(
                "[{i}] symbol: {:<30}\tvalue : {:#16x}",
                str::from_utf8(symbol_name).unwrap(),
                symbol.value()
            );
            i += 1;
    }

    let mut i = 0;
    let section = elf_file.section_by_name(b".rela.plt")?.unwrap();
    for relocation in section.relocation_iter(){
            let relocation_offset = &relocation.offset();
            println!(
                "[{i}] relocation offset: {:#}",
                relocation_offset
                );
            i += 1;
    }

    let mut i = 0;

    for section in elf_file.section_iter(){
            let section_name = &section.name();
            let section_name = &section_name[0..section_name.len().min(30)];
            println!(
                "[{i}] section: {:<30}\toffset : {:#16x}",
                str::from_utf8(section_name).unwrap(),
                section.file_offset()
            );
            i += 1;
    }

    Ok(())
}
