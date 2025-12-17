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
    {
        let mut ec = 10;
        ec = 20;
        dbg!(ec);
        let ae = ec;
        println!("The value of ec is: {}", ec);
        println!("The value of ae is: {}", ae);
    }
    {
        let mut str = String::from("value");

        str.push_str("nihao");

        let str1 = str;
        // println!("The value of str is: {}", str);
        println!("The value of str is: {}", str1);

        let mut sst = String::from("value");
        println!("The value of sst is: {}", sst);
        // let sst2 = &sst;

        let sst1 = &mut sst;
        // let sst2 = &mut sst;

        sst1.push_str("--value");
        // sst2.push_str("--");
        // println!("The value of sst is: {}", sst);
        println!("The value of sst1 is: {}", sst1);
    }
}

fn get_num() {}
