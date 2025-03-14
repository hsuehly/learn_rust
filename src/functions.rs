pub fn run() {
    print_labeled_measurement(5, 'h');
    //     语句和表达式
    // Rust 是一门基于表达式（expression-based）的语言
    // 语句（statement）是执行一些操作但不返回值的指令。表达式（expression）计算并产生一个值。
    // rust 所有的参数传递都是传值。
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}
fn plus_one(x: i32) -> i32 {
    x + 1
}
