use multiarray::*;
fn main() {
  let i = include_str!("input").split("\n");
  let input:Vec<&str> = i.clone().collect();
  let width = input.clone().pop().unwrap().len();
  let length = input.len();
  let mut orchard = Array2D::new([width, length], 0);
  for (y, line) in i.enumerate() {
    let chars = line.chars();
    for (x, tree) in chars.enumerate() {
      orchard[[x,y]] = tree.to_digit(10).unwrap();
    }
  }
  let mut visible_count = 0;
  for y in 1..(length-1) {
    for x in 1..(width-1){
      let height = orchard.eliminated_dim(1, y)[x];
      let mut upvisible = true;
      let mut downvisible = true;
      let mut leftvisible = true;
      let mut rightvisible = true;
      //look up and down
      for yy in 0..(y){
        if !upvisible{break;}
        let other_height = orchard.eliminated_dim(1, yy)[x];
        if other_height>=height {upvisible=false;}
      }
      for yy in (y+1)..length{
        if !downvisible{break;}
        let other_height = orchard.eliminated_dim(1, yy)[x];
        if other_height>=height {downvisible=false;}
      }
      //look left and right
      for xx in 0..(x) {
        if !leftvisible{break;}
        let other_height = orchard.eliminated_dim(1, y)[xx];
        if other_height>=height {leftvisible=false;}
      }
      for xx in (x+1)..(width) {
        if !rightvisible{break;}
        let other_height = orchard.eliminated_dim(1, y)[xx];
        if other_height>=height {rightvisible=false;}
      }
      //verify visibility
      if upvisible || downvisible || leftvisible || rightvisible {visible_count=visible_count+1;}
    }
  }
  println!("Visible Trees: {}", visible_count+width+width+length+length-4);
}