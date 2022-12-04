use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
  let path = "/home/kat/AOC2022/day1/input";
  let input = BufReader::new(File::open(path)?);
  let mut elves : Vec<i32> = Vec::new();
  let mut current_total = 0;

  for line in input.lines() {
    match line {
      Ok(cal) => {
        if cal == "/n" || cal == "" {
          elves.push(current_total);
          current_total = 0;
          continue;
        } else {
          current_total += cal.parse::<i32>().unwrap();
          continue;
        }
      }
      Err(e) => {
        return Err(e);
      }
    }
  }
  elves.sort();
  let first = elves.pop().unwrap();
  let second = elves.pop().unwrap();
  let third = elves.pop().unwrap();
  println!("{}", first);

  println!("{}", first+second+third);

  Ok(())
}
