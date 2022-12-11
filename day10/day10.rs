fn main() {
  let mut input = include_str!("input").lines();
  let mut pending_work = false;
  let mut working = false;
  let mut cycle = 1;
  let mut register:i32 = 1;
  let mut pending:i32 = 0;
  let mut signal_strength = 0;
  loop {
    //check for special cycles
    if cycle==20 || ((cycle-20)%40)==0{
      signal_strength+=cycle*register;
      //println!("Cycle {}:Register {}:Signal Strength {}", cycle, register, signal_strength);
    }
    if !working {
      let instruction = match input.next() {
        Some(i) => i,
        None => {break;}
      };
      let mut split_instruction = instruction.split(" ");
      match split_instruction.next() {
        Some("noop") => {},
        Some("addx") => {
          pending = split_instruction.next().unwrap().parse::<i32>().unwrap();
          pending_work = true;
        }
        Some(_) => {break;}
        None => {break;}
      }
      
    } 
    //CRT Code
    let current_pixel = (cycle-1)%40;
    if (current_pixel == register) || (current_pixel == (register+1)) || (current_pixel == (register-1)){
      print!("#");
    } else {
      print!(".");
    }
    if cycle%40 == 0 {
      print!("\n");
    }

    if working { //complete the addx job
      register+=pending;
      pending_work=false;
    }
    working=pending_work;
    pending_work=false;
    cycle+=1;
  }
  println!("Signal Strength: {}", signal_strength);
}