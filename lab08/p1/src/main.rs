use anyhow::Result;
use std::{fs, collections::HashMap};

fn format_print(sorted : Vec<(String, i32)>) -> (){
    let mut v = sorted.clone();
    v.sort_unstable_by_key(|x| x.0.len());
    let max_number = v.iter().next_back().map(|x| x.0.len()).unwrap_or(0);
    for i in sorted.iter(){
        println!("{:<number$} => {}",i.0, i.1, number = max_number as usize);
    }
}

fn parse_file(content : &str) {
    let mut map = HashMap::<String,i32>::new();

    for line in content.lines(){
        for word in line.split(|c| c == ' ' || c == '.'){
            if !word.is_empty(){
                map.entry(word.to_lowercase()).and_modify(|x| *x = *x + 1).or_insert(1);
            }
        }
    }

    let mut v: Vec<(String, i32)> = map.into_iter().collect();
    v.sort_unstable_by_key(|x| -x.1);
    format_print(v);
}

fn main() ->Result<()> {
    let content_string = fs::read_to_string("file_input.txt")?;
    let content_str: &str = &content_string;
    parse_file(content_str);  
    Ok(())
}
