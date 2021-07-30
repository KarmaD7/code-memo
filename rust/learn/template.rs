fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
  let mut largest = list[0];
  for &item in list.iter() {
    if item > largest {
      largest = item
    }
  }
  largest
}

pub trait Summary {
  fn summarize(&self) -> String;
}

fn main() {
  let a: [i32; 5] = [1, 2, 3, 4, 5];
  let b = largest(&a);
  println!("{}", b);
}