use std::io;

fn main() {
    println!("Welcome to the guessing game!");

    loop {
        println!("\nPick difficulty:");
        println!("1) Easy   1 -> 50   [10 lives]");
        println!("2) Medium 1 -> 100  [12 lives]");
        println!("3) Hard   1 -> 500  [15 lives]");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice = choice.trim();

        let (min, max, lives, secret) = match choice {
            "1" => (1, 50, 10, 27),     // Easy
            "2" => (1, 100, 12, 73),    // Medium
            "3" => (1, 500, 15, 314),   // Hard
            _ => {
                println!("Invalid choice, try again.");
                continue;
            }
        };

        let mut remaining = lives;
        let range_size = max - min;

        println!("Guess the number between {} and {}!", min, max);

        while remaining > 0 {
            let mut guess = String::new();
            io::stdin().read_line(&mut guess).expect("Failed to read input");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number.");
                    continue;
                }
            };

            remaining -= 1;

            if guess == secret {
                println!("Congrats! You guessed the number!");
                break;
            } else {
                if guess < secret {
                    println!("Too low! Lives left: {}", remaining);
                } else {
                    println!("Too high! Lives left: {}", remaining);
                }

                let distance = if secret > guess { secret - guess } else { guess - secret };
                let closeness = ((range_size - distance) * 20 / range_size) as usize;
                let bar = format!(
                    "[{}{}]",
                    "#".repeat(closeness),
                    "-".repeat(20 - closeness)
                );
                println!("Closeness: {}", bar);
            }

            if remaining == 0 {
                println!("Game over! The secret number was {}.", secret);
            }
        }

        println!("Play again? (y/n)");
        let mut again = String::new();
        io::stdin().read_line(&mut again).expect("Failed to read input");
        if again.trim().to_lowercase() != "y" {
            println!("Thanks for playing!");
            break;
        }
    }
}
