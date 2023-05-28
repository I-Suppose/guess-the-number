use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("guess the number version 0.0.0.1");

    // Generate secret number
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Record the number of attempts
    let mut num_loop = 0;
 
    loop {
        // Increment attempt counter
        num_loop += 1;

        // Asking user for guess
        let mut user_guess = String::new();
        println!("guess the number???!!!");
        io::stdin()
            .read_line(&mut user_guess)
            .expect("entre another number");
        println!("You guessed: {user_guess}");

        // Convert user guess from String to i32
        let user69: i32 = match user_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("enter a number dumbass");
                continue;
            },
        };

        // Determine if user guess is right
        match user69.cmp(&secret_number) {
            Ordering::Less => println!("your guess is too low"),
            Ordering::Greater => println!("your number is too high"),
            Ordering::Equal => {
                println!("u got the number :)");
                println!("you guessed {} times", num_loop);
                break;
            },
        };
    }
}
