use std::{io::{BufRead, Seek, BufReader}, fs::File};


crate::main!(1);


fn main_0(input: &mut BufReader<File>) {
    let mut line = String::new();
    let mut current_sum = 0;
    let mut max_sum = 0;
    while let Ok(len) = input.read_line(&mut line) {
        match len {
            0 => {
                println!("part 1: {}", if current_sum > max_sum {current_sum} else {max_sum});
                return;
            },
            1..=2 => {
                max_sum = if current_sum > max_sum {current_sum} else {max_sum};
                current_sum = 0;
            },
            _ => {
                current_sum += line.trim_end().parse::<i32>().unwrap();
            }
        }
        line.clear();
    }
}


fn main_1(input: &mut BufReader<File>) {
    let mut line = String::new();
    let mut current_sum = 0;
    let mut sums = Vec::new();
    while let Ok(len) = input.read_line(&mut line) {
        match len {
            0 => {
                sums.sort_unstable();
                println!("part 2: {}", sums.iter().rev().take(3).sum::<i32>());
                return;
            },
            1..=2 => {
                sums.push(current_sum);
                current_sum = 0;
            },
            _ => {
                current_sum += line.trim_end().parse::<i32>().unwrap();
            }
        }
        line.clear();
    }
}
