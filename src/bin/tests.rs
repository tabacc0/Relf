use relf::raw::elf32::elf::*;
fn main(){
    let mut elf_file = match Elf32::from_file("./test") {
        Ok(f) =>f,
        Err(_) => return (),
    };
    let test_section = match elf_file.section_header(3) {
        Ok(value) => value,
        Err(_) => return (),
    };
    let test_section2 = match elf_file.section_header(3) {
        Ok(value) => value,
        Err(_) => return (),
    };
    println!("{:#?}",test_section);
}
