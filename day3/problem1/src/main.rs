use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Error, Lines},
    num::ParseIntError,
};

fn problem_input_iterator() -> Lines<BufReader<File>> {
    let arguments: Vec<String> = env::args().collect();
    let filepath = &arguments.get(1).unwrap();
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);

    return reader.lines();
}

fn problem_input_vector() -> Result<Vec<String>, Error> {
    let mut vec: Vec<String> = Vec::new();

    for line in problem_input_iterator() {
        vec.push(line?);
    }

    Ok(vec)
}

#[derive(Debug)]
struct Number {
    line: isize,
    char: isize,
    length: isize,
    source: isize,
}

impl Number {
    fn is_adjacent(&self, file: &Vec<String>, line_width: isize) -> bool {
        for y_offset in -1..=1 {
            let y = self.line + y_offset;

            if y < 0 || y >= file.len() as isize {
                continue;
            }

            for x_offset in -1..=self.length {
                let x = self.char + x_offset;

                if y_offset == 0 && x_offset >= 0 && x_offset < self.length {
                    continue;
                }

                if x < 0 || x >= line_width {
                    continue;
                }

                let char = file
                    .get(y as usize)
                    .unwrap()
                    .chars()
                    .nth(x as usize)
                    .unwrap();
                if !char.is_numeric() && char != '.' {
                    return true;
                }
            }
        }

        false
    }
}

fn find_numbers(file: &Vec<String>, line_width: isize) -> Result<Vec<Number>, String> {
    let mut numbers: Vec<Number> = Vec::new();

    for (line_number, line) in file.iter().enumerate() {
        let mut current_number = String::new();

        for (char_number, char) in line.chars().enumerate() {
            if char.is_numeric() {
                current_number.push(char);
            } else if current_number.len() > 0 {
                let length = current_number.len() as isize;
                numbers.push(Number {
                    length,
                    line: line_number as isize,
                    char: char_number as isize - length,
                    source: current_number
                        .parse()
                        .map_err(|err: ParseIntError| err.to_string())?,
                });

                current_number.clear();
            }
        }

        if current_number.len() > 0 {
            let length = current_number.len() as isize;
            numbers.push(Number {
                length,
                line: line_number as isize,
                char: line_width - 1 - length,
                source: current_number
                    .parse()
                    .map_err(|err: ParseIntError| err.to_string())?,
            });
        }
    }

    Ok(numbers)
}

fn main() {
    let file = problem_input_vector().unwrap();
    let line_width = file.get(0).ok_or("Cannot detect line width").unwrap().len() as isize;
    let numbers = find_numbers(&file, line_width).unwrap();

    println!(
        "\n[AOC 2023 D3P1] The result is: {}",
        numbers
            .iter()
            .filter(|number| number.is_adjacent(&file, line_width))
            .map(|number| number.source)
            .reduce(|acc, cur| acc + cur)
            .unwrap()
    )
}
