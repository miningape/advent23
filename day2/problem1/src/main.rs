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

    fn is_bounded_by(&self, other: &Hand) -> bool {
        if self.red > other.red {
            return false;
        }

        if self.blue > other.blue {
            return false;
        }

        if self.green > other.green {
            return false;
        }

        return true;
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    hands: Vec<Hand>,
}

impl Game {
    fn is_bounded_by(&self, other: &Hand) -> bool {
        for hand in &self.hands {
            if !hand.is_bounded_by(other) {
                return false;
            }
        }

        return true;
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
    let bound = &Hand {
        red: 12,
        blue: 14,
        green: 13,
    };

    for line in problem_input {
        let line = line.unwrap();
        let game = parse_line(line).unwrap();

        if game.is_bounded_by(bound) {
            sum += game.id;
        }
    }

    println!("\n[AOC 2023 D2P1] The result is: {}", sum)
}
