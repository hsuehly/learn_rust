// use std::net::IpAddr;
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

enum IpAddrKind {
    V4,
    V6,
}
// enum IpAddr {
//     V4(String),
//     V6(String),
// }
enum IpAddr {
    Ipv4,
    Ipv6,
}
// 枚举值通过 :: 来访问成员
#[derive(Debug)]
enum PokeSuit {
    Heart,
    Spade,
    Club,
    Diamond,
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    CbangeColor(i32, i32, i32),
}
// 主要用于函数返回值
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }
// 主要用于值
// enum Option<T> {
//     Some(T),
//     None,
// }
pub fn run() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // let home = IpAddr::V4(String::from("127.0.0.1"));
    //
    // let loopback = IpAddr::V6(String::from("::1"));
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
    if let Some(u) = config_max {
        println!("The maximum is configured to be {}", u);
    }
    println!("{:?}", PokeSuit::Club);
    let m1 = Message::Quit;
    let m2 = Message::Move { x: 21, y: 12 };
    let m3 = Message::CbangeColor(21, 34, 40);
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dire = PokeSuit::Club;
    // match 匹配必须要穷举出所有可能性 可以使用 _来代表剩下的可能性
    // match 的每一个分支都是表达式 所有表达式返回的值类型必须相同
    // x | y 代表或 只需满足一项就可以
    match dire {
        PokeSuit::Club => println!("Club message"),
        PokeSuit::Heart => println!("Heart message"),
        _ => println!("Not a write message"),
    }
    // match 本身也是一个表达式，因此可以用它来赋值：
    let ip1 = IpAddr::Ipv4;
    let ip_str = match ip1 {
        IpAddr::Ipv4 => "127.0.0.1",
        _ => "::1",
    };
    println!("{}", ip_str);

    // 变量遮蔽
    let age = Some(25);
    println!("age {:?}", age);

    if let Some(age) = age {
        println!("age1 {:?}", age)
    }
    println!("age2 {:?}", age);
    let arr: &[u16] = &[114, 514];
    if let [x, ..] = arr {
        assert_eq!(x, &114);
    }
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            println!("new_setting_value:{:?}", new_setting_value);
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let x = 4;
    let y = false;

    match x {
        // 满足 4｜5｜6 并判断y为true
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // 绑定新变量 `p`，同时对 `Point` 进行解构
    let p @ Point { x: px, y: py } = Point { x: 10, y: 23 };
    println!("x: {}, y: {}", px, py);
    println!("{:?}", p);

    let point = Point { x: 10, y: 5 };
    if let p @ Point { x: 10, y } = point {
        println!("x is 10 and y is {} in {:?}", y, p);
    } else {
        println!("x was not 10 :(");
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}
