use std::fs;
use std::collections::HashMap;


fn read_data() -> String {
  return fs::read_to_string("./data/data")
        .expect("Should have been able to read the file");
}


fn solve(data: String) -> i32 {
  let mut elf_cals: HashMap<i32, i32> = HashMap::new();
  let mut current_elf_cals: i32 = 0;
  let mut elf_num: i32 = 1;
  for line in data.lines() {
    if line == "" {
      elf_cals.insert(elf_num, current_elf_cals);
      current_elf_cals = 0;
      elf_num += 1;
    }
    else {
      current_elf_cals += line.parse::<i32>().unwrap();
    }
  }
  return *elf_cals.values().max().unwrap();
}


fn main() {
  let test_data: String = read_data();
  let ans = solve(test_data);
  println!("{} is the largest amount of calories carried by an elf.", 
  ans);
}
