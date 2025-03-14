const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
pub fn run() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    let y = 18;
    let y = y + 29;
    println!("The value of x is: {}", y);
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);
    println!("The value of x is: {}", THREE_HOURS_IN_SECONDS);
    let num = 4;
    // num 值 跨不了作用域
    get_num()
}

fn get_num() {}
