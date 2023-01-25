mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;

fn main() -> std::io::Result<()> {
    day1::main()?;
    day2::main()?;
    day3::main()?;
    day4::main()?;
    day5::main()?;
    day6::main()?;
    day7::main()?;
    day8::main()?;
    day9::main()?;
    day10::main()?;
    Ok(())
}


mod main {
    use std::{fs::File, io::{BufReader, Error}};
    
    pub fn get_file(day_num: i8, test: Option<u32>) -> Result<std::io::BufReader<std::fs::File>, Error> {
        Ok(BufReader::new(File::open(format!(
            "data/day{day_num}{}.txt",
            if let Some(index) = test {format!("_test{index}")} else {String::new()}
        ).trim_end())?))
    }

    #[macro_export]
    macro_rules! main {
        ($day_num: expr) => {
            pub fn main() -> std::io::Result<()> {
                println!("day {}: ", $day_num);
                let mut input = $crate::main::get_file($day_num, None)?;
                main_0(&mut input)?;
                input.rewind().unwrap();
                main_1(&mut input)?;
                Ok(())
            }

            pub fn test(index: u32) -> std::io::Result<()> {
                println!("day {}: ", $day_num);
                let mut input = $crate::main::get_file($day_num, Some(index))?;
                main_0(&mut input)?;
                input.rewind().unwrap();
                main_1(&mut input)?;
                Ok(())
            }
        };
    }
}
