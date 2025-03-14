use std::thread;
use std::time::Duration;

pub fn run() {
    //闭包：可以捕获环境的匿名函数 闭包允许捕获其被定义时所在作用域中的值
    {
        let store = Inventory {
            shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
        };
        let user_pref1 = Some(ShirtColor::Red);
        let giveaway1 = store.giveaway(user_pref1);
        println!(
            "The user with preference {:?} gets {:?}",
            user_pref1, giveaway1
        );
        let user_pref2 = None;
        let giveaway2 = store.giveaway(user_pref2);
        println!(
            "The user with preference {:?} gets {:?}",
            user_pref2, giveaway2
        );
    }
    // fn add_one_v1(x: u32) -> u32 {
    //     x + 1
    // }
    // let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v3 = |x| x + 1;
    // let add_one_v4 = |x| x + 1;
    {
        let expensive_closure = |num: u32| -> u32 {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num
        };
        let example_closure = |x| x;

        let s = example_closure(String::from("hello")); // 第一次调用闭包  闭包类型以及返回类型被确认
        // let n = example_closure(5); // 再次修改类型导致错误 类型不匹配
    }
    {
        let list = vec![1, 2, 3];
        println!("Before defining closure: {list:?}");

        let only_borrows = || println!("From closure: {list:?}");

        println!("Before calling closure: {list:?}");
        only_borrows();
        println!("After calling closure: {list:?}");
    }
    {
        let mut list = vec![1, 2, 3, 4];
        println!("Before defining closure: {list:?}");

        let mut borrows_mutably = || list.push(7);
        // println!("After calling closure: {list:?}");

        borrows_mutably();
        println!("After calling closure: {list:?}");
    }
    {
        let list = vec![1, 2, 3];
        println!("Before defining closure: {list:?}");
        //使用 move 来强制闭包为线程获取 list 的所有权
        thread::spawn(move || println!("From thread: {list:?}"))
            .join()
            .unwrap();
    }
    /*
    FnOnce 适用于只能被调用一次的闭包。所有闭包至少都实现了这个 trait，因为所有闭包都能被调用。一个会将捕获的值从闭包体中移出的闭包只会实现 FnOnce trait，而不会实现其他 Fn 相关的 trait，因为它只能被调用一次。
    FnMut 适用于不会将捕获的值移出闭包体，但可能会修改捕获值的闭包。这类闭包可以被调用多次。
    Fn 适用于既不将捕获的值移出闭包体，也不修改捕获值的闭包，同时也包括不从环境中捕获任何值的闭包。这类闭包可以被多次调用而不会改变其环境，这在会多次并发调用闭包的场景中十分重要
    */
    {
        let mut list = [
            Rectangle {
                width: 10,
                height: 1,
            },
            Rectangle {
                width: 3,
                height: 5,
            },
            Rectangle {
                width: 7,
                height: 12,
            },
        ];

        list.sort_by_key(|r| r.width);
        println!("{list:#?}");
    }
    // 迭代器
    // 迭代器是 惰性的（lazy） 在调用消费迭代器的方法之前不会执行任何操作
    {
        let v1 = vec![1, 2, 3];
        // iter 方法生成一个不可变引用的迭代器
        // 迭代可变引用，可以调用 iter_mut
        let mut v1_iter = v1.iter();
        // let v1_item = v1_iter.next().unwrap();
        // println!("v1_item = {:?}", v1_item);
        // let v1_item = v1_iter.next().unwrap();
        // println!("v1_item = {:?}", v1_item);
        //
        // let v1_item = v1_iter.next().unwrap();
        // println!("v1_item = {:?}", v1_item);

        // for val in v1_iter {
        //     println!("Got: {val}");
        // }
        // 调用 next 方法的方法被称为 消费适配器（consuming adaptors）

        let total: u32 = v1_iter.sum();
        println!("total = {:?}", total);
        // 迭代器适配器（iterator adaptors）
        let v1 = vec![1, 2, 3];
        // Vec<_> 自动推断类型
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
        println!("v1 = {:?}", v2);
    }
}
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}
impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
