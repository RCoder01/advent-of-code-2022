use std::{io::{Seek, BufReader, BufRead}, fs::File, any::type_name, collections::VecDeque, rc::Rc};
use core::mem::size_of_val;


crate::main!(11);

const INTS: [i32; 4] = [1, 2, 3, 4];
fn main_0(input: &mut BufReader<File>) -> Result<(), std::io::Error> {
    Ok(())
}

fn main_1(input: &mut BufReader<File>) -> Result<(), std::io::Error> {
    Ok(())
}


#[derive(Debug)]
struct Item {
    worry_level: u32,
}

impl From<u32> for Item {
    fn from(value: u32) -> Self {
        Self {worry_level: value}
    }
}


#[derive(Debug)]
struct Monkey<O, T>
where
    O: FnMut(&mut Item),
    T: FnMut(Item),
{
    items: VecDeque<Item>,
    operation: O,
    test: T,
}


impl<O, T> Monkey<O, T> 
where
    O: FnMut(&mut Item),
    T: FnMut(Item)
{
    fn new(items: impl Iterator<Item=Item>, operation: O, test: T) -> Self {
        Self {items: items.collect(), operation, test}
    }

    fn turn(&mut self) {
        while let Some(mut item) = self.items.pop_front() {
            (self.operation)(&mut item);
            item.worry_level /= 3;
            (self.test)(item);
        }
    }

    fn give_item(&mut self, item: Item) {
        self.items.push_back(item);
    }
}


#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
struct MonkeyInfo {
    start_items: Vec<u32>,
    operation: Operation,
    operation_amount: u32,
    test_div_num: u32,
    throw_on_true: usize,
    throw_on_false: usize,
}

#[derive(Debug)]
struct Monkeys {
    // items: Vec<Monkey>,
}


impl Monkeys {
    fn new(monkey_infos: Vec<MonkeyInfo>) -> Self {
        let mut monkeys = Rc::new(Vec::new());
        let monkey_clone = Rc::clone(&monkeys);
        let monkey_ref = Rc::get_mut(&mut monkeys).unwrap();
        for monkey in monkey_infos {
            let monkey_clone_2 = Box::new(Rc::clone(&monkey_clone));
            monkey_ref.push(Monkey::new(
                monkey.start_items.iter().map(|n| (*n).into()),
                move |item| item.worry_level = Self::operation(item.worry_level, monkey.operation_amount, monkey.operation),
                move |item| {let i = monkey_clone_2; Rc::get_mut(&mut i).unwrap(); ()}
            ));
        };
        Self {}
    }

    fn operation(a: u32, b: u32, op: Operation) -> u32 {
        match op {
            Operation::Add => a + b,
            Operation::Subtract => a - b,
            Operation::Multiply => a * b,
            Operation::Divide => a / b,
        }
    }
}
