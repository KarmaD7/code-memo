use std::io;

fn main() {
  let mut a = 1;
  let mut b = 1;
  println!("{}", a);
  println!("{}", b);
  for i in (3..10) {
    let c = a + b;
    a = b;
    b = c;
    println!("{}", b);
  }
}