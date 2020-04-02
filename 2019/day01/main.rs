use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error>{
    let path = "input.txt";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let sum = buffered.lines().fold(0, |acc, line| acc + (line.unwrap_or(String::from("0")).parse::<i32>().unwrap_or(0) / 3) - 2);
    println!("{}", sum);

    Ok(())
}
