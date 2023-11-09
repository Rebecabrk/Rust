use anyhow::Result;
use serde_derive::Deserialize;
use std::fs;
#[derive(Debug, Deserialize)]
struct Student {
    name: String,
    phone: String,
    age: i32,
}

fn main() -> Result<()> {
    let text = fs::read_to_string("text.json").unwrap();
    let students: Vec<Student> = serde_json::from_str(&text).unwrap();
    let mut min_age: i32 = i32::MAX;
    let mut max_age: i32 = i32::MIN;
    let mut youngest = Student {
        name: String::from(""),
        age: 0,
        phone: String::from(""),
    };
    let mut oldest = Student {
        name: String::from(""),
        age: 0,
        phone: String::from(""),
    };

    for student in students {
        if student.age < min_age {
            min_age = student.age;
            youngest = student;
        } else if student.age > max_age {
            max_age = student.age;
            oldest = student;
        }
    }
    println!(
        "youngest: name={}, age={}, phone_number=0{} ",
        youngest.name, youngest.age, youngest.phone
    );
    println!(
        "oldest: name={}, age={}, phone_number=0{} ",
        oldest.name, oldest.age, oldest.phone
    );
    Ok(())
}
