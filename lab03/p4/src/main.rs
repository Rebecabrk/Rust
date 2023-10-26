#[derive(PartialEq)]
enum MyError {
    NotAscii,
    NotDigit,
    NotBase16Digit,
    NotLetter,
    NotPrintable,
}

fn to_uppercase(c: char) -> Result<char, MyError> {
    //41-5a????? 61-7a???? utf-8
    let ascii = c as u8;
    if (ascii >= 65 && ascii <= 90) || (ascii >= 97 && ascii <= 122) {
        return Ok((ascii - 32) as char);
    }
    Err(MyError::NotLetter)
}

fn to_lowercase(c: char) -> Result<char, MyError> {
    //41-5a????? 61-7a???? utf-8
    let ascii = c as u8;
    if (ascii >= 65 && ascii <= 90) || (ascii >= 97 && ascii <= 122) {
        return Ok((ascii + 32) as char);
    }
    Err(MyError::NotLetter)
}

fn print_char(c: char) -> Result<char, MyError> {
    if c >= ' ' && c <= '~' {
        Ok(c)
    } else {
        Err(MyError::NotPrintable)
    }
}

fn char_to_number(c: char) -> Result<u32, MyError> {
    let code = c as u32;
    if code > 127 {
        return Err(MyError::NotAscii);
    }
    if code < 48 && code > 57 {
        return Err(MyError::NotDigit);
    }
    Ok(code)
}

fn char_to_number_hex(c: char) -> Result<u32, MyError> {
    let code = c as u32;
    if code > 127 {
        return Err(MyError::NotAscii);
    }
    if (code < 48 || code > 57) && (code < 65 || code > 70) {
        return Err(MyError::NotBase16Digit);
    }
    Ok(code)
}



fn main() {
    match to_uppercase('a') {
        Err(e) => print_error(e),
        Ok(x) => println!("toUpperCase: {}", x),
    }
    match to_lowercase('A') {
        Err(e) => print_error(e),
        Ok(x) => println!("toLowerCase: {}", x),
    }
    match print_char(15 as char) {
        Err(e) => print_error(e),
        Ok(x) => println!("{} is printable", x),
    }
    match char_to_number('r') {
        Err(e) => print_error(e),
        Ok(x) => println!("charToNumber: {}", x),
    }
    match char_to_number_hex('F') {
        Err(e) => print_error(e),
        Ok(x) => println!("charToNumberHex: {}", format!("{:X}", x)),
    }
}
