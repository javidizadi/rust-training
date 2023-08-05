use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io::stdin;
fn main() {
    println!("Generating secret number...");
    let answer: u8 = thread_rng().gen_range(1..=50);
    let mut guess = String::new();
    loop {
        guess.clear();
        println!("Guess the secret number: ");
        match stdin().read_line(&mut guess) {
            Ok(_) => {}
            Err(_) => continue,
        }
        let guess_num: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess_num.cmp(&answer) {
            Ordering::Less => println!("LESS"),
            Ordering::Greater => println!("BIGGER"),
            Ordering::Equal => {
                println!("DONE!");
                break;
            }
        }
    }
}
