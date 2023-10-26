fn add(x: u32, y: u32) -> u32 {
    let sum = x as u64 + y as u64;
    if sum > std::u32::MAX as u64 {
        panic!("This addition cannot fit into u32!");
    }
    sum as u32
}

fn multiply(x: u32, y: u32) -> u32 {
    let mult = x as u64 * y as u64;
    if mult > std::u32::MAX as u64 {
        panic!("This multiplication cannot fit into u32!");
    }
    mult as u32
}
fn main() {
    let x = add(100, 200);
    println!("{}", x);
    let y = multiply(4294967294, 2);
    println!("{}", y);
}
