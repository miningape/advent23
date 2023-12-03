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
    fn is_adjacent_to_point(&self, point_x: usize, point_y: usize) -> bool {
        for y_offset in -1..=1 {
            let y = point_y as isize + y_offset;

            if y != self.line {
                continue;
            }

            for x_offset in -1..=1 {
                let x = point_x as isize + x_offset;

                if x >= self.char && x < self.char + self.length {
                    return true;
                }
            }
        }

        false
    }
}

fn find_numbers(file: &Vec<String>) -> Result<Vec<Number>, String> {
    let line_width = file.get(0).ok_or("Cannot detect line width")?.len() as isize;
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

fn find_gears(file: &Vec<String>) -> Vec<(usize, usize)> {
    let mut gears: Vec<(usize, usize)> = Vec::new();

    for (line_number, line) in file.iter().enumerate() {
        for (char_number, char) in line.chars().enumerate() {
            if char == '*' {
                gears.push((char_number, line_number))
            }
        }
    }

    gears
}

fn main() {
    let file = problem_input_vector().unwrap();
    let numbers = find_numbers(&file).unwrap();
    let gears = find_gears(&file);

    println!(
        "\n[AOC 2023 D3P2] The result is: {}",
        gears
            .iter()
            .map(|(x, y)| {
                numbers
                    .iter()
                    .filter(|number| number.is_adjacent_to_point(*x, *y))
                    .collect()
            })
            .filter(|numbers: &Vec<&Number>| numbers.len() == 2)
            .map(|numbers| {
                numbers
                    .iter()
                    .map(|number| number.source)
                    .reduce(|acc, cur| acc * cur)
                    .unwrap() // We know there are 2 numbers
            })
            .reduce(|acc, cur| acc + cur)
            .unwrap()
    )
}
