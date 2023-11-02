use std::{io, fs};

fn main() -> Result<(), io::Error> {
    let s = fs::read_to_string("C:\\Windows\\System32\\drivers\\etc\\protocol")?;
    let mut index : usize ;
    let mut ignore;
    for line in s.lines(){
        if line.starts_with('#'){
            ignore = true;
        }
        else {
            ignore = false;
        }
        index = 0;
        for word in line.split_ascii_whitespace(){
            if ignore == false{
                index += 1;
                if index == 1 {
                    print!("{word} => ");
                }
                else if index == 2 {
                    println!("{word}");
                }
            }
        }
    }
    Ok(())
}
