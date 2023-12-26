use std::io::{BufRead, BufReader};
use super::utils;

pub fn day1() {
  println!("day1");
  let lines = utils::read_file("src/adventofcode/example1.txt");
  let result = process_line(&lines);
  println!("{:?}", result);
}

fn process_line(lines: &Vec<String>) -> Vec<String> {
  lines.iter()
  .map(
    |l|l.chars()
    .filter(|c| c.is_numeric())
    .flat_map(|c| [c, c])
    .collect::<String>()
  )
  .filter_map(|l| format!("{}{}", l.chars().next()?, l.chars().last()?).into())
  .collect()
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_day1() {
    let lines:Vec<String> = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"]
    .into_iter()
    .map(|s| s.to_string())
    .collect();
    
    let result = process_line(&lines);
    assert_eq!(result, vec!["12", "38", "15", "77"]);
  }
}
