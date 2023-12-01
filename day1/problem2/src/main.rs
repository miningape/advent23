use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Lines},
    ops::Index,
    str::Chars,
};

fn problem_input_iterator() -> Lines<BufReader<File>> {
    let arguments: Vec<String> = env::args().collect();
    let filepath = &arguments.get(1).unwrap();
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);

    return reader.lines();
}

struct LineReader {
    line: String,
    position: usize,
    size: usize,
}

impl LineReader {
    fn new(line: String) -> LineReader {
        LineReader {
            line: line.clone(),
            size: line.len(),
            position: 0,
        }
    }

    fn is_at_end(&mut self) -> bool {
        self.position >= self.size
    }

    fn get(&mut self) -> Option<char> {
        self.line.chars().nth(self.position)
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn get_advance(&mut self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }

        let char = self.get();
        self.advance();
        char
    }

    fn match_char(&mut self, char: char) -> bool {
        if self.get() == Some(char) {
            self.advance();
            return true;
        }

        return false;
    }

    fn expect(&mut self, char: char) -> Option<char> {
        if self.match_char(char) {
            Some(char)
        } else {
            None
        }
    }

    fn expect_many(&mut self, sequence: &str) -> Option<bool> {
        for char in sequence.chars() {
            self.expect(char)?;
        }

        Some(true)
    }

    fn read_numeric_string(&mut self) -> Option<u32> {
        match self.get_advance() {
            Some('o') => {
                self.expect_many("ne")?;
                Some(1)
            }
            Some('n') => {
                self.expect_many("ine")?;
                Some(9)
            }

            Some('e') => {
                self.expect_many("ight")?;
                Some(8)
            }
            Some('t') => match self.get_advance() {
                Some('h') => {
                    self.expect_many("ree")?;
                    Some(3)
                }
                Some('w') => {
                    self.expect('o')?;
                    Some(2)
                }
                _rest => None,
            },
            Some('s') => match self.get_advance() {
                Some('i') => {
                    self.expect('x')?;
                    Some(6)
                }
                Some('e') => {
                    self.expect_many("ven")?;
                    Some(7)
                }
                _rest => None,
            },

            Some('f') => match self.get_advance() {
                Some('o') => {
                    self.expect_many("ur")?;
                    Some(4)
                }
                Some('i') => {
                    self.expect_many("ve")?;
                    Some(5)
                }
                _rest => None,
            },
            _rest => None,
        }
    }

    pub fn read_next(&mut self) -> Option<u32> {
        let char = self.get()?;

        if char.is_numeric() {
            self.advance();
            return char.to_digit(10);
        }

        let position_before = self.position;

        let rtn = self.read_numeric_string();
        self.position = position_before;
        self.advance();
        rtn
    }
}

fn first_and_last_digit(line: String) -> Result<u32, String> {
    let mut line_reader = LineReader::new(line.clone());

    let mut first: Option<u32> = None;
    let mut last: Option<u32> = None;

    while !line_reader.is_at_end() {
        if first.is_none() {
            let num = line_reader.read_next();
            first = num.clone();
            last = num.clone();
        } else {
            last = match line_reader.read_next() {
                None => last,
                some => some,
            }
        }
    }

    if let Some(f) = first {
        if let Some(l) = last {
            return Ok((f * 10) + l);
        }
    }

    Err(format!("No digits in line: \"{}\"", line))
}

fn main() {
    let problem_input = problem_input_iterator();

    let mut sum = 0;
    for (i, line) in problem_input.enumerate() {
        let line = line.unwrap();

        print!("line: {} - \"{}\"", i, line);
        let value = first_and_last_digit(line).unwrap();
        println!(" = {}", value);

        sum += value;
    }

    println!("\n[AOC 2023 D1P2] The result is: {}", sum)
}
