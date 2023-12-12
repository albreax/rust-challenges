pub fn fibonacci(durations: u32) -> Vec<u32>{
    let mut vec = vec![0, 1];
    let mut i = 0;
    while i < durations {
        let next = vec[vec.len() - 1] + vec[vec.len() - 2];
        vec.push(next);
        i += 1;
    }
    return vec;
}

#[cfg(test)] 
mod tests {
    use super::*;
    
    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(10), vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89]);
    }
}