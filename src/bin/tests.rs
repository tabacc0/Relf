use relf::raw::elf32::elf::*;
fn main(){
    let elf_file = match Elf32::from_file("./test") {
        Ok(f) =>f.header,
        Err(_) => return (),
    };
    println!("{:#?}",elf_file);
}
