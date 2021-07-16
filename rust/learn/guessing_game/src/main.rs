use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number.");
    let ans = rand::thread_rng().gen_range(1..101);
    println!("ans is {}", ans);
    loop {
        println!("Please enter your result!");
        let mut res = String::new();

        io::stdin()
            .read_line(&mut res)
            .expect("Failed to read line");
        
        let res : u32 = match res.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match res.cmp(&ans) {
            Ordering::Less => println!("less!"),
            Ordering::Equal => {
                println!("Correct!"); 
                break;
            },
            Ordering::Greater => println!("greater!")
        }

        println!("You guess {}", res);
    }
}
