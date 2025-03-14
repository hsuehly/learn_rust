use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn run() {
    println!("开始猜数");
    // let mut apples = String::from("hello");
    // let banana = apples;
    // apples = "kkk".to_string();
    // println!("apples {}", apples);
    // println!("banana {}", banana);
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("secret_number {}", secret_number);
    loop {
        let mut guess = String::new();

        if let Err(e) = io::stdin().read_line(&mut guess) {
            println!("Error reading line: {}", e);
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };
        println!("guess: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
