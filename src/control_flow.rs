pub fn run() {
    // if 表达式 代码中的条件 必须 是 bool 值。如果条件不是 bool 值， 会得到一个错误

    let number = 6;
    // 只会输出第一个为true的分支
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // 在 let 语句中使用 if

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
    // loop循环
    // 从循环返回值
    let mut counter = 0;

    // loop 是一个表达式，因此可以返回一个值
    let result = loop {
        counter += 1;
        // break 可以单独使用，也可以带一个返回值，有些类似 return
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    //循环标签：在多个循环之间消除歧义
    //
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // while 条件循环 当条件为 true，执行循环。当条件不再为 true，调用 break 停止循环。
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
    // for 循环会消耗所有权 一般使用引用 .iter() 不可变借用   .iter_mut() 可变借用
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
