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
    
    for i in 0..section.entry_count(){
            let symbol = section.symbol(i)?;
            let symbol_name = &symbol.name();
            let symbol_name = &symbol_name[0..symbol_name.len().min(30)];
            println!(
                "[{i}] symbol: {:<30}\tvalue : {:#16x}",
                str::from_utf8(symbol_name).unwrap(),
                symbol.value()
            );
    }
    Ok(())
}
