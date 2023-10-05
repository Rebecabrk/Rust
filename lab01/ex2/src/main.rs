fn are_coprime(mut x: i32, mut y: i32) -> bool {
    let mut rest;
    if y == 0 {
        return false;
    }
    while y != 0 {
        rest = x % y;
        x = y;
        y = rest;
    }
    if x == 1 {
        return true;
    } else {
        return false;
    }
}

fn main() {
    let mut i = 0;
    let mut j: i32;
    let mut contor = 0;
    println!("Searching for all coprime numbers from 0 to 100...");
    while i <= 100 {
        j = 0;
        while j <= 100 {
            if are_coprime(i, j) == true {
                println!("({},{})", i, j);
                contor += 1;
            }
            j += 1;
        }
        i += 1;
    }
    println!(
        "Search ended. Total number of coprime numbers is: {}",
        contor
    );
}
