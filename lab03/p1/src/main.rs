fn next_prime(n: u16) -> Option<u16> {
    let mut i = n as u32 + 1;
    loop {
        let mut is_prime = true;
        let mut j = 2;
        while j < i {
            if i % j == 0 {
                is_prime = false;
                break;
            }
            j += 1;
        }
        if i == u16::MAX as u32 {
            return None;
        }
        if is_prime {
            return Some(i as u16);
        }
        i += 1;
    }
}

fn main() {
    let mut flag: bool = true;
    let mut i = 1;
    while flag {
        let x = next_prime(i);
        match x {
            Some(x) => {
                println!("{}", x);
                i = x;
            }
            None => {
                println!("no next prime number as u16");
                flag = false;
            }
        }
    }
}
