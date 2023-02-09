use std::fs;
use std::collections::HashMap;


fn read_data() -> String {
  return fs::read_to_string("./data/test_data")
        .expect("Should have been able to read the file");
}


fn solve(data: String) -> i32 {
}


fn main() {
  let test_data: String = read_data();
  let ans = solve(test_data);
  println!("{}", ans);
}
