use std::fs;


fn read_data() -> String {
  return fs::read_to_string("./data/test_data")
        .expect("Should have been able to read the file");
}


fn solve(data: String) -> i32 {
  for line in data.lines() {
    if let Some((a, b)) = line.split_once(' ') {
      println!("{} {}", a, b);
    }
  }
  1;
}


fn main() {
  let test_data: String = read_data();
  let ans = solve(test_data);
  println!("{}", ans);
}
