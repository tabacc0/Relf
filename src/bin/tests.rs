use relf::raw::elf32::elf::*;
fn main() {
    let elf_bytes = match std::fs::read("./test") {
        Ok(f) => f,
        Err(_) => return (),
    };
    let elf_file = match Elf32::from_bytes(&elf_bytes) {
        Ok(f) => f,
        Err(_) => return (),
    };
    let section = match elf_file.section_by_name(b".rel.plt") {
        Ok(value) => value.unwrap(),
        Err(_) => return (),
    };
    let section_name = section.name();
    println!(" section : {}  ", str::from_utf8(section_name).unwrap());

    if section.is_reltab() {
        println!("relocations : ");
        for i in 0..section.entry_count(){
            let relocation= match section.relocation(i as usize) {
                Ok(value) => value,
                Err(_) => break,
            };
            let  target = relocation.target_section().name() ;
            let  symbol= match relocation.symbol() {
                Ok(value) => value,
                Err(_) => break,
            };
            let symbol_name = symbol.name();
            println!(
                "\trelocation offset : {}\n\
                \taffected section : {}\n\
                \t\trellocated symbol:\n\
                \t\t\t name : {}\n\
                \t\t\t value : {}\n\
                ",
                relocation.offset(),
                str::from_utf8(target).unwrap(),
                str::from_utf8(symbol_name).unwrap(),
                symbol.value()
            );
        }
    }
}
