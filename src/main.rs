use rand::Rng;
use std::cmp::Ordering;
use std::fs::read_link;
use std::io;

fn main() {
    println!("Guessing Game");
    let secret_num: u32 = rand::thread_rng().gen_range(1..101);
    //println!("Guess A number between 1 and 100");
    loop {
        println!("Please input your guess:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error, please try again");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Greater => println!("Number too big"),
            Ordering::Less => println!("Number too small"),
            Ordering::Equal => {
                println!("You won");
                break;
            }
        };
    }
}
