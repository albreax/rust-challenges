pub fn fibonacci(to: u32) -> Vec<u32>{
    let mut vec = vec![0, 1];
    let mut i = 0;
    while i < to {
        let next = vec[vec.len() - 1] + vec[vec.len() - 2];
        vec.push(next);
        i += 1;
    }
    return vec;
}