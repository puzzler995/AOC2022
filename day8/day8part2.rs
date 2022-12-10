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
  let mut best_scenic_score = 0;
  for y in 0..(length) {
    for x in 0..(width){
      let height = orchard.eliminated_dim(1, y)[x];
      let mut upvisible = true;
      let mut downvisible = true;
      let mut leftvisible = true;
      let mut rightvisible = true;
      let mut up_scenic_score = 0;
      let mut down_scenic_score = 0;
      let mut left_scenic_score = 0;
      let mut right_scenic_score = 0;
      //look up and down
      for yy in (0..y).rev(){
        if !upvisible{break;}
        let other_height = orchard.eliminated_dim(1, yy)[x];
        if other_height>=height {
          upvisible = false;
          up_scenic_score += 1;
        } else {
          up_scenic_score += 1;
        }
      }
      for yy in (y+1)..length{
        if !downvisible{break;}
        let other_height = orchard.eliminated_dim(1, yy)[x];
        if other_height>=height {
          downvisible = false;
          down_scenic_score += 1;
        } else {
          down_scenic_score += 1;
        }
      }
      //look left and right
      for xx in (0..x).rev() {
        if !leftvisible{break;}
        let other_height = orchard.eliminated_dim(1, y)[xx];
        if other_height>=height {
          leftvisible = false;
          left_scenic_score += 1;
        } else {
          left_scenic_score += 1;
        }
      }
      for xx in (x+1)..width {
        if !rightvisible{break;}
        let other_height = orchard.eliminated_dim(1, y)[xx];
        if other_height>=height {
          rightvisible = false;
          right_scenic_score += 1;
        } else {
          right_scenic_score += 1;
        }
      }
      let scenic_score = up_scenic_score*down_scenic_score*left_scenic_score*right_scenic_score;
      //see if it's the best
      if best_scenic_score<scenic_score{
        best_scenic_score=scenic_score;
      }
    }
  }
  println!("Best Score: {}", best_scenic_score);
}