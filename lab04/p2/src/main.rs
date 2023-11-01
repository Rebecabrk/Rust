fn my_rot13(s: String) -> Option<String> {
    let mut new_string = String::new();
    let mut code: u8;
    for i in s.chars() {
        if (i >= 'a' && i < 'n') || (i >= 'A' && i < 'N') {
            code = i as u8 + 13;
        } else if (i >= 'n' && i <= 'z') || (i >= 'N' && i <= 'Z') {
            code = i as u8 - 13;
        } else {
            return None;
        }
        new_string.push(code as char);
    }
    Some(new_string)
}

fn main() {
    let message = String::from("Hello");
    println!("mesaj: \"{message}\" => ");
    let weakly_encoded_message = my_rot13(message);
    match weakly_encoded_message {
        Some(x) => println!("mesaj secret: {:?}", x),
        None => println!("am intalnit caracter non-ascii"),
    }
}
