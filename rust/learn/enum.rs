#[derive(Debug)]
enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}

impl Message {
  fn call(&self) {
      // 在这里定义方法体
      println!("{:?}", self);
  }
}

enum Coin {
  One, 
  Five,
  Fifty,
  OneHundred,
}

fn value(coin: Coin) -> u8 {
  match coin {
    Coin::One => 1,
    Coin::Five => 5,
    Coin::Fifty => 50,
    Coin::OneHundred => 100,
  }
}

fn main() {
  let msg = Message::Write(String::from("dsf19"));
  msg.call();
}