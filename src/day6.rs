use std::{io::{Seek, BufReader, BufRead}, fs::File, collections::VecDeque};


crate::main!(6);


fn main_0(input: &mut BufReader<File>) -> Result<(), std::io::Error> {
    let mut line = String::new();
    input.read_line(&mut line)?;
    let iter = line.as_bytes().iter().enumerate();
    let mut last = VecDeque::new();
    for (index, char) in iter {
        if last.len() < 3 {
            last.push_back(*char);
            continue;
        }
        last.push_back(*char);
        // dbg!{&last, char};
        if !last.iter().any(|c| last.iter().map(|d| (*d == *c) as u8).sum::<u8>() >= 2) {
            println!("part 1: {}", index + 1);
            break;
        }
        last.pop_front();
    }
    Ok(())
}


fn main_1(input: &mut BufReader<File>) -> Result<(), std::io::Error> {
    let mut line = String::new();
    input.read_line(&mut line)?;
    let iter = line.as_bytes().iter().enumerate();
    let mut last = VecDeque::new();
    for (index, char) in iter {
        if last.len() < 13 {
            last.push_back(*char);
            continue;
        }
        last.push_back(*char);
        // dbg!{&last, char};
        if !last.iter().any(|c| last.iter().map(|d| (*d == *c) as u8).sum::<u8>() >= 2) {
            println!("part 1: {}", index + 1);
            break;
        }
        last.pop_front();
    }
    Ok(())
}