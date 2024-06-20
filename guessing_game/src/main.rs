use rand::Rng;
use std::cmp::Ordering;
use std::io;

// you can use cargo update to implicantly specify dependency version upgrades
fn main() {
    println!("--- Number guessing game ---");

    let secret_numer: u32 = rand::thread_rng().gen_range(1..=100);

    //println!("The secret numer is: {secret_numer}");

    loop {
        println!("Please enter a guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_numer) {
            Ordering::Less => println!("Your guess is too small"),
            Ordering::Greater => println!("Your guess is too big"),
            Ordering::Equal => {
                println!("Your guess is right, good job!");
                break;
            }
        }
    }
}
