mod day1;

fn main() -> std::io::Result<()> {
    day1::main()?;
    Ok(())
}

mod main {
    use std::{fs::File, io::{BufReader, Error}};
    
    pub fn get_file(day_num: i8) -> Result<std::io::BufReader<std::fs::File>, Error> {
        Ok(BufReader::new(File::open(format!("data/day{day_num}.txt").trim_end())?))
    }
}
