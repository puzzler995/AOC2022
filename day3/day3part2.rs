use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn find_match(sack_one:HashSet<char>, sack_two:HashSet<char>, sack_three:HashSet<char>) -> char{
    for item_one in &sack_one {
        for item_two in &sack_two {
            for item_three in &sack_three {
              if item_one == item_two && item_two == item_three {
                return *item_one;
              }
            }
        }
    }
    return 'a';
}

fn main() -> Result<(), Error> {
    let priorities = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z','A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];
    let path = "/home/kat/AOC2022/day3/input";
    let input = BufReader::new(File::open(path)?);
    let mut sacks:VecDeque<String> = VecDeque::new();

    let mut total_priority = 0;
    for line in input.lines() {
        match line {
            Ok(sack) => {
              sacks.push_back(sack);
                // let sack = sack.replace("\n", "");
                // let splitp = sack.chars().count()/2;
                // let compartment_one: HashSet<char> = (&sack[0..splitp]).chars().collect();
                // let compartment_two: HashSet<char> = (&sack[splitp..]).chars().collect();
                // let common = find_match(compartment_one, compartment_two);
                // total_priority += priorities.iter().position(|&x| x == common).unwrap()+1;
                // println!("{}", common);
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
    while !sacks.is_empty(){
      let sacka = sacks.pop_front().unwrap();
      let sackb = sacks.pop_front().unwrap();
      let sackc = sacks.pop_front().unwrap();
      let common = find_match(sacka.chars().collect(), sackb.chars().collect(), sackc.chars().collect());
      total_priority += priorities.iter().position(|&x| x == common).unwrap()+1;
    }
    println!("{}", total_priority);
    Ok(())
}