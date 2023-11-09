use anyhow::Result;
use std::fs;

#[derive(Debug)]
struct Student {
    name: String,
    age: i32,
    phone_number: i64,
}

fn main() -> Result<()> {
    let text = fs::read_to_string("text.txt")?;
    let mut contor;
    let mut min_age: i32 = i32::MAX;
    let mut max_age: i32 = i32::MIN;
    //let mut age: i32 = 0;
    //let mut phone_number: i64 = 0;
    let mut youngest = Student {
        name: String::from(""),
        age: 0,
        phone_number: 0,
    };
    let mut oldest = Student {
        name: String::from(""),
        age: 0,
        phone_number: 0,
    };

    for line in text.lines() {
        let mut s = Student {
            name: String::from(""),
            age: 0,
            phone_number: 0,
        };
        contor = 0;
        for word in line.split(',') {
            //let mut name = String::from("");
            if contor == 0 {
                s.name = word.to_string();
            } else if contor == 1 {
                s.phone_number = word.parse()?;
            } else if contor == 2 {
                s.age = word.parse()?;
            }
            contor += 1;
        }
        if s.age < min_age {
            min_age = s.age;
            youngest = s;
        } else if s.age > max_age {
            max_age = s.age;
            oldest = s;
        }
    }
    println!(
        "youngest: name={}, age={}, phone_number=0{} ",
        youngest.name, youngest.age, youngest.phone_number
    );
    println!(
        "oldest: name={}, age={}, phone_number=0{} ",
        oldest.name, oldest.age, oldest.phone_number
    );
    Ok(())
}
