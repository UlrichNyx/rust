// ### Author
// 
// * UlrichNyx
//
// ### Date
//
// * 2024-10-10
//
// ### Description
//
// A Rust program that implements a guessing game. It allows users to input their name, 
// load player data from a file, guess a randomly generated number, and update their score 
// if they guess correctly. The program uses external crates like `serde` for JSON serialization 
// and deserialization to manage player data, and it displays the leaderboard sorted by player scores.

use std::io::{self, BufReader};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;
use rand::Rng;
use std::cmp::Ordering;

/// Represents a player in the game.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Player {
    name: String,
    score: u32,
}

/// Holds a collection of players.
#[derive(Serialize, Deserialize)]
struct PlayerData {
    players: Vec<Player>
}

/// Implements ordering for players based on their score.
impl Ord for Player {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)  // Compare by score
    }
}

/// Implements partial ordering for players.
impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Player {}

/// Implements equality check for players based on their score.
impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

/// Loads player data from a file.
///
/// ### Arguments
///
/// * `filename` - The path to the file containing player data.
///
/// ### Returns
///
/// * `Result<PlayerData, io::Error>` - Returns player data or an I/O error.
fn load_players(filename: &str) -> Result<PlayerData, io::Error> {
    let file: File = File::open(filename)?;
    let reader = BufReader::new(file);
    let player_data: PlayerData = serde_json::from_reader(reader).expect("Failed to read!");
    Ok(player_data)
}

/// Displays the leaderboard in descending order of player scores.
///
/// ### Arguments
///
/// * `leaderboard` - A mutable reference to the player data.
fn display_leaderboard(leaderboard: &mut PlayerData) {
    leaderboard.players.sort_by(|a, b| b.score.cmp(&a.score));
    println!("========LEADERBOARD========");
    for (index, element) in leaderboard.players.iter().enumerate() {
        println!("|{}| - {} | {}", index + 1, element.name, element.score);
    }
}

/// Main function to start the game and manage player data.
///
/// ### Returns
///
/// * `Result<(), io::Error>` - Returns an empty result or an I/O error.
fn main() -> Result<(), io::Error> {
    let filename = "leaderboard.json";
    println!("Welcome to the guessing game! What is your name?");
    let mut player_name = String::new();
    io::stdin().read_line(&mut player_name).expect("Failed to read line!");
    let player_name: &str = player_name.trim();

    let mut player_data: PlayerData = load_players(&filename)?;

    // Look for the player in the existing leaderboard
    let mut player_index: Option<usize> = None;
    let mut player: Player = Player {
        name: player_name.to_string(),
        score: 0
    };

    for (index, element) in player_data.players.iter().enumerate() {
        if element.name == player_name {
            player = Player::from(element.clone());
            player_index = Some(index);
            println!("Welcome back, {}!", player.name);
            break;
        }
    }

    if player_index.is_none() {
        println!("Welcome, {}!", player.name);
    }

    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret_number}");

    // Game loop for number guessing
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line.");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            },
        };

        println!("You guessed: {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    // Update player's score and save data
    player.score += 1;
    if let Some(index) = player_index {
        player_data.players[index].score = player.score;
    } else {
        player_data.players.push(player);
    }

    let json_data: String = serde_json::to_string_pretty(&player_data).expect("Failed to serialize");
    let mut file: File = File::create(filename)?;
    file.write_all(json_data.as_bytes())?;

    // Display the updated leaderboard
    display_leaderboard(&mut player_data);

    Ok(())
}
