use super::utils;

const RED: u8 = 12;
const GREEN: u8 = 13;
const BLUE: u8 = 14;

pub fn day2() {
  println!("day2");
  let lines = utils::read_file("src/adventofcode/example2.txt");
  let result = process_line(&lines);
  println!("{:?}", sum_up(result));
}

fn sum_up(vec: Vec<u8>) -> u16 {
  vec.iter().map(|&v| v as u16).sum()
}

fn process_line(lines: &Vec<String>) -> Vec<u8> {
  lines.iter().map(|line| {
    let parts = line.split(" ").collect::<Vec<&str>>();
    let game:u8 = parts[1].replace(':',"").parse().unwrap();
    
    let mut num:u8 = 0;
    let mut is_valid = true;

    parts.iter().skip(2).for_each(|p|{
      let part = p.replace(',',"")
        .replace(';',"")
        .replace(':', "");

      if let Some(n) = part.parse().ok() {
        num = n;
      } else {
        if 
          part == "red" && num > RED || 
          part == "green" && num > GREEN || 
          part == "blue" && num > BLUE 
        { is_valid = false; }
      }
    });

    match is_valid {
        true => game,
        false => 0,
    }
  }).collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_process_line() {
    let lines = vec![
      String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
      String::from("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
      String::from("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"),
      String::from("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"),
      String::from("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green")
    ];
    let result = process_line(&lines);
    assert_eq!(result, vec![1, 2, 0, 0, 5]);
  }

  #[test]
  fn test_sum_up() {
    let vec = vec![1, 2, 0, 0, 5];
    let result = sum_up(vec);
    assert_eq!(result, 8);
  }
}