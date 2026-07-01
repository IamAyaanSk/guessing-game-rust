use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let random_number = rand::thread_rng().gen_range(0..=100);

    let mut guess = String::new();
    println!("Guess Game");

    loop {
        guess.clear();

        println!("Type 'exit' to end the game");
        println!("Please enter a number:");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to input number");

        if guess.trim() == "exit" {
            println!("Game Ended!! The number was {random_number}");
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&random_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                println!("Congratulations!!!");
                break;
            }
        }
    }
}
