mod fibonacci;
use fibonacci::fibonacci;

fn main() {
  fibonacci(10).iter().for_each(|x| println!("{}", x));
}
