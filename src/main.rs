mod fibonacci;
use fibonacci::fibonacci;

fn main() {
  // TODO: add a test
  fibonacci(10).iter().for_each(|x| println!("{}", x));
}
