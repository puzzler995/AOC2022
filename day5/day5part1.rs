use std::collections::VecDeque;

fn main() {
  let input = include_str!("input").lines();
  let mut stack_one: VecDeque<char> = VecDeque::new();
  let mut stack_two: VecDeque<char> = VecDeque::new();
  let mut stack_three: VecDeque<char> = VecDeque::new();
  let mut stack_four: VecDeque<char> = VecDeque::new();
  let mut stack_five: VecDeque<char> = VecDeque::new();
  let mut stack_six: VecDeque<char> = VecDeque::new();
  let mut stack_seven: VecDeque<char> = VecDeque::new();
  let mut stack_eight: VecDeque<char> = VecDeque::new();
  let mut stack_nine: VecDeque<char> = VecDeque::new();
  for line in input {
    match line.chars().nth(0) {
      Some('[') => {
        //if line.chars().nth(0).unwrap() == '['{
          let mut boxes = line.chars();
          let box_one = boxes.nth(1).unwrap();
          let box_two = boxes.nth(3).unwrap();
          let box_three = boxes.nth(3).unwrap();
          let box_four = boxes.nth(3).unwrap();
          let box_five = boxes.nth(3).unwrap();
          let box_six = boxes.nth(3).unwrap();
          let box_seven = boxes.nth(3).unwrap();
          let box_eight = boxes.nth(3).unwrap();
          let box_nine = boxes.nth(3).unwrap();
          if box_one != ' ' {
            stack_one.push_front(box_one);
          }
          if box_two != ' ' {
            stack_two.push_front(box_two);
          }
          if box_three != ' ' {
            stack_three.push_front(box_three);
          }
          if box_four != ' ' {
            stack_four.push_front(box_four);
          }
          if box_five != ' ' {
            stack_five.push_front(box_five);
          }
          if box_six != ' ' {
            stack_six.push_front(box_six);
          }
          if box_seven != ' ' {
            stack_seven.push_front(box_seven);
          }
          if box_eight != ' ' {
            stack_eight.push_front(box_eight);
          }
          if box_nine != ' ' {
            stack_nine.push_front(box_nine);
          }
      }
      Some('m') => {
        let move_qty:i32 = line.split(' ').nth(1).unwrap().parse().unwrap();
        let from_stack:i32 = line.split(' ').nth(3).unwrap().parse().unwrap();
        let to_stack:i32 = line.split(' ').nth(5).unwrap().parse().unwrap();
        for _i in 1..(move_qty+1){
          let mut moving_box:char = ' ';
          match from_stack {
            1 => {moving_box = stack_one.pop_back().unwrap();}
            2 => {moving_box = stack_two.pop_back().unwrap();}
            3 => {moving_box = stack_three.pop_back().unwrap();}
            4 => {moving_box = stack_four.pop_back().unwrap();}
            5 => {moving_box = stack_five.pop_back().unwrap();}
            6 => {moving_box = stack_six.pop_back().unwrap();}
            7 => {moving_box = stack_seven.pop_back().unwrap();}
            8 => {moving_box = stack_eight.pop_back().unwrap();}
            9 => {moving_box = stack_nine.pop_back().unwrap();}
            _ => {}
          }
          match to_stack {
            1 => {stack_one.push_back(moving_box);}
            2 => {stack_two.push_back(moving_box);}
            3 => {stack_three.push_back(moving_box);}
            4 => {stack_four.push_back(moving_box);}
            5 => {stack_five.push_back(moving_box);}
            6 => {stack_six.push_back(moving_box);}
            7 => {stack_seven.push_back(moving_box);}
            8 => {stack_eight.push_back(moving_box);}
            9 => {stack_nine.push_back(moving_box);}
            _ => {}
          }
          
        }
      }
      Some(_) => {}
      None => {}
    }
  }
  let mut answer:String = String::new();
  answer.push(stack_one.pop_back().unwrap());
  answer.push(stack_two.pop_back().unwrap());
  answer.push(stack_three.pop_back().unwrap());
  answer.push(stack_four.pop_back().unwrap());
  answer.push(stack_five.pop_back().unwrap());
  answer.push(stack_six.pop_back().unwrap());
  answer.push(stack_seven.pop_back().unwrap());
  answer.push(stack_eight.pop_back().unwrap());
  answer.push(stack_nine.pop_back().unwrap());
  println!("{}", answer);
}