fn is_prime(x: i32) -> bool {
    if x < 2 {
        return false;
    } else if x == 2 {
        return true;
    } else if x % 2 == 0 {
        return false;
    }

    let mut i = 3;
    while i < x / 2 {
        if x % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

fn main() {
    println!("Starting to serch for all prime numbers from 0 to 100...");
    let mut iterator = 0;
    let mut contor = 0;
    while iterator <= 100 {
        if is_prime(iterator) == true {
            println!("{} ", iterator);
            contor += 1;
        }
        iterator += 1;
    }
    println!("Search ended.Total number is: {}", contor);
}
