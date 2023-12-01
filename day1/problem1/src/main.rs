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

fn if_numeric_then_char(char: char) -> Option<char> {
    if !char.is_numeric() {
        return None;
    }

    return Some(char);
}

fn first_and_last_digit(line: String) -> Result<u32, String> {
    let mut first: Option<char> = None;
    let mut last: Option<char> = None;

    for char in line.chars() {
        if first.is_none() {
            let char_if_numeric = if_numeric_then_char(char);
            first = char_if_numeric.clone();
            last = char_if_numeric.clone();
        } else {
            last = match if_numeric_then_char(char) {
                None => last,
                some => some,
            };
        }
    }

    if first.is_none() {
        return Err(String::from("Could not detect first numeric value in line"));
    }

    if last.is_none() {
        return Err(String::from("Could not detect last numeric value in line"));
    }

    let number = format!("{}{}", first.unwrap(), last.unwrap());
    number.parse::<u32>().map_err(|err| err.to_string())
}

fn main() {
    let problem_input = problem_input_iterator();

    let mut sum = 0;
    for line in problem_input {
        let line = line.unwrap();
        let value = first_and_last_digit(line).unwrap();
        sum += value;
    }

    println!("\n[AOC 2023 D1P1] The result is: {}", sum)
}
