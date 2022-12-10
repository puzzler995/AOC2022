fn main() {
  let input = include_str!("input").lines().next().unwrap();
  let mut char1: char = '~';
  let mut char2: char = '~';
  let mut char3: char = '~';

  for (loc, data_byte) in input.chars().enumerate() {
    if char1 == '~' {
      char1 = data_byte;
    }
    else if char2 == '~' {
      char2 = data_byte;
    }
    else if char3 == '~' {
      char3 = data_byte;
    }
    else {
      //Check if the character is unique to the current buffer
      if buffer_check(char1, char2, char3, data_byte) {
        println!("{}: {}", loc+1, data_byte);
        break;
      } else { //advance the buffer
        char1 = char2;
        char2 = char3;
        char3 = data_byte;
      }
    }
  }

}

fn buffer_check(char1: char, char2: char, char3: char, char4: char) -> bool{
  if (char1 != char2 && char1 != char3 && char1 != char4) 
  && (char2 != char3 && char2 != char4)
  && (char3 != char4){
    return true;
  }
  false
}