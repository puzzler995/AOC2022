fn main(){
  let input = include_str!("input").lines();
  let mut total_full_overlaps = 0;
  let mut total_overlaps = 0;

  for line in input {
    let mut pairings_list = line.split(|x| x == ',' || x == '-');
    let pair_one_first:i32 = pairings_list.next().unwrap().parse().unwrap();
    let pair_one_second:i32 = pairings_list.next().unwrap().parse().unwrap();
    let pair_two_first:i32 = pairings_list.next().unwrap().parse().unwrap();
    let pair_two_second:i32 = pairings_list.next().unwrap().parse().unwrap();
    
    if (pair_one_first<=pair_two_first && pair_one_second >= pair_two_second) || (pair_one_first >=pair_two_first && pair_one_second<=pair_two_second){
      total_full_overlaps += 1;
    }
    if pair_one_second>=pair_two_first &&  pair_one_first<=pair_two_second {
      total_overlaps += 1;
    }
  }

  println!("Total Full Overlaps: {}", total_full_overlaps);
  println!("Total Overlaps: {}", total_overlaps);
}
