use std::{io::{Seek, BufReader, BufRead}, fs::File, ops::Range};


crate::main!(4);


fn main_0(input: &mut BufReader<File>) -> Result<(), std::io::Error> {
    let mut contains_count = 0;
    for line in input.lines() {
        let line = line?;
        let mut points = line.split(&[',', '-']).map(|int| int.parse::<i32>().unwrap());
        let points = [points.next().unwrap(), points.next().unwrap(), points.next().unwrap(), points.next().unwrap()];
        contains_count += (((points[0] <= points[2]) && (points[3] <= points[1])) || ((points[0] >= points[2]) && (points[3] >= points[1]))) as i32;
    }
    println!("part 1: {}", contains_count);
    Ok(())
}


fn main_1(input: &mut BufReader<File>) -> Result<(), std::io::Error> {
    let mut contains_count = 0;
    for line in input.lines() {
        let line = line?;
        let mut points = line.split(&[',', '-']).map(|int| int.parse::<i32>().unwrap());
        let points = [points.next().unwrap(), points.next().unwrap(), points.next().unwrap(), points.next().unwrap()];
        let range_0 = Range {start: points[0], end: points[1]+1};
        let range_1 = Range {start: points[2], end: points[3]+1};
        contains_count += (range_0.contains(&points[2]) || range_0.contains(&points[3]) || range_1.contains(&points[0]) || range_1.contains(&points[1])) as i32;
    }
    println!("part 2: {}", contains_count);
    Ok(())
}
