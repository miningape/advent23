use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Lines},
};

fn problem_input_iterator() -> Lines<BufReader<File>> {
    let arguments: Vec<String> = env::args().collect();
    let filepath = &arguments.get(1).unwrap();
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);

    return reader.lines();
}

#[derive(Debug)]
struct Hand {
    red: u32,
    blue: u32,
    green: u32,
}

impl Hand {
    fn new() -> Hand {
        Hand {
            red: 0,
            blue: 0,
            green: 0,
        }
    }

    fn power(&self) -> u64 {
        u64::from(self.red) * u64::from(self.blue) * u64::from(self.green)
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Game {
    id: u32,
    hands: Vec<Hand>,
}

fn pick_greatest(left: u32, right: u32) -> u32 {
    if left > right {
        left
    } else {
        right
    }
}

impl Game {
    fn minimum_hand(&self) -> Hand {
        let mut minumum_hand = Hand::new();

        for hand in &self.hands {
            minumum_hand.red = pick_greatest(minumum_hand.red, hand.red);
            minumum_hand.blue = pick_greatest(minumum_hand.blue, hand.blue);
            minumum_hand.green = pick_greatest(minumum_hand.green, hand.green);
        }

        minumum_hand
    }
}

fn game_id(entry: &&str) -> Result<u32, String> {
    if !entry.starts_with("Game ") {
        return Err(String::from("Line does not start with \"Game \""));
    }

    let id: Vec<&str> = entry.split("Game ").collect();
    id.get(1)
        .ok_or(String::from("Could not get game ID"))?
        .parse::<u32>()
        .map_err(|e| e.to_string())
}

fn hand(hand_string: &str) -> Result<Hand, String> {
    let pulls: Vec<&str> = hand_string.split(',').collect();
    let mut hand = Hand::new();

    for pull in pulls {
        let detail: Vec<&str> = pull.split(' ').collect();
        let number = detail
            .get(1)
            .ok_or("Could not read number of colored cubes")?
            .parse::<u32>()
            .map_err(|err| err.to_string())?;
        let color = detail
            .get(2)
            .ok_or("Could not read color of colored cubes")?;

        match color {
            &"red" => hand.red += number,
            &"blue" => hand.blue += number,
            &"green" => hand.green += number,
            _ => return Err(String::from("Could not match color")),
        }
    }

    Ok(hand)
}

fn hands(entry: &&str) -> Result<Vec<Hand>, String> {
    let hand_strings: Vec<&str> = entry.split(';').collect();
    let mut hands: Vec<Hand> = Vec::new();

    for hand_string in hand_strings {
        hands.push(hand(hand_string)?)
    }

    Ok(hands)
}

fn error_on_line(line: String) -> impl Fn(String) -> String {
    return move |err: String| format!("Error on line: \"{}\"\n\t - {}", line, err);
}

fn parse_line(line: String) -> Result<Game, String> {
    let line = line.clone();
    let tokens: Vec<&str> = line.split(':').collect();
    let id = game_id(tokens.get(0).ok_or("Could not read game ID")?)
        .map_err(error_on_line(line.clone()))?;
    let hands = hands(tokens.get(1).ok_or("Could not read game hands")?)
        .map_err(error_on_line(line.clone()))?;

    Ok(Game { id, hands })
}

fn main() {
    let problem_input = problem_input_iterator();

    let mut sum = 0;

    for line in problem_input {
        let line = line.unwrap();
        let game = parse_line(line).unwrap();

        sum += game.minimum_hand().power();
    }

    println!("\n[AOC 2023 D2P1] The result is: {}", sum)
}
