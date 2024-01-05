use super::utils;

const RED: u8 = 12;
const GREEN: u8 = 13;
const BLUE: u8 = 14;

pub fn day2() {
  println!("day2");
  let lines = utils::read_file("src/adventofcode/example2.txt");
  let result = line_number(&lines);
  println!("{:?}", result.iter().map(|&x| x as u32).sum::<u32>());
  println!("{:?}", line_power(&lines).iter().sum::<u32>());
}

fn line_to_parts(line: &str) -> Vec<&str> {
  line.split(" ").collect::<Vec<&str>>()
}

fn clean_part(part: &str) -> String {
  part.replace(',',"")
    .replace(';',"")
    .replace(':', "")
}

fn line_number(lines: &Vec<String>) -> Vec<u8> {
  lines.iter().map(|line| {
    let parts = line_to_parts(line);
    let game:u8 = parts[1].replace(':',"").parse().unwrap();
    
    let (is_valid, _) = parts.iter().skip(2).fold((true, 0), |(is_valid,num), p|{
      let part = clean_part(p);

      if let Some(n) = part.parse().ok() {
        return (is_valid, n);
      }

      match  
          part == "red" && num > RED || 
          part == "green" && num > GREEN || 
          part == "blue" && num > BLUE 
      {
        true => return (false, num),
        false => return (is_valid, num),
      }
    });

    match is_valid {
        true => game,
        false => 0,
    }
  }).collect()
}

fn line_power(lines: &Vec<String>) -> Vec<u32> {
  lines.iter().map(|line| {
    let parts = line_to_parts(line);
    
    
    let (r, g, b, _) = parts.iter().skip(2).fold((0, 0, 0, 0), |(r, g, b, n), p| {
      let part = clean_part(p);
      
      if let Some(n) = part.parse().ok() {
        return (r, g, b, n);
      }

      let acc = (r, g, b, n);

      match part.as_str() {
        "red" => if r < n { (n, g, b, 0) } else { acc },
        "green" => if g < n { (r, n, b, 0) } else { acc },
        "blue" => if b < n { (r, g, n, 0) } else { acc },
        _ => panic!("Invalid color")
      }
    });
    r as u32 * g as u32 * b as u32
  }).collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_line_number() {
    let lines = vec![
      String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
      String::from("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
      String::from("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"),
      String::from("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"),
      String::from("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green")
    ];
    let result = line_number(&lines);
    assert_eq!(result, vec![1, 2, 0, 0, 5]);
  }

  #[test]
  fn test_sum_up() {
    let vec = vec![1, 2, 0, 0, 5];
    assert_eq!(vec.iter().map(|&x| x as u32 ).sum::<u32>(), 8);
  }
  
  #[test]
  fn test_line_power() {
    let lines = vec![
      String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
      String::from("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
      String::from("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"),
      String::from("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"),
      String::from("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green")
    ];
    let result = line_power(&lines);
    assert_eq!(result, vec![48, 12, 1560, 630, 36]);
  }
}