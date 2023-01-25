use std::{
    fs::File,
    io::{BufRead, BufReader, Seek},
};

crate::main!(1);

fn main_0(input: &mut BufReader<File>) -> Result<(), std::io::Error> {
    let mut current_sum = 0;
    let mut max_sum = 0;
    for line in input.lines() {
        match line?.as_str() {
            "" => {
                max_sum = if current_sum > max_sum {
                    current_sum
                } else {
                    max_sum
                };
                current_sum = 0;
            }
            num => {
                current_sum += num.parse::<i32>().expect("Expected numbers");
            }
        }
    }
    println!(
        "part 1: {}",
        if current_sum > max_sum {
            current_sum
        } else {
            max_sum
        }
    );
    Ok(())
}

fn main_1(input: &mut BufReader<File>) -> Result<(), std::io::Error> {
    let mut current_sum = 0;
    let mut sums = Vec::new();
    for line in input.lines() {
        match line?.as_str() {
            "" => {
                sums.push(current_sum);
                current_sum = 0;
            }
            num => {
                current_sum += num.parse::<i32>().expect("Expected numbers");
            }
        }
    }
    sums.sort_unstable();
    println!("part 2: {}", sums.iter().rev().take(3).sum::<i32>());
    Ok(())
}
