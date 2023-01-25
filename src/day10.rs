use std::{
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader, Seek},
    iter::Peekable,
    ops::RangeInclusive,
};

crate::main!(10);

const SIGNALS: [u32; 6] = [20, 60, 100, 140, 180, 220];
fn main_0(input: &mut BufReader<File>) -> Result<(), std::io::Error> {
    let mut strength = 0;
    let mut add_strength = |cpu: &CPU| strength += cpu.cycle as i32 * cpu.register;
    let mut cpu = CallbackCPU::new(SIGNALS.into_iter(), &mut add_strength);
    for line in input.lines() {
        let line = line?;
        let mut instruction = line.split(' ');
        match instruction.next() {
            Some("addx") => {
                cpu.addx(2, instruction.next().unwrap().parse().unwrap());
            }
            Some("noop") => {
                cpu.advance(1);
            }
            _ => {
                panic!();
            }
        }
    }
    println!("part 1: {strength}");
    Ok(())
}

const ONE_OFF: RangeInclusive<i32> = -2..=0;
fn main_1(input: &mut BufReader<File>) -> Result<(), std::io::Error> {
    let mut screen = Vec::new();
    let mut render =
        |cpu: &CPU| screen.push(ONE_OFF.contains(&(cpu.register - (cpu.cycle % 40) as i32)));
    let mut cpu = CallbackCPU::new(1..=240, &mut render);
    for line in input.lines() {
        let line = line?;
        let mut instruction = line.split(' ');
        match instruction.next() {
            Some("addx") => cpu.addx(2, instruction.next().unwrap().parse().unwrap()),
            Some("noop") => {
                cpu.advance(1);
            }
            _ => {
                panic!("Unknown instruction");
            }
        }
    }
    for (index, enabled) in screen.iter().enumerate() {
        if index % 40 == 0 {
            println!();
        }
        print!("{}", if *enabled { "#" } else { "." });
    }
    Ok(())
}

#[derive(Debug)]
struct CPU {
    cycle: u32,
    register: i32,
}

impl CPU {
    fn new() -> Self {
        Self {
            cycle: 0,
            register: 1,
        }
    }

    fn addx(&mut self, delta: i32) {
        self.register += delta;
    }
}

struct CallbackCPU<'a, T: Iterator<Item = u32>> {
    cpu: CPU,
    signals: Peekable<T>,
    callback: &'a mut dyn FnMut(&CPU),
}

impl<T: Iterator<Item = u32> + Debug> Debug for CallbackCPU<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CallbackCPU")
            .field("cpu", &self.cpu)
            .field("signals", &self.signals)
            .field("callback", &"lambda expr")
            .finish()
    }
}

impl<T: Iterator<Item = u32>> CallbackCPU<'_, T> {
    fn new(signals: T, callback: &'_ mut dyn FnMut(&CPU)) -> CallbackCPU<T> {
        CallbackCPU {
            cpu: CPU::new(),
            signals: signals.peekable(),
            callback,
        }
    }

    fn advance(&mut self, cycles: u32) {
        let final_cycle = self.cpu.cycle + cycles;
        while let Some(next_signal) = self.signals.peek() {
            if final_cycle >= *next_signal {
                self.cpu.cycle = *next_signal;
                (self.callback)(&self.cpu);
                self.signals.next();
            } else {
                break;
            }
        }
        self.cpu.cycle = final_cycle;
    }

    fn addx(&mut self, cycles: u32, delta: i32) {
        self.advance(cycles);
        self.cpu.addx(delta);
    }
}
