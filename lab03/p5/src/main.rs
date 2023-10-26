use std::io::{self, BufRead};

#[derive(PartialEq)]
enum Feelings {
    Sad,
    Angry,
    Lonely,
    Desperate,
    Hopeless,
    Helpless,
    Other,
}

fn read_word() -> Result<String, Feelings> {
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.lock().read_line(&mut buffer).unwrap();
    let mut s = String::new();
    match buffer.trim() {
        "Sad" => Err(Feelings::Sad),
        "Angry" => Err(Feelings::Angry),
        "Lonely" => Err(Feelings::Lonely),
        "Desperate" => Err(Feelings::Desperate),
        "Helpless" => Err(Feelings::Helpless),
        "Hopeless" => Err(Feelings::Hopeless),
        "Happy" => {
            s.push_str("Happy");
            Ok(s)
        }
        "Content" => {
            s.push_str("Content");
            Ok(s)
        }
        "Ok" => {
            s.push_str("Ok");
            Ok(s)
        }
        _ => Err(Feelings::Other),
    }
}

fn print_error(mood: Feelings) -> () {
    match mood {
        Feelings::Sad => println!("You're making me sad too. Goodye!"),
        Feelings::Angry => println!("WHYYYYY???????? BYE!"),
        Feelings::Lonely => println!("I am lonely too. Join the club."),
        Feelings::Desperate => println!("Pfff, nothing is so important, anyways."),
        Feelings::Helpless => println!("Get some help from someone else, would you?."),
        Feelings::Hopeless => println!("That's the most awful thing, I'm really sorry."),
        _ => println!("I don't understand. Try again later."),
    }
}

fn main() {
    println!("How are you feeling today? Choose one: ");
    println!("-> Happy");
    println!("-> Content");
    println!("-> Ok");
    println!("-> Sad");
    println!("-> Angry");
    println!("-> Lonely");
    println!("-> Desperate");
    println!("-> Hopeless");
    println!("-> Helpless");

    match read_word() {
        Err(e) => print_error(e),
        Ok(x) => {
            println!("It's good you are {}", x);
            println!("Good for YOU!!!!!!");
        }
    }
}
