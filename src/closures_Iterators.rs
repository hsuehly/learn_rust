use std::collections::HashMap;
use std::thread;
use std::time::Duration;

pub fn run() {
    //闭包：可以捕获环境的匿名函数 闭包允许捕获其被定义时所在作用域中的值
    // 闭包表达式 |param1| 返回表达式 闭包可以不标注类型
    // |param1, param2,...| {
    //     语句1;
    //     语句2;
    //     返回表达式
    // }
    // fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    // let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v3 = |x|             { x + 1 };
    // let add_one_v4 = |x|               x + 1  ;
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
    /* 三种 Fn 特征
    FnOnce 该类型的闭包会拿走被捕获变量的所有权 并只运行一次 适用于只能被调用一次的闭包。所有闭包至少都实现了这个 trait，因为所有闭包都能被调用。一个会将捕获的值从闭包体中移出的闭包只会实现 FnOnce trait，而不会实现其他 Fn 相关的 trait，因为它只能被调用一次。
    FnMut 可变借用的方式捕获环境的值  适用于不会将捕获的值移出闭包体，但可能会修改捕获值的闭包。这类闭包可以被调用多次。
    Fn 适用于既不将捕获的值移出闭包体，也不修改捕获值的闭包，同时也包括不从环境中捕获任何值的闭包。这类闭包可以被多次调用而不会改变其环境，这在会多次并发调用闭包的场景中十分重要
    如果要闭包捕获的类型都实现了Copy特征的话，这个闭包就会默认实现Copy特征
    如果想强制闭包取得捕获变量的所有权 可以在参数列表前添加 move关键字 这种做法通常是闭包的生命周期大于捕获变量的生命周期
    */
    /*
    FnOnce、FnMut、Fn 三种关系
    实际上，一个闭包并不仅仅实现某一种 Fn 特征，规则如下：
    所有的闭包都自动实现了 FnOnce 特征，因此任何一个闭包都至少可以被调用一次
    没有移出所捕获变量的所有权的闭包自动实现了 FnMut 特征
    不需要对捕获变量进行改变的闭包自动实现了 Fn 特征
    */

    {
        let mut s = String::new();

        let update_string = |str| s.push_str(str);

        // exec(update_string);

        println!("{:?}", s);
    }
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
    {
        // let closure_slision = |x: &i32| -> &i32 { x };
        let closure_slision = fun(|x: &i32| -> &i32 { x });
        assert_eq!(*closure_slision(&45), 45);
    }

    {
        let s = String::new();
        let update_string = || println!("{}", s);
        exec(update_string);
        exec1(update_string);
        exec2(update_string);
    }
    // 迭代器
    // 迭代器是 惰性的（lazy） 在调用消费迭代器的方法之前不会执行任何操作
    // into_iter 会夺走所有权
    // iter 是借用
    // iter_mut 是可变借用
    // into_ 之类的，都是拿走所有权，_mut 之类的都是可变借用，剩下的就是不可变借用
    //
    // .iter() 方法实现的迭代器，调用 next 方法返回的类型是 Some(&T)
    // .iter_mut() 方法实现的迭代器，调用 next 方法返回的类型是 Some(&mut T)，因此在 if let Some(v) = values_iter_mut.next() 中，v 的类型是 &mut i32，最终我们可以通过 *v = 0 的方式修改其值
    // Iterator 和 IntoIterator 的区别
    // Iterator 就是迭代器特征，只有实现了它才能称为迭代器，才能调用 next
    // IntoIterator 强调的是某一个类型如果实现了该特征，它可以通过 into_iter，iter 等方法变成一个迭代器。
    {
        let v1 = vec![1, 2, 3];
        // iter 方法生成一个不可变引用的迭代器
        // 迭代可变引用，可以调用 iter_mut
        let mut v1_iter = v1.iter();
        // next 方法返回的是 Option 类型，当有值时返回 Some(i32)，无值时返回 None
        // 遍历是按照迭代器中元素的排列顺序依次进行的，因此我们严格按照数组中元素的顺序取出了 Some(1)，Some(2)，Some(3)
        // 手动迭代必须将迭代器声明为 mut 可变，因为调用 next 会改变迭代器其中的状态数据（当前遍历的位置等），而 for 循环去迭代则无需标注 mut，因为它会帮我们自动完成

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
    {
        let values = vec![1, 2, 3];

        {
            let result = match IntoIterator::into_iter(values) {
                mut iter => loop {
                    match iter.next() {
                        Some(x) => {
                            println!("{}", x);
                        }
                        None => break,
                    }
                },
            };
            result
        }
    }
    {
        // zip 是一个迭代器适配器，它的作用就是将两个迭代器的内容压缩到一起，形成 Iterator<Item=(ValueFromA, ValueFromB)> 这样的新的迭代器，在此处就是形如 [(name1, age1), (name2, age2)] 的迭代器。
        let names = ["sunface", "sunfei"];
        let age = [18, 18];
        let folks: HashMap<_, _> = names.into_iter().zip(age.into_iter()).collect();
        println!("{:?}", folks);
    }
    {
        let mut counter = Counter::new();
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
        // let a = i8::MAX;
        // println!("{}", a);
        let mut values: [i32; 2] = [1, 2];
        let p1: *mut i32 = values.as_mut_ptr();
        println!("p1_addr{:?}", p1);
        let first_address = p1 as usize; // 将p1内存地址转换为一个整数
        println!("first_address{:?}", first_address);
        let second_address = first_address + 4; // 4 == std::mem::size_of::<i32>()，i32类型占用4个字节，因此将内存地址 + 4
        println!("second_address{:?}", second_address);
        let p2 = second_address as *mut i32; // 访问该地址指向的下一个整数p2
        println!("p2{:?}", p2);
        unsafe {
            *p2 += 1;
        }
        println!("p2_{:?}", values[1])
    }
    //enumerate() 用来获取迭代的索引 返回的是元组
}
fn fun<T, F: Fn(&T) -> &T>(f: F) -> F {
    f
}
// fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
//     f("hello")
// }
fn exec<F: FnOnce()>(f: F) {
    f()
}
fn exec1<F: FnMut()>(mut f: F) {
    f()
}
fn exec2<F: Fn()>(f: F) {
    f()
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

struct Counter {
    count: u32,
}
impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }
}
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
