use std::{io::{Seek, BufReader, BufRead}, fs::File, collections::BTreeSet};


crate::main!(9);


fn main_0(input: &mut BufReader<File>) -> Result<(), std::io::Error> {
    let mut rope = Rope::new();
    let mut positions = BTreeSet::new();
    positions.insert(rope.tail);
    for line in input.lines() {
        let line = line?;
        let mut words = line.split(' ');
        let direction = match words.next().unwrap() {
            "U" => {Direction::Up},
            "D" => {Direction::Down},
            "L" => {Direction::Left},
            "R" | _ => {Direction::Right},
        };
        for _ in 0..words.next().unwrap().parse::<i32>().unwrap() {
            rope.pull(direction);
            positions.insert(rope.tail);
        }
    };
    println!("part 1: {}", positions.len());
    Ok(())
}


fn main_1(input: &mut BufReader<File>) -> Result<(), std::io::Error> {
    Ok(())
}


#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rope {
    head: Position,
    tail: Position,
}

impl Rope {
    fn new() -> Self {
        Self { head: Position { x: 0, y: 0 }, tail: Position { x: 0, y: 0 } }
    }

    fn pull(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {self.up()},
            Direction::Down => {self.down()},
            Direction::Left => {self.left()},
            Direction::Right => {self.right()},
        }
    }

    fn up(&mut self) {
        if self.tail.y < self.head.y {
            self.tail = self.head;
        }
        self.head.y += 1;
    }

    fn down(&mut self) {
        if self.tail.y > self.head.y {
            self.tail = self.head;
        }
        self.head.y -= 1;
    }

    fn left(&mut self) {
        if self.tail.x > self.head.x {
            self.tail = self.head;
        }
        self.head.x -= 1;
    }

    fn right(&mut self) {
        if self.tail.x < self.head.x {
            self.tail = self.head;
        }
        self.head.x += 1;
    }
}
