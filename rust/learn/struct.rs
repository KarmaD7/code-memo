#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }
  fn square(sz: u32) -> u32 {
    sz * sz
  }
}


fn main() {
  let rec = Rectangle { width: 40, height: 50 };
  let area = rec.area();
  let square = Rectangle::square(9);
  println!("{}", square);
}