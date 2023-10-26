#[derive(Debug)]
enum Error {
    Overflow,
    _Underflow,
}

fn add(x: u32, y: u32) -> Result<u32, Error> {
    let sum = x as u64 + y as u64;
    if sum > std::u32::MAX as u64 {
        return Err(Error::Overflow);
    }

    Ok(sum as u32)
}

fn multiply(x: u32, y: u32) -> Result<u32, Error> {
    let mult = x as u64 * y as u64;
    if mult > std::u32::MAX as u64 {
        return Err(Error::Overflow);
    }
    Ok(mult as u32)
}

fn check_addition(x: u32, y: u32) -> Result<u32, Error> {
    add(x, y)?;
    Ok(add(x, y)?)
}
fn check_multiplication(x: u32, y: u32) -> Result<u32, Error> {
    multiply(x, y)?;
    Ok(multiply(x, y)?)
}

fn main() {
    let x = 500;
    let y = 600;
    match check_addition(x, y) {
        Ok(x) => println!("value: {}", x),
        Err(e) => eprintln!("{:?}", e),
    }
    match check_multiplication(x, y) {
        Ok(x) => println!("value: {}", x),
        Err(e) => eprintln!("{:?}", e),
    }
}
