use std::{io::{Seek, BufReader, BufRead}, fs::File, collections::BTreeSet};


crate::main!(3);


fn main_0(input: &mut BufReader<File>) -> Result<(), std::io::Error> {
    let mut priority = 0;
    for line in input.lines() {
        let line = line?;
        let (comp_1, comp_2) = line.split_at(line.len() / 2);
        let comp_1: BTreeSet<_> = comp_1.as_bytes().iter().collect();
        let comp_2: BTreeSet<_> = comp_2.as_bytes().iter().collect();
        let char = comp_1.intersection(&comp_2).into_iter().next().unwrap();
        priority += match char {
            b'A'..=b'Z' => {
                **char - 64 + 26
            },
            b'a'..=b'z' => {
                **char - 96
            },
            _ => {0}
        } as u32
    }
    println!("part 1: {priority}");
    Ok(())
}


fn main_1(input: &mut BufReader<File>) -> Result<(), std::io::Error> {
    let mut line = String::new();
    let mut lines: Vec<String> = Vec::new();
    let mut priority = 0;
    loop {
        if lines.len() >= 3 {
            let char = lines
                .iter()
                .map(|str| str
                    .trim_end()
                    .as_bytes()
                    .iter()
                    .cloned()
                    .collect::<BTreeSet<_>>())
                .fold(
                    BTreeSet::new(),
                    |accum, item| 
                        if accum.is_empty() {item} else {accum
                            .intersection(&item)
                            .into_iter()
                            .cloned()
                            .collect()})
                .into_iter()
                .next()
                .unwrap();
            priority += match char {
                    b'A'..=b'Z' => {
                        char - 64 + 26
                    },
                    b'a'..=b'z' => {
                        char - 96
                    },
                    _ => {0}
                } as u32;
            lines.clear();
        }
        let len = input.read_line(&mut line)?;
        if len == 0 {break;}
        lines.push(std::mem::take(&mut line));
    }
    println!("part 2: {priority}");
    Ok(())
}
