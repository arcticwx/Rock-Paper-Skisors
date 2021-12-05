#[macro_use]
extern crate text_io;

use rand::Rng;

// If random value 1 picked, rock is selected by computer.
// If random value 2 picked, paper is selected by computer.
// If random value 3 picked, scissors is selected by computer.

fn help(){
    //DEBUG PURPOUSES
    //println!("You called the help function.")
    //END OF DEBUG PROCESS

    println!("Rock paper skisors, a game of rock papaer scissors played in the terminal, if you are unaware RPS is a game where two entities select
    an option of either rock, paper, or scissors. Rock beats scissors, scissors beat paper, paper beats rock.");

    println!("All you have to do is say an option, rock, papper, or scissors. Go forth and have fun!");
}

fn rock(){
    let mut rng = rand::thread_rng();
    //Doesn't pick 3 if 3 is the max range for some reason? the max range will stay at 4 then.
    let mut rps = rng.gen_range(1..4);

    match rps {
        1 => println!("You draw!, The computer picked rock."),
        2 => println!("You loose!, The computer picked paper."),
        3 => println!("You win!, The computer picked scissors"),
        _ => println!("Failed!")
    }
}

fn paper(){
    let mut rng = rand::thread_rng();
    //See above note in the rock() function to see why max range is 4 not 3.
    let mut rps = rng.gen_range(1..4);
    
    match rps {
        1 => println!("You win!, The computer picked rock."),
        2 => println!("You draw!, The computer picked paper."),
        3 => println!("You loose!, The computer picked scissors"),
        _ => println!("Failed!")
    }
}

fn skisors(){
    let mut rng = rand::thread_rng();
    //See above note in the rock() function to see why max range is 4 not 3.
    let mut rps = rng.gen_range(1..4);
    
    match rps {
        1 => println!("You loose!, The computer picked rock."),
        2 => println!("You win!, The computer picked paper."),
        3 => println!("You draw!, The computer picked scissors"),
        _ => println!("Failed!")
    }
}

fn main() {
    println!("Welcome to rock paper skisors");
    println!("Enter rock, paper, scissors, or help");
    let player_input: String = read!();

    match player_input.as_str() {
        "help" => help(),
        "Help" => help(),
        "rock" => rock(),
        "Rock" => rock(),
        "paper" => paper(),
        "Paper" => paper(),
        "scissors" => skisors(),
        "Scissors" => skisors(),
        "skisors" => skisors(),
        "Skisors" => skisors(),
        _ => println!("Failed!"),
    }
}
