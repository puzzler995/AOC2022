use std::{collections::HashSet};
use std::io;

type Coords = (i64, i64);

fn main() {
  let input: Vec<String> = io::stdin().lines().flatten().collect();

  let mut rope = vec![(0,0); 2];
  println!("{}", moving(&input, &mut rope));

  let mut rope = vec![(0,0); 10];
  println!("{}", moving(&input, &mut rope));
  
}

fn moving(moves: &[String], rope: &mut [Coords]) -> usize {
  let mut spot_list: HashSet<Coords> = HashSet::new();
  for mov in moves {
    let (x, y, d) = match mov.split_at(2) {
        ("R ", d) => (1, 0, d),
        ("L ", d) => (-1, 0, d),
        ("U ", d) => (0, 1, d),
        ("D ", d) => (0, -1, d),
        (_, _) => unreachable!(),  
    };
    for _ in 0..d.parse::<usize>().unwrap() {
      rope[0].0 += x;
      rope[0].1 += y;
      for knot in 1..rope.len() {
        if let Some(coor) = adjust(&rope[knot], &rope[knot-1]) {
          rope[knot] = coor;
        } else {
          break;
        }
        
      }
      spot_list.insert(*rope.last().unwrap());
    }   
  }
  spot_list.len()
}
fn adjust(tail: &Coords, head: &Coords) -> Option<Coords> {
  let delta_x = tail.0 - head.0;
  let delta_y = tail.1 - head.1;

  if (delta_x == 2 || delta_x == -2) && (delta_y == 2 || delta_y == -2) {
    Some((head.0 + delta_x.clamp(-1, 1), head.1 + delta_y.clamp(-1, 1)))
  } else if delta_x == 2 || delta_x == -2 {
    Some((head.0 + delta_x.clamp(-1, 1), head.1))
  } else if delta_y == 2 || delta_y == -2 {
    Some((head.0, head.1 + delta_y.clamp(-1, 1)))
  } else {
      None
  }
}