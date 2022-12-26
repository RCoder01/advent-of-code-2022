use std::{io::{Seek, BufReader, BufRead}, fs::File};


crate::main!(8);

fn main_0(input: &mut BufReader<File>) -> Result<(), std::io::Error> {
    let mut trees = Vec::new();
    let mut visible = Vec::new();
    for line in input.lines() {
        let line = line?;
        // maps b'0' through b'9' to 1u8 through 10u8
        trees.push(line.as_bytes().iter().map(|b| b - 47).collect::<Vec<u8>>());
        visible.push(vec![false; line.len()]);
    }
    for (tree_row, visible_row) in trees.iter().zip(visible.iter_mut()) {
        let mut left = 0;
        for (is_visible, tree) in visible_row.iter_mut().zip(tree_row.iter()) {
            if *tree > left {
                left = *tree;
                *is_visible = true;
            }
        }

        let mut right = 0;
        for (is_visible, tree) in visible_row.iter_mut().rev().zip(tree_row.iter().rev()) {
            if *tree > right {
                right = *tree;
                *is_visible = true;
            }
        }
    }

    for i in 0..trees.first().unwrap().len() {
        let mut top = 0;
        for (tree_row, visible_row) in trees.iter().zip(visible.iter_mut()) {
            if tree_row[i] > top {
                top = tree_row[i];
                visible_row[i] = true;
            }
        }

        let mut bottom = 0;
        for (tree_row, visible_row) in trees.iter().rev().zip(visible.iter_mut().rev()) {
            if tree_row[i] > bottom {
                bottom = tree_row[i];
                visible_row[i] = true;
            }
        }
    }
    println!{"part 1: {}", visible.into_iter().map(|r| r.into_iter().map(|b| b as u16).sum::<u16>()).sum::<u16>()};
    Ok(())
}

fn main_1(input: &mut BufReader<File>) -> Result<(), std::io::Error> {
    let mut trees = Vec::new();
    let mut scenic_score = Vec::new();
    for line in input.lines() {
        let line = line?;
        // maps b'0' through b'9' to 0u8 through 9u8
        trees.push(line.as_bytes().iter().map(|b| b - 48).collect::<Vec<u8>>());
        scenic_score.push(vec![1; line.len()]);
    }
    
    for (tree_row, scenic_row) in trees.iter().zip(scenic_score.iter_mut()) {
        let mut left = ViewDistance::new();
        for (index, (score, tree)) in scenic_row.iter_mut().zip(tree_row.iter()).enumerate() {
            *score *= match left.get(*tree) {
                None => index,
                Some(found) => index - found,
            };
            left.set(*tree, index);
        }
        let mut right = ViewDistance::new();
        for (index, (score, tree)) in scenic_row.iter_mut().rev().zip(tree_row.iter().rev()).enumerate() {
            *score *= match right.get(*tree) {
                None => index,
                Some(found) => index - found,
            };
            right.set(*tree, index);
        }
    }

    for i in 0..trees.first().unwrap().len() {
        let mut top = ViewDistance::new();
        for (index, (tree_row, scenic_row)) in trees.iter().zip(scenic_score.iter_mut()).enumerate() {
            scenic_row[i] *= match top.get(tree_row[i]) {
                None => index,
                Some(found) => index - found,
            };
            top.set(tree_row[i], index);
        }
        let mut bottom = ViewDistance::new();
        for (index, (tree_row, scenic_row)) in trees.iter().rev().zip(scenic_score.iter_mut().rev()).enumerate() {
            scenic_row[i] *= match bottom.get(tree_row[i]) {
                None => index,
                Some(found) => index - found,
            };
            bottom.set(tree_row[i], index);
        }
    }

    println!("part 2: {}", scenic_score.iter().map(|v| v.iter().max().unwrap()).max().unwrap());
    Ok(())
}

#[derive(Debug)]
struct ViewDistance {
    history: [usize; 10],
}

impl ViewDistance {
    fn new() -> Self {
        Self {history: [0; 10]}
    }

    fn get(&self, height: u8) -> Option<usize> {
        match self.history[(height as usize)..].iter().max().unwrap() {
            0 => None,
            num => Some(num - 1),
        }
    }

    fn set(&mut self, height: u8, index: usize) {
        self.history[height as usize] = index + 1;
    }

    // fn get_set(&mut self, height: u8, index: usize) -> usize {
    //     let prior = self.get(height);
    //     self.set(height, index);
    //     prior
    // }
}
