const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
pub fn run() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    // 变量遮蔽 后面声明的变量会遮蔽掉前面声明的
    let y = 18;
    let y = y + 29;
    println!("The value of x is: {}", y);
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);
    println!("The value of x is: {}", THREE_HOURS_IN_SECONDS);
    let _num = 4;
    // num 值 跨不了作用域
    get_num();
    // 结构赋值
    let (a, b, c);
    (a, b) = (1, 2);
    [c, ..] = [3, 4, 5];
    println!("The value of a is: {}", a);
    println!("The value of b is: {}", b);
    println!("The value of c is: {}", c);
}

fn get_num() {}
