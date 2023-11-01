use std::{io, fs};


fn main() -> Result<(), io::Error> {
    let s = fs::read_to_string("C:\\Windows\\System32\\drivers\\etc\\protocol")?;
    let mut new_s = String::new();
    let mut index : usize ;
    //let mut name = String::new();
    //let mut number = String::new();
    let mut ok = 1;
    let mut third_column_index;
    for lines in s.lines(){
        if ok == 1{
            if lines.find("IP").is_some() {
                third_column_index = lines.find("IP").unwrap();
            }
            else{
                continue;
            }
        }
        else {
            continue;
        }

        ok += 1;
        
        if lines.find('#').is_some() {
            index = lines.find('#').unwrap();
        }
        else  {
            //nu exista #
            //println!("{lines}");
            index = 0;
            new_s.push_str(&lines[0..third_column_index]);
        }
        
        if lines.is_empty(){
            //ignoram
            continue;
        } 
        else if index >= 1{
            //exista #, dar nu e la inceput
            new_s.push_str(&lines[0..third_column_index]);
        }
    }
    
    println!("{new_s}");
    Ok(())
}
