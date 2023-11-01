use std::{io, fs};


fn main() -> Result<(), io::Error> {
    let s = fs::read_to_string("C:\\Windows\\System32\\drivers\\etc\\protocol")?;
    let mut index : usize ;
    let mut column1 = String::new();
    let mut column2 = String::new();
    for lines in s.lines(){
        if lines.find('#').is_some() {
            index = lines.find('#').unwrap();
        }
        else  {
            //nu exista #
            println!("{lines}");
            index = 0;
        }
        
        if lines.is_empty(){
            continue;
        } 
        else if index >= 1{
            //exista #, dar nu e la inceput
            //println!("{lines}");
            for coloumn in lines.splitn(3,' '){
                //column1.push_str(coloumn)
            }
        }
    }
    Ok(())
}
