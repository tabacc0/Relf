use relf::raw::elf32::elf::*;
fn main(){
    let mut elf_file = match Elf32::from_file("./test") {
        Ok(f) =>f,
        Err(_) => return (),
    };
    let mut i : usize = 0;
    let mut test_section = match elf_file.section(i) {
        Ok(value) => value,
        Err(_) => return (),
    };
    while !test_section.is_symtab(){
        i = i+1;
        test_section = match elf_file.section(i) {
            Ok(value) =>  value,
            Err(e) => continue,
        };
    }


    let test_symbol = match test_section.symbol(3) {
        Ok(value) => value,
        Err(_) => return (),
    };

    println!("{:#?}",test_symbol);
}
