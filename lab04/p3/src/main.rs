use std::{fs, io};

fn main() -> Result<(), io::Error> {
    let mut s = fs::read_to_string("input.txt")?;
    //let abrev = String::from("pt,ptr,dl,dna,omw,lol");
    s = s.replace("pt", "pentru");
    s = s.replace("ptr", "pentru");
    s = s.replace("dl", "domnul");
    s = s.replace("doamna", "doamna");
    println!("{s}");
    Ok(())
}
