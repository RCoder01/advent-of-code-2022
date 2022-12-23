use std::{io::{Seek, BufReader, BufRead}, fs::File};


crate::main!(2);


fn main_0(input: &mut BufReader<File>) {
    let mut line = String::new();
    let mut score = 0;
    while let Ok(len) = input.read_line(&mut line) {
        match len {
            0..=2 => {
                println!("part 1: {}", score);
                return;
            },
            _ => {
                score += match line.trim_end() {
                    "A X" | "B Y" | "C Z" => {
                        3
                    },
                    "A Y" | "B Z" | "C X" => {
                        6
                    },
                    "A Z" | "B X" | "C Y" | _ => {
                        0
                    }
                } + match line.as_bytes()[2] {
                    b'X' => {1},
                    b'Y' => {2},
                    b'Z' => {3},
                    _ => {0}
                }
            }
        }
        line.clear();
    }
}


fn main_1(input: &mut BufReader<File>) {
    let mut line = String::new();
    let mut score = 0;
    while let Ok(len) = input.read_line(&mut line) {
        match len {
            0..=2 => {
                println!("part 2: {}", score);
                return;
            },
            _ => {
                score += match line.trim_end() {
                    "A Y" | "B X" | "C Z" => {
                        1
                    },
                    "A Z" | "B Y" | "C X" => {
                        2
                    },
                    "A X" | "B Z" | "C Y" | _ => {
                        3
                    }
                } + match line.as_bytes()[2] {
                    b'X' => {0},
                    b'Y' => {3},
                    b'Z' => {6},
                    _ => {0}
                }
            }
        }
        line.clear();
    }
}
