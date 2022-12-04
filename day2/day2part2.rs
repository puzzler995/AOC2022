use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let path = "/home/kat/AOC2022/day2/input";
    let input = BufReader::new(File::open(path)?);
    let mut my_score = 0;
    let mut bad_score = 0;

    for line in input.lines() {
        match line {
            Ok(line) => {
                let mut moves = line.chars();
                let bad_move = moves.nth(0).unwrap();
                let my_move = moves.nth(1).unwrap();
                match bad_move{
                    'A'=> {
                        match my_move {
                            'X'=> {
                                my_score += 3;
                                bad_score += 7;
                            }
                            'Y' => {
                                my_score += 4;
                                bad_score += 4;
                            }
                            'Z' => {
                                my_score += 8;
                                bad_score += 1;
                            }
                            _ => {}
                        }
                    }
                    'B'=> {
                        match my_move {
                            'X' => {
                                my_score += 1;
                                bad_score += 8;
                            }
                            'Y' => {
                                my_score += 5;
                                bad_score += 5;
                            }
                            'Z' => {
                                my_score += 9;
                                bad_score += 2;
                            }
                            _ => {}
                        }
                    }
                    'C'=> {
                        match my_move {
                            'X' => {
                                my_score += 2;
                                bad_score += 9;
                            }
                            'Y' => {
                                my_score += 6;
                                bad_score += 6;
                            }
                            'Z' => {
                                my_score += 7;
                                bad_score += 3;
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }

            }
            Err(e) => {
                return Err(e);
            }
        }
    }
    println!("My Score: {}", my_score);
    println!("Their Score: {}", bad_score);
    Ok(())
}