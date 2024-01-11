use anyhow::Result;
use std::cell::RefCell;
use std::io;
use std::io::Write;

#[derive(Copy, Clone)]
struct Numbers {
    num: i32,
    is_prime: bool,
}

struct Cache {
    last_10_num: RefCell<Vec<Numbers>>,
}

impl Cache {
    fn new() -> Cache {
        Cache {
            last_10_num: RefCell::new(Vec::new()),
        }
    }
    fn add_num(self: &mut Self, num: i32) -> () {
        if self.last_10_num.borrow().len() == 10 {
            self.delete_num();
        }
        if is_prime(num) == true {
            self.last_10_num.borrow_mut().push(Numbers {
                num: num,
                is_prime: true,
            });
        } else {
            self.last_10_num.borrow_mut().push(Numbers {
                num: num,
                is_prime: false,
            });
        }
    }
    fn delete_num(self: &mut Self) -> () {}
    fn check_num(self: &mut Self, num: i32) -> Numbers {
        for i in self.last_10_num.borrow().iter() {
            if i.num == num {
                return *i;
            }
        }
        Numbers {
            num: -1,
            is_prime: false,
        }
    }
}

fn is_prime(num: i32) -> bool {
    if num < 2 {
        return false;
    }
    if num == 2 {
        return true;
    }
    if num % 2 == 0 {
        return false;
    }
    for i in (3..).step_by(2){
        if i * i > num{
            break;
        } else if num % i == 0{
            return false;
        }
    }
    true
}

fn main() -> Result<()> {
    let mut cache = Cache::new();
    let mut input = String::new();

    println!("Welcome to the isPrime Calculator.");
    println!("Please enter a number that you want to check if it is prime or not. When you are done checking numbers, please write <quit>");
    print!("Enter number: ");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut input)?;
    println!();
    while input.trim() != "quit" {
        let num: i32 = input.trim().parse()?;
        let n = cache.check_num(num);
        if n.num == num {
            print!("The value {num} is already in the cache and it");
            if n.is_prime == true {
                println!(" is prime!");
            } else {
                println!(" is not prime!");
            }
        } else {
            print!("The value {num} is not in the cache. ");
            cache.add_num(num);
            if is_prime(num) == true {
                println!("The number is prime!");
            } else {
                println!("The number is not prime!");
            }
        }
        println!("Use <quit> to stop the programme.");
        print!("Enter number: ");
        let _ = io::stdout().flush();
        input.clear();
        io::stdin().read_line(&mut input)?;
        println!();
    }
    println!("Goodbye!");
    Ok(())
}
