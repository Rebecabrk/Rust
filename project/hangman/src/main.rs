use anyhow::Result;
use chrono::Local;
use rand::seq::SliceRandom;
use serde_derive::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, Read, Write};

#[derive(Deserialize)]
struct Category {
    category: String,
    words: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Score {
    date: String,
    value: i32,
}

#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    best_score: i32,
    total_score: i32,
    scores: Vec<Score>,
}

fn print_user_scores(username: String) {
    let file = File::open("scores.json").expect("Failed to open file");
    let reader = BufReader::new(file);

    let users: Vec<User> = serde_json::from_reader(reader).expect("Failed to deserialize users");

    for user in users {
        if user.username == username {
            println!("The scores of {username}");
            println!("Total score: {}", user.total_score);
            println!("Best score: {}", user.best_score);
            for score in user.scores {
                println!("Date: {}, Score: {}", score.date, score.value);
            }
        }
    }
}

fn update_scores(username: String, score: i32) {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("scores.json")
        .expect("Failed to open file");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    let mut users: Vec<User> = if contents.is_empty() {
        Vec::new()
    } else {
        serde_json::from_str(&contents).expect("Failed to deserialize users")
    };

    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    match users.iter_mut().find(|user| user.username == username) {
        Some(user) => {
            user.scores.push(Score {
                date: now.clone(),
                value: score,
            });
            if user.best_score < score {
                user.best_score = score;
            }
            user.total_score += score;
        }
        None => {
            users.push(User {
                username: username.to_string(),
                best_score: score,
                total_score: score,
                scores: vec![Score {
                    date: now.clone(),
                    value: score,
                }],
            });
        }
    }

    let json = serde_json::to_string_pretty(&users).expect("Failed to serialize users");
    drop(file);
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("scores.json")
        .expect("Failed to open file");

    file.write_all(json.as_bytes())
        .expect("Failed to write to file");
}

fn generate_word(cat: String) -> Result<String> {
    let file = File::open("dictionary.json")?;
    let reader = BufReader::new(file);
    let categories: Vec<Category> = serde_json::from_reader(reader)?;
    let mut rng = rand::thread_rng();

    for c in categories {
        if c.category == cat {
            let word = c.words.choose(&mut rng);
            if let Some(word) = word {
                return Ok(word.to_string());
            }
        }
    }
    Ok(String::new())
}

fn main() {
    let mut correct_guesses: Vec<char> = Vec::new();
    let mut all_guesses: Vec<char> = Vec::new();
    let mut category = String::new();
    let mut username = String::new();
    let mut answer = String::new();
    let mut mistakes = 0;
    let mut letter: char;

    println!("Welcome to the Hangman!");
    println!("Please enter a username: ");
    match io::stdin().read_line(&mut username) {
        Ok(_) => {
            username = username.replace('\r', "");
            username = username.replace('\n', "");
            println!("Hello, {}!", username);
        }
        Err(e) => println!("Failed to read from stdin: {e}"),
    }

    println!("Please enter a category:");
    println!("-> sport");
    println!("-> school");
    println!("-> kitchen");
    println!("-> geography");
    println!("-> farm");
    match io::stdin().read_line(&mut category) {
        Ok(_) => println!("You entered: {}", category),
        Err(e) => println!("Failed to read from stdin: {e}"),
    }
    category = category.replace('\r', "");
    category = category.replace('\n', "");

    let word = generate_word(category);
    let guess: String = match word {
        Ok(a) => {
            print!("Your word is:");
            for _x in a.chars() {
                print!("-");
            }
            println!();
            a
        }
        Err(e) => {
            println!("There was an error at generating a word: {e}");
            println!("Please come back later.");
            return;
        }
    };

    let mut distinct_chars: Vec<char> = guess.chars().collect();
    distinct_chars.sort_unstable();
    distinct_chars.dedup();

    let mistakes_allowed = ((guess.len() as f32) * 0.5 + 1.0) as usize;
    while mistakes < mistakes_allowed {
        loop {
            println!(
                "The number of remaining attempts: {}",
                mistakes_allowed - mistakes
            );
            print!("Give me a letter: ");
            std::io::stdout().flush().unwrap();
            let mut buffer = String::new();
            match io::stdin().read_line(&mut buffer) {
                Ok(_) => println!(),
                Err(e) => println!("Failed to read from stdin: {e}"),
            }
            buffer = buffer.replace('\r', "");
            buffer = buffer.replace('\n', "");

            match buffer.len() {
                0 => println!("You did not enter anything!"),
                1 => {
                    let aux = buffer.chars().next();
                    match aux {
                        Some(x) => {
                            if x.is_ascii_lowercase() || x.is_ascii_uppercase() {
                                letter = x;
                            } else {
                                println!("Please provide an ascii characther: [a-z][A-Z].");
                                letter = ' ';
                            }
                        }
                        None => {
                            println!("Error at getting the letter!");
                            return;
                        }
                    }
                    break;
                }
                _ => println!("One letter at a time, please!"),
            }
        }
        if letter != ' ' {
            if guess.contains(letter) {
                if all_guesses.contains(&letter) {
                    println!("You already guessed this letter!");
                } else {
                    correct_guesses.push(letter);
                    all_guesses.push(letter);
                    for i in guess.chars() {
                        if correct_guesses.contains(&i) {
                            print!("{i}");
                            std::io::stdout().flush().unwrap();
                        } else {
                            print!("-");
                            std::io::stdout().flush().unwrap();
                        }
                    }
                    println!();
                    println!();
                }
            } else if all_guesses.contains(&letter) {
                println!("You already tried this letter! It's not correct.");
                print!("Your guessed letters up until now: ");
                println!();
                for i in guess.chars() {
                    if correct_guesses.contains(&i) {
                        print!("{i}");
                        std::io::stdout().flush().unwrap();
                    } else {
                        print!("-");
                        std::io::stdout().flush().unwrap();
                    }
                }
                println!();
                println!();
            } else {
                all_guesses.push(letter);
                println!("No. Try again!");
                print!("Your guessed letters up until now: ");
                println!();
                for i in guess.chars() {
                    if correct_guesses.contains(&i) {
                        print!("{i}");
                        std::io::stdout().flush().unwrap();
                    } else {
                        print!("-");
                        std::io::stdout().flush().unwrap();
                    }
                }
                println!();
                println!();
                mistakes += 1;
            }
        }
        if correct_guesses.len() == distinct_chars.len() {
            break;
        }
    }
    if mistakes == mistakes_allowed {
        println!("You lost. The word was: {guess}");
        println!("Incorrect guesses: {mistakes_allowed}");
        update_scores(username.clone(), 0);
    } else {
        println!("Congratulations! You won!");
        println!("The word was indeed: {guess}");
        println!("Incorrect guesses: {mistakes}");
        println!(
            "Your score is {}",
            guess.len() * (mistakes_allowed - mistakes)
        );
        update_scores(
            username.clone(),
            (guess.len() * (mistakes_allowed - mistakes)) as i32,
        );
    }

    println!("Do you want to see your scores?[yes/no]");
    match io::stdin().read_line(&mut answer) {
        Ok(_) => {
            answer = answer.replace('\r', "");
            answer = answer.replace('\n', "");
            match answer.as_str() {
                "yes" => print_user_scores(username),
                "Yes" => print_user_scores(username),
                "y" => print_user_scores(username),
                "Y" => print_user_scores(username),
                "YES" => print_user_scores(username),
                "no" => println!("Ok, goodbye!"),
                "No" => println!("Ok, goodbye!"),
                "n" => println!("Ok, goodbye!"),
                "N" => println!("Ok, goodbye!"),
                "NO" => println!("Ok, goodbye!"),
                _ => println!(
                    "I'm not sure what your answer is. Next time answer with [yes/no]. Goodbye!"
                ),
            }
        }
        Err(e) => println!("Failed to read from stdin: {e}"),
    }
}
