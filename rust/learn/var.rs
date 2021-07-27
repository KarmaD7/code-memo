fn main() {
  let mut a = [2; 5];
  a[0] = 3;
  let b = a;
  println!("{ }", a[0]);
}