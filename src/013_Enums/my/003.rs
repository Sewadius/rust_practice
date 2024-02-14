use std::io::{self, Write};
use rand::Rng;

// The program is rock/paper/scissors game
#[derive(Debug, PartialEq, Clone, Copy)]
enum Choice {
    Rock, Paper, Scissors,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Result {
    Win, Lose, Draw,
}

fn main() {
    println!("The program is rock/paper/scissors game.");
    let mut score: i32 = 0;

    loop {
        println!("\nChoose your move: 1. Rock, 2. Paper, 3. Scissors");
        print!("Enter your choice (1, 2, 3) or 'q' to quit: ");
        io::stdout().flush().unwrap();

        let user_input: String = get_user_input();

        if &user_input == "q" {
            println!("\nQuitting the game. Goodbye!");
            io::stdin().read_line(&mut String::new()).unwrap();
            break;
        }

        handle_user_input(user_input, &mut score);
    }
}

/// Get user's input from console
fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading user input");
    input.trim().to_lowercase().to_string()
}

/// Operations with user input
fn handle_user_input(user_input: String, score: &mut i32) {
    let correct_range: std::ops::Range<usize> = 1..4;

    match user_input.parse::<usize>() {
        Ok(user_choice) => {
            if correct_range.contains(&user_choice) {
                let computer_choice: Choice = get_computer_choice();
                let user_choice: Choice = get_choice_from_number(user_choice);

                println!("Your choice: {:?}", user_choice);
                println!("Computer's choice: {:?}", computer_choice);

                match determine_winner(user_choice, computer_choice) {
                    Result::Win => {
                        *score += 1;
                        println!("You win! Total score is {}", score);
                    },
                    Result::Lose => {
                        *score -= 1;
                        println!("You lose! Total score is {}", score);
                    },
                    Result::Draw => println!("It's a draw! Total score is {}", score)
                };

            } else {
                println!("Invalid choice! Please enter a number between 1 and 3.");
            }
        }
        Err(_) => println!("Invalid input! Please enter a number or 'q' to quit.")
    };
}

/// Get random computer choice
fn get_computer_choice() -> Choice {
    let choices: [Choice; 3] = [
        Choice::Rock,
        Choice::Paper,
        Choice::Scissors
    ];

    let random_choice: usize = rand::thread_rng().gen_range(0..3);
    choices[random_choice]
}

/// Get current choice via user input
fn get_choice_from_number(number: usize) -> Choice {
    match number {
        1 => Choice::Rock,
        2 => Choice::Paper,
        3 => Choice::Scissors,
        _ => unreachable!(),
    }
}

/// Determine the winner
fn determine_winner(user_choice: Choice, computer_choice: Choice) -> Result {
    if user_choice == computer_choice { return Result::Draw }

    match user_choice {
        Choice::Rock => match computer_choice {
            Choice::Paper => Result::Lose,
            Choice::Scissors => Result::Win,
            _ => unreachable!()
        },
        Choice::Paper => match computer_choice {
            Choice::Scissors => Result::Lose,
            Choice::Rock => Result::Win,
            _ => unreachable!()
        },
        Choice::Scissors => match computer_choice {
            Choice::Rock => Result::Lose,
            Choice::Paper => Result::Win,
            _ => unreachable!()
        }
    }
}
