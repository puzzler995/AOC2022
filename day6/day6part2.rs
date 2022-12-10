use std::collections::VecDeque;

fn main() {
  let input = include_str!("input").lines().next().unwrap();
  let mut buffer : VecDeque<char> = VecDeque::new();
  
  for (loc, data_byte) in input.chars().enumerate() {
    if buffer.len() < 14 {
      buffer.push_back(data_byte);
    } else {
      buffer.pop_front();
      buffer.push_back(data_byte);
      if check_unique(buffer.clone()) {
        println!("{}: {}", loc+1, data_byte);
        break;
      } else {
      }
    }
  }
}

fn check_unique(mut buf: VecDeque<char>) -> bool{
  while buf.len()>0 {
    let curr = buf.pop_front().unwrap();
    if buf.contains(&curr){return false;}
    
  }
  true
}