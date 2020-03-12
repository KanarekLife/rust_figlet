use std::collections::HashMap;
use std::io::{BufReader, BufRead};
use std::fs::File;

pub struct Font {
    letter_height: usize,
    letters: HashMap<char, Letter>
}

impl Font {
    pub fn new(mut reader: BufReader<File>) -> Font {
        let letter_height: usize;
        let comment_lines: u64;
        let mut letters: HashMap<char, Letter> = HashMap::new();
        let mut buffer: String = String::new();

        reader.read_line(&mut buffer).expect("Couldn't read the file!");

        letter_height = buffer.split_ascii_whitespace().skip(1).take(1).collect::<String>().parse().expect("Couldn't parse!");

        comment_lines = buffer.split_ascii_whitespace().skip(5).take(1).collect::<String>().parse().expect("Couldn't parse!");

        //Skip comment lines
        for _i in 0..comment_lines {
            buffer.clear();
            reader.read_line(&mut buffer).unwrap();
        }


        for i in 32..126 {
            let mut data: Vec<String> = vec![];
            for j in 0..letter_height {
                buffer.clear();
                reader.read_line(&mut buffer).expect("Couldn't read the line!");
                let mut line = buffer.replace('$', " ");
                if j + 1 == letter_height {
                    if letter_height == 1 {
                        line.remove(line.len()-1);
                        data.push(line);
                    } else {
                        line.remove(line.len()-2);
                        line.remove(line.len()-2);
                        data.push(line);
                    }
                } else {
                    line.remove(line.len()-2);
                    data.push(line);
                }
            }
            let letter = Letter {
                parts: data
            };
            letters.insert(i as u8 as char, letter);
        }
        Font {
            letter_height,
            letters
        }
    }

    pub fn parse_text(&self, text: &str) -> String {
        let mut result = String::new();
        for i in 0..self.letter_height {
            for c in text.chars() {
                let temp = self.letters[&c].parts[i].replace("\n","").clone();
                result.push_str(&temp);
            }
            result.push('\n');
        }
        result
    }
}

pub struct Letter {
    parts: Vec<String>
}