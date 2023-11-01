use std::io::{Error, ErrorKind};
use std::{fs, io};

fn my_rot13() -> Result<(),io::Error> {
    let my_err = Error::new(ErrorKind::Other, "NotAscii");
    let s = fs::read_to_string("input.txt")?;
    let mut new_string = String::new();
    let mut code: u8;
    for i in s.chars() {
        if (i >= 'a' && i < 'n') || (i >= 'A' && i < 'N') {
            code = i as u8 + 13;
        } else if (i >= 'n' && i <= 'z') || (i >= 'N' && i <= 'Z') {
            code = i as u8 - 13;
        } else if i >= ' ' && i <= '~' {
            code = i as u8;
        }
        else {
            return Err(my_err);
        }
        new_string.push(code as char);
    }
    fs::write("output.txt", &new_string)?;
    Ok(())
}

fn main() {
    match my_rot13(){
        Ok(()) => (),
        Err(e) => println!("Eroare: {e}"),
    }
}
