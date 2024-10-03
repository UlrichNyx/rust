use std::io::{self, BufReader};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;
use rand::Rng;
use std::cmp::Ordering; 

impl Ord for Player {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)  // Compare by score
    }
}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Player {}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Player {
    name: String,
    score: u32,
}

#[derive(Serialize, Deserialize)]
struct PlayerData {
    players: Vec<Player>
}

fn load_players(filename: &str) -> Result<PlayerData, io::Error> {
    let file : File = File::open(filename)?;
    let reader = BufReader::new(file);
    let player_data: PlayerData = serde_json::from_reader(reader).expect("Failed to read!");
    Ok(player_data)
}

fn display_leaderboard(leaderboard: &mut PlayerData) {
    leaderboard.players.sort_by(|a, b| b.score.cmp(&a.score));
    println!("========LEADERBOARD========");
    for (index, element) in leaderboard.players.iter().enumerate()
    {
        println!("|{}| - {} | {}", index + 1, element.name, element.score)
    }
}

fn main() -> Result<(), io::Error>  {
    let filename = "leaderboard.json";
    println!("Welcome to the guessing game! What is your name?");
    let mut player_name = String::new();
    io::stdin().read_line(&mut player_name).expect("Failed to read line!");
    let player_name: &str = player_name.trim();

    let mut player_data : PlayerData = load_players(&filename)?;

    let mut player_index: Option<usize> = None;

    let mut player: Player = Player {
        name: player_name.to_string(),
        score: 0
    };

    for (index, element) in player_data.players.iter().enumerate()
    {
        if element.name == player_name
        {
            player = Player::from(element.clone());
            player_index = Some(index);
            println!("Welcome back, {}!", player.name);
            break;
        }
    }

    if player_index == None
    {
        println!("Welcome, {}!", player.name);
    }

    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret_number}");

    
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line.");
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
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

    player.score += 1;

    if let Some(index) = player_index
    {
        player_data.players[index].score = player.score;
    }
    else {
        
        player_data.players.push(player);
    }
    let json_data: String = serde_json::to_string_pretty(&player_data).expect("Failed to serialize");

    let mut file: File = File::create("leaderboard.json")?;

    display_leaderboard(&mut player_data);

    file.write_all(json_data.as_bytes())?;
    Ok(())




}