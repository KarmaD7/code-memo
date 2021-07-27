use std::collections::HashMap;

fn main() {
  // let mut v = vec![1, 2, 3];
  // v.push(7);
  // let mut c = v;
  // println!("{}", c[3]);
  // for i in &mut c {
  //   *i += 100;
  // }
  // for i in &c {
  //   println!("{}", i);
  // }
  let s1 = String::from("tic");
  let s2 = String::from("tac");
  let s3 = String::from("toe");

  let s = format!("{}-{}-{}", s1, s2, s3);
  for c in "नमस्ते".chars() {
    println!("{}", c);
  }

  // let teams  = vec![String::from("Blue"), String::from("Yellow")];
  // let initial_scores = vec![10, 50];
  // let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
}