use std::{io::{Seek, BufReader, BufRead}, fs::File, collections::BTreeSet, ops::{Add, AddAssign, Sub, SubAssign}, fmt::Debug};


crate::main!(9);


fn main_0(input: &mut BufReader<File>) -> Result<(), std::io::Error> {
    let mut rope = Rope::new(2, Position::default());
    let mut positions = BTreeSet::new();
    positions.insert(rope.tail());
    for line in input.lines() {
        let line = line?;
        let mut words = line.split(' ');
        let direction = match words.next().unwrap() {
            "U" => {Direction::Up},
            "D" => {Direction::Down},
            "L" => {Direction::Left},
            "R" => {Direction::Right},
            _ => panic!()
        };
        for _ in 0..words.next().unwrap().parse::<i32>().unwrap() {
            rope.push(direction);
            positions.insert(rope.tail());
        }
    };
    println!("part 1: {}", positions.len());
    Ok(())
}


fn main_1(input: &mut BufReader<File>) -> Result<(), std::io::Error> {
    let mut rope = Rope::new(10, Position::default());
    let mut positions = BTreeSet::new();
    positions.insert(rope.tail());
    for line in input.lines() {
        let line = line?;
        let mut words = line.split(' ');
        let direction = match words.next().unwrap() {
            "U" => {Direction::Up},
            "D" => {Direction::Down},
            "L" => {Direction::Left},
            "R" => {Direction::Right},
            _ => panic!()
        };
        for _ in 0..words.next().unwrap().parse::<i32>().unwrap() {
            rope.push(direction);
            positions.insert(rope.tail());
        }
    };
    println!("part 2: {}", positions.len());
    Ok(())
}


trait Movable {
    fn push(&mut self, direction: Direction);
}


#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
    Center,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self {x, y}
    }

    fn direction(&self) -> Direction {
        match self {
            Position {x:     0, y:   1..} => Direction::Up,
            Position {x:   1.., y:   1..} => Direction::UpRight,
            Position {x:   1.., y:     0} => Direction::Right,
            Position {x:   1.., y: ..=-1} => Direction::DownRight,
            Position {x:     0, y: ..=-1} => Direction::Down,
            Position {x: ..=-1, y: ..=-1} => Direction::DownLeft,
            Position {x: ..=-1, y:     0} => Direction::Left,
            Position {x: ..=-1, y:   1..} => Direction::UpLeft,
            Position {x:     0, y:     0} => Direction::Center,
        }
    }
}

impl From<(i32, i32)> for Position {
    fn from(value: (i32, i32)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Position { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Position {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Movable for Position {
    fn push(&mut self, direction: Direction) {
        *self += match direction {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::UpLeft => (-1, 1),
            Direction::UpRight => (1, 1),
            Direction::DownLeft => (-1, -1),
            Direction::DownRight => (1, -1),
            Direction::Center => (0, 0),
        }.into();
    }
}

#[derive(Debug)]
enum Rope {
    Link(Position, Box<Rope>),
    Knot(Position),
}


impl Rope {
    fn new(num_knots: u32, position: Position) -> Self {
        match num_knots {
            0 => panic!(),
            1 => {
                Self::Knot(position)
            },
            count => {
                Self::Link(position, Box::new(Rope::new(count - 1, position)))
            }
        }
    }

    fn head(&self) -> Position {
        match self {
            Rope::Link(knot, _) | Rope::Knot(knot) => *knot,
        }
    }

    fn tail(&self) -> Position {
        match self {
            Rope::Link(_, rope) => rope.tail(),
            Rope::Knot(knot) => *knot,
        }
    }
}

impl Movable for Rope {
    fn push(&mut self, direction: Direction) {
        match self {
            Rope::Link(knot, link) => {
                let relative = (*knot - link.head()).direction();
                knot.push(direction);
                link.push(match (direction, relative) {
                    (Direction::Up, Direction::UpRight) => Direction::UpRight,
                    (Direction::Up, Direction::Up) => Direction::Up,
                    (Direction::Up, Direction::UpLeft) => Direction::UpLeft,
                    (Direction::Down, Direction::DownLeft) => Direction::DownLeft,
                    (Direction::Down, Direction::Down) => Direction::Down,
                    (Direction::Down, Direction::DownRight) => Direction::DownRight,
                    (Direction::Left, Direction::UpLeft) => Direction::UpLeft,
                    (Direction::Left, Direction::Left) => Direction::Left,
                    (Direction::Left, Direction::DownLeft) => Direction::DownLeft,
                    (Direction::Right, Direction::UpRight) => Direction::UpRight,
                    (Direction::Right, Direction::Right) => Direction::Right,
                    (Direction::Right, Direction::DownRight) => Direction::DownRight,
                    (Direction::UpLeft, Direction::DownLeft) => Direction::Left,
                    (Direction::UpLeft, Direction::UpRight) => Direction::Up,
                    (Direction::UpRight, Direction::DownRight) => Direction::Right,
                    (Direction::UpRight, Direction::UpLeft) => Direction::Up,
                    (Direction::DownRight, Direction::UpRight) => Direction::Right,
                    (Direction::DownRight, Direction::DownLeft) => Direction::Down,
                    (Direction::DownLeft, Direction::UpLeft) => Direction::Left,
                    (Direction::DownLeft, Direction::DownRight) => Direction::Down,
                       (Direction::UpLeft,
                           Direction::Up
                         | Direction::UpLeft
                         | Direction::Left)
                     | (Direction::UpRight,
                           Direction::Up
                         | Direction::UpRight
                         | Direction::Right)
                     | (Direction::DownLeft,
                           Direction::Down
                         | Direction::DownLeft
                         | Direction::Left)
                     | (Direction::DownRight,
                           Direction::Down
                         | Direction::DownRight
                         | Direction::Right) => direction,
                    _ => Direction::Center,
                });
            },
            Rope::Knot(knot) => knot.push(direction),
        }
    }
}