use std::{fs, io};

fn main() -> Result<(), io::Error> {
    let s = fs::read_to_string("input.txt")?;
    let mut nr_char = 0;
    let mut nr_line = 0;
    for lines in s.lines() {
        nr_line += 1;
        for _i in lines.chars() {
            nr_char += 1;
        }
        println!(
            "Pe linia {nr_line} sunt {nr_char} caractere si {} bytes.",
            lines.len()
        );
        nr_char = 0;
    }
    Ok(())
}
