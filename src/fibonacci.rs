pub fn fibonacci(durations: u32) -> Vec<u32>{
  if durations == 0 {
      return vec![];
  } else if durations == 1 {
      return vec![0];
  } else if durations == 2 {
      return vec![0, 1];
  }
  let mut vec = vec![0, 1];
  let mut i = 0;
  while i < durations - 2 {
      let next = vec[vec.len() - 1] + vec[vec.len() - 2];
      vec.push(next);
      i += 1;
  }
  vec
}

#[cfg(test)] 
mod tests {
    use super::*;
    
    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(10), vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }

    #[test]
    fn test_fibonacci_2() {
        assert_eq!(fibonacci(2), vec![0, 1]);
    }

    #[test]
    fn test_fibonacci_1() {
        assert_eq!(fibonacci(1), vec![0]);
    }

    #[test]
    fn test_fibonacci_0() {
        assert_eq!(fibonacci(0), vec![]);
    }
}