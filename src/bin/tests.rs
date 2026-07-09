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
    let mut j: usize = 0;
    loop {
        let section = match elf_file.section(j) {
            Ok(value) => value,
            Err(_) => return (),
        };
        let section_name = section.name();
        println!("[{j}] {} ", str::from_utf8(section_name).unwrap());

        if section.is_symtab() {
            println!("symbols : ");
            let mut i: usize = 0;
            loop {
                let symbol = match section.symbol(i) {
                    Ok(value) => value,
                    Err(_) => break,
                };
                i += 1;
                let symbol_name = symbol.name();
                println!(
                    "      [{i}] {}",
                    str::from_utf8(symbol_name).unwrap()
                );
            }
        }
        j += 1;
    }
}
