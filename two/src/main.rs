use std::fs;

fn read_data() -> String {
    return fs::read_to_string("./data/data").expect("Should have been able to read the file");
}

fn calculate_score(round: &str) -> u32 {
  let hand_shapes = round.split(' ').collect::<Vec<&str>>();
  let [opponents_move, my_move] = <[&str; 2]>::try_from(hand_shapes).ok().unwrap();
  
  let outcome_score = match (opponents_move, my_move) {
    ("A", "X") => 3,
    ("A", "Y") => 6,
    ("A", "Z") => 0,
    ("B", "X") => 0,
    ("B", "Y") => 3,
    ("B", "Z") => 6,
    ("C", "X") => 6,
    ("C", "Y") => 0,
    ("C", "Z") => 3,
    _ => 0,
  };
  
  let shape_score = match my_move {
    "X" => 1,
    "Y" => 2,
    "Z" => 3,
    _ => 0,
  };

  return shape_score + outcome_score;
}

fn main() {
  let data = read_data();
  let total: u32 = data.lines().map(calculate_score).sum();
  println!("{:?}", total);
}
