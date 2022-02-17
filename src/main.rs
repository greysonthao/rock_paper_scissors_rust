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

        let min_rounds = 1;
        let max_rounds = 10;

        confirm_legit_input(&mut rounds, min_rounds, max_rounds);

        let mut round_count: i32 = 1;

        let mut user_score = 0;
        let mut computer_score = 0;
        let mut ties = 0;

        while round_count <= rounds {
            print_weapons(round_count);

            let mut user_input = String::new();

            io::stdin()
                .read_line(&mut user_input)
                .expect("Failure to read line.");

            let mut user_input: i32 = user_input.trim().parse().expect("Please input a number.");

            let min_user_input = 1;
            let max_user_input = 3;

            confirm_legit_input(&mut user_input, min_user_input, max_user_input);

            generate_user_output(user_input);

            let computer_input: i32 = rand::thread_rng().gen_range(1..=3);

            generate_comp_output(computer_input);

            determine_round_winner(
                user_input,
                computer_input,
                &mut ties,
                &mut round_count,
                &mut user_score,
                &mut computer_score,
            );
        }
        determine_game_winner(user_score, computer_score, ties, rounds);

        let play_again = determine_play_again();

        if play_again.to_lowercase().contains("y") {
            ();
        } else {
            println!("You are exiting the application. Good bye.");
            break;
        }
    }
}

fn determine_play_again() -> String {
    println!("\nWould you like to play again? Y / N");
    let mut play_again = String::new();
    io::stdin()
        .read_line(&mut play_again)
        .expect("Failure to read line.");
    play_again
}

fn determine_game_winner(user_score: i32, computer_score: i32, ties: i32, rounds: i32) {
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
}

fn print_weapons(round_count: i32) {
    println!("\nRound {}\n--------------------------", round_count);
    println!("Choose:");
    println!("1. Rock");
    println!("2. Paper");
    println!("3. Scissors");
}

fn determine_round_winner(
    user_input: i32,
    computer_input: i32,
    ties: &mut i32,
    round_count: &mut i32,
    user_score: &mut i32,
    computer_score: &mut i32,
) {
    if user_input == computer_input {
        println!("You tied!");
        *ties += 1;
        *round_count += 1;
    } else if user_input == 1 && computer_input == 3
        || user_input == 2 && computer_input == 1
        || user_input == 3 && computer_input == 2
    {
        println!("You won!");
        *user_score += 1;
        *round_count += 1;
    } else {
        println!("You lost!");
        *computer_score += 1;
        *round_count += 1;
    }
}

fn generate_comp_output(computer_input: i32) {
    match computer_input {
        1 => println!("The computer chose rock."),
        2 => println!("The computer chose paper."),
        3 => println!("The computer chose scissors."),
        _ => (),
    }
}

fn generate_user_output(user_input: i32) {
    match user_input {
        1 => println!("You chose rock."),
        2 => println!("You chose paper."),
        3 => println!("You chose scissors."),
        _ => (),
    }
}

fn confirm_legit_input(user_input: &mut i32, min: i32, max: i32) {
    while *user_input < min || *user_input > max {
        println!("Please input a number between {} - {}.", min, max);

        let mut new_user_input = String::new();

        io::stdin()
            .read_line(&mut new_user_input)
            .expect("Failure to read line.");

        *user_input = new_user_input
            .trim()
            .parse()
            .expect("Please input a number.");
    }
}
