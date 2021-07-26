fn main() {
  let test = String::from("Hello World");
  let ans = first_word(&test);
  println!("{}", ans);
}

fn first_word(s: &String) -> &str {
  let sb = s.as_bytes();
  for (i, &ch) in sb.iter().enumerate() {
    if ch == b' ' {
      return &s[..i]
    }
  }
  &s
}