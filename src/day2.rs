use std::{
    fs::File,
    io::{BufRead, BufReader, Seek},
};

crate::main!(2);

fn main_0(input: &mut BufReader<File>) -> Result<(), std::io::Error> {
    let mut score = 0;
    for line in input.lines() {
        let line = line?;
        score += match line.trim_end() {
            "A X" | "B Y" | "C Z" => 3,
            "A Y" | "B Z" | "C X" => 6,
            "A Z" | "B X" | "C Y" => 0,
            _ => panic!("Undefined sequence"),
        } + match line.as_bytes()[2] {
            b'X' => 1,
            b'Y' => 2,
            b'Z' => 3,
            _ => 0,
        }
    }
    println!("part 1: {score}");
    Ok(())
}

fn main_1(input: &mut BufReader<File>) -> Result<(), std::io::Error> {
    let mut score = 0;
    for line in input.lines() {
        let line = line?;
        score += match line.as_str() {
            "A Y" | "B X" | "C Z" => 1,
            "A Z" | "B Y" | "C X" => 2,
            "A X" | "B Z" | "C Y" => 3,
            _ => panic!("Undefined sequence"),
        } + match line.as_bytes()[2] {
            b'X' => 0,
            b'Y' => 3,
            b'Z' => 6,
            _ => 0,
        }
    }
    println!("part 2: {score}");
    Ok(())
}
