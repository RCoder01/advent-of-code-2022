use std::{
    fs::File,
    io::{BufRead, BufReader, Seek},
};

crate::main!(5);

fn main_0(input: &mut BufReader<File>) -> Result<(), std::io::Error> {
    let mut line = String::new();
    let mut stack_lines: Vec<String> = Vec::new();
    let mut stacks = vec![Vec::<u8>::new(); 9];
    while let Ok(len) = input.read_line(&mut line) {
        if len <= 2 {
            stack_lines.pop();
            break;
        }
        stack_lines.push(std::mem::take(&mut line));
    }
    for line in stack_lines.iter().rev() {
        let line = line
            .trim_end()
            .as_bytes()
            .iter()
            .skip(1)
            .step_by(4)
            .enumerate();
        for (index, char) in line {
            if *char != b' ' {
                stacks.get_mut(index).unwrap().push(*char);
            }
        }
    }
    for line in input.lines() {
        let nums: Vec<_> = line?
            .split(' ')
            .skip(1)
            .step_by(2)
            .map(|int| int.parse::<usize>().unwrap() - 1)
            .collect();
        for _ in 0..=nums[0] {
            let source = stacks[nums[1]].pop().unwrap();
            stacks[nums[2]].push(source);
        }
    }
    let tops: Vec<_> = stacks.iter_mut().map(|vec| vec.pop().unwrap()).collect();
    println!("part 1: {}", String::from_utf8(tops).unwrap());
    Ok(())
}

fn main_1(input: &mut BufReader<File>) -> Result<(), std::io::Error> {
    let mut line = String::new();
    let mut stack_lines: Vec<String> = Vec::new();
    let mut stacks = vec![Vec::<u8>::new(); 9];
    while let Ok(len) = input.read_line(&mut line) {
        if len <= 2 {
            stack_lines.pop();
            break;
        }
        stack_lines.push(std::mem::take(&mut line));
    }
    for line in stack_lines.iter().rev() {
        let line = line
            .trim_end()
            .as_bytes()
            .iter()
            .skip(1)
            .step_by(4)
            .enumerate();
        for (index, char) in line {
            if *char != b' ' {
                stacks.get_mut(index).unwrap().push(*char);
            }
        }
    }
    for line in input.lines() {
        let nums: Vec<_> = line?
            .split(' ')
            .skip(1)
            .step_by(2)
            .map(|int| int.parse::<usize>().unwrap() - 1)
            .collect();
        let carried = (0..=nums[0])
            .into_iter()
            .map(|_| stacks[nums[1]].pop().unwrap())
            .collect::<Vec<_>>();
        stacks[nums[2]].extend(carried.iter().rev());
    }
    let tops: Vec<_> = stacks.iter_mut().map(|vec| vec.pop().unwrap()).collect();
    println!("part 2: {}", String::from_utf8(tops).unwrap());
    Ok(())
}
