use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("\nWelcome to ROCK, PAPER, SCISSORS!\n------------------------------");

    loop {
        println!("\nHow many rounds would you like to play? (Choose a number between 1 - 10.)");

        let mut rounds = String::new();

        io::stdin()
            .read_line(&mut rounds)
            .expect("Failure to read line.");

        let mut rounds: i32 = match rounds.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        while rounds < 1 || rounds > 10 {
            println!("Please input a number between 1 and 10");

            let mut new_rounds = String::new();

            io::stdin()
                .read_line(&mut new_rounds)
                .expect("Failure to read line.");

            rounds = match new_rounds.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
        }

        let mut round_count: i32 = 1;

        let mut user_score = 0;
        let mut computer_score = 0;
        let mut ties = 0;

        while round_count <= rounds {
            println!("\nRound {}\n--------------------------", round_count);

            println!("Choose:");
            println!("1. Rock");
            println!("2. Paper");
            println!("3. Scissors");

            let mut user_input = String::new();

            io::stdin()
                .read_line(&mut user_input)
                .expect("Failure to read line.");

            let mut user_input: i32 = user_input.trim().parse().expect("Please input a number.");

            while user_input < 1 || user_input > 3 {
                println!("Please input a number between 1 - 3.");

                let mut new_user_input = String::new();

                io::stdin()
                    .read_line(&mut new_user_input)
                    .expect("Failure to read line.");

                user_input = new_user_input
                    .trim()
                    .parse()
                    .expect("Please input a number.");
            }

            match user_input {
                1 => println!("You chose rock."),
                2 => println!("You chose paper."),
                3 => println!("You chose scissors."),
                _ => (),
            }

            let computer_input: i32 = rand::thread_rng().gen_range(1..=3);

            match computer_input {
                1 => println!("The computer chose rock."),
                2 => println!("The computer chose paper."),
                3 => println!("The computer chose scissors."),
                _ => (),
            }

            if user_input == computer_input {
                println!("You tied!");
                ties += 1;
                round_count += 1;
            } else if user_input == 1 && computer_input == 3
                || user_input == 2 && computer_input == 1
                || user_input == 3 && computer_input == 2
            {
                println!("You won!");
                user_score += 1;
                round_count += 1;
            } else {
                println!("You lost!");
                computer_score += 1;
                round_count += 1;
            }
        }
        println!(
            "Your wins: {}, Your losses: {}, Ties: {}, Total rounds: {}",
            user_score, computer_score, ties, rounds
        );
        match user_score.cmp(&computer_score) {
            Ordering::Less => {
                println!("The computer beat you :(");
            }
            Ordering::Greater => {
                println!("You beat the computer!");
            }
            Ordering::Equal => {
                println!("You tied with the computer.");
            }
        }
        println!("\nWould you like to play again? Y / N");

        let mut play_again = String::new();

        io::stdin()
            .read_line(&mut play_again)
            .expect("Failure to read line.");

        if play_again.to_lowercase().contains("y") {
            ();
        } else {
            println!("You are exiting the application. Good bye.");
            break;
        }
    }
}
