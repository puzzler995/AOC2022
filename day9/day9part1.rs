use std::{borrow::Borrow, fs::File, io::Write};

fn main() {
  let input = include_str!("input");
  let _maxsize = (500,500);
  let mut head_current_x = 250;
  let mut head_current_y = 250;
  let mut tail_current_x = 250;
  let mut tail_current_y = 250;
  let mut spots = 1;
  let mut spot_list:Vec<(usize, usize)> = Vec::new();
  spot_list.push((tail_current_x as usize, tail_current_y as usize));
  for line in input.lines() {
    let mut instructions = line.split(" ");
    let direction = instructions.next().unwrap();
    let distance:usize = instructions.next().unwrap().parse().unwrap();
    for _step in 0..distance {
      match direction {
          "U" => {
            head_current_y+=1;
          }
          "D" => {
            head_current_y-=1;
          }
          "R" => {
            head_current_x+=1;
          }
          "L" => {
            head_current_x-=1;
          }
          _ => {}
      }
      //print_state(maxsize.1, maxsize.0, thisisdumb.borrow(), head_current_x, head_current_y, tail_current_x, tail_current_y);
      if ((head_current_x-tail_current_x)==2) && (head_current_y==tail_current_y) {
        tail_current_x+=1;
      } else if ((tail_current_x-head_current_x)==2) && (head_current_y == tail_current_y){
        tail_current_x-=1;
      } else if ((head_current_y-tail_current_y)==2) && (head_current_x == tail_current_x){
        tail_current_y+=1;
      } else if ((tail_current_y-head_current_y)==2) && (head_current_x == head_current_x) {
        tail_current_y-=1;
      } else if (((head_current_x-tail_current_x)==1) && ((head_current_y-tail_current_y)==2))
            || (((head_current_x-tail_current_x)==2) && ((head_current_y-tail_current_y)==1)){
        tail_current_x+=1;
        tail_current_y+=1;
      } else if (((head_current_x-tail_current_x)==1) && ((tail_current_y-head_current_y)==2))
            || (((head_current_x-tail_current_x)==2) && ((tail_current_y-head_current_y)==1)){
        tail_current_x+=1;
        tail_current_y-=1;
      } else if (((tail_current_x-head_current_x)==1) && ((tail_current_y-head_current_y)==2))
            || (((tail_current_x-head_current_x)==2) && ((tail_current_y-head_current_y)==1)){
        tail_current_x-=1;
        tail_current_y-=1;
      } else if (((tail_current_x-head_current_x)==1) && ((head_current_y-tail_current_y)==2))
            || (((tail_current_x-head_current_x)==2) && ((head_current_y-tail_current_y)==1)){
        tail_current_x-=1;
        tail_current_y+=1;
      }
      let spot_state = spot_list.contains(&(tail_current_x as usize, tail_current_y as usize));
      if !spot_state {
        spots += 1;
        spot_list.push((tail_current_x as usize, tail_current_y as usize));
      }
      //print_state(maxsize.1, maxsize.0, thisisdumb.borrow(), head_current_x, head_current_y, tail_current_x, tail_current_y);
    }
  }
  println!("Spots {}", spots);
}

fn print_state(maxy:usize, maxx:usize, useds:&Vec<(usize, usize)>, headx:i32, heady:i32, tailx: i32, taily: i32) {
  let mut output = File::create("/home/kat/AOC2022/day9/output").expect("create failed");
  for y in 0..maxy {
    let mut opline = String::new();
    for x in 0..maxx{
      if (x==(headx as usize)) && (y==heady as usize) {
        opline.push('H');
      } else if (x==(tailx as usize)) && (y==taily as usize) {
        opline.push('T');
      } else if useds.contains(&(x,y)) {
        opline.push('#');
      }else {
        opline.push('.');
      }
    }
    opline.push_str("\n");
    output.write(opline.as_bytes()).expect("write failed");
  }
  output.write("\n".as_bytes()).expect("write failed");
}