use std::{
    fmt::{Debug, Display},
    slice::from_raw_parts,
    str::from_utf8_unchecked,
};
// 如果你要使用一个特征的方法，那么你需要将该特征引入当前的作用域中
pub fn run() {
    {
        let number_list = vec![34, 50, 25, 100, 65];
        let mut largest = &number_list[0];
        for i in &number_list {
            if i > largest {
                largest = i;
            }
        }
        println!("The largest is {}", largest);
        println!("The number_list {:?}", number_list);
    }
    {
        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest(&number_list);
        println!("The largest number is {result}");

        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest(&char_list);
        println!("The largest char is {result}");
    }
    {
        let integer = Point { x: 5, y: 10 };
        let float = Point { x: 1.0, y: 4.0 };
    }
    {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };

        println!("1 new tweet: {}", tweet.summarize());
    }
    // 生命周期 是以’开头 如果有& 那么生命周期在&之后 并用空格来将引用参数分隔开
    // &i32        // 一个引用
    // &'a i32     // 具有显式生命周期的引用
    // &'a mut i32 // 具有显式生命周期的可变引用
    // 1. 每一个引用参数都会获得独自的生命周期
    // 例如一个引用参数的函数就有一个生命周期标注: fn foo<'a>(x: &'a i32)，两个引用参数的有两个生命周期标注:fn foo<'a, 'b>(x: &'a i32, y: &'b i32), 依此类推。
    // 2. 若只有一个输入生命周期（函数参数中只有一个引用类型），那么该生命周期会被赋给所有的输出生命周期，也就所有返回值的生命周期等于该输入生命周期
    // 例如函数 fn foo(x: &i32) -> &i32，x 参数的生命周期会被自动赋给返回值 &i32，因此该函数等同于 fn foo<'a>(x: &'a i32) -> &'a i32
    // 3. 若存在多个输入生命周期，且其中一个是&self 或 &mut self 则 &self的生命周期被赋给所有的输出生命周期
    // 拥有 &self 形式的参数，说明该函数是一个 方法，该规则让方法的使用便利度大幅提升
    //
    {
        let string1 = String::from("abcde");
        // let result;
        // {
        //     let string2 = String::from("xyz");
        //     result = longest(string1.as_str(), string2.as_str());
        // }
        // println!("The longest string is {}-", result);
    }
    {
        let s = "hello world".to_owned();

        let mut s1 = s.as_str();
        let hello = strtok(&mut s1, ' ');
        // println!("{}", hello);
        // println!("{}", s1)
        println!("hello:{}-s1:{}-s:{}", hello, s1, s);
    }
    {
        // rust 会在编译期间自动重拍结构体以使内存对齐来节省空间
        println!(
            "sizeof S1: {}, S2: {}, S3: {}",
            size_of::<S1>(),
            size_of::<S2>(),
            size_of::<S3>()
        );
        println!(
            "alignof S1: {}, S2: {}, S3: {}",
            align_of::<S1>(),
            align_of::<S2>(),
            align_of::<S3>()
        )
    }
    // 静态生命周期
    {
        let (pointer, length) = get_memory_location();
        let message = get_str_at_location(pointer, length);
        println!(
            "The {} bytes at 0x{:X} stored: {}",
            length, pointer, message
        );
        // 如果大家想知道为何处理裸指针需要 `unsafe`，可以试着反注释以下代码
        // let message = get_str_at_location(1000, 10);
    }
}
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
// Self 与 self
//两个self，一个指代当前的实例对象，一个指代特征或者方法类型的别名
struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

//不可将随机数生成器写成 const fn。

const fn add(a: i32, b: i32) -> i32 {
    a + b
}
const RESULT: i32 = add(5, 10);

//N 就是 const 泛型，定义的语法是 const N: usize，表示 const 泛型 N ，它基于的值类型是 usize
fn display_array<T: std::fmt::Debug, const N: usize>(array: [T; N]) {
    println!("{:?}", array)
}
pub trait Summary {
    fn summarize_author(&self) -> String;
    // 在特征中定义具体的方法那么这个方法就是默认实现 其他类型无需再实现该方法 也可以进行重写方法
    // 默认实现允许调用相同特征中的其他方法，哪怕这些方法没有默认实现
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // 想要使用默认实现可以不实现方法 impl Summary for NewsArticle {} 就可以
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
pub struct Post {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Post {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
impl Summary for Tweet {
    // 重载 直接覆盖原有实现
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// 适合短小的列子  &impl 是语法糖
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// Trait Bound 语法 更直观 特征约束
// 通过 + 指定多个 trait bound
pub fn notify2<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    34
}
fn some_function2<T, U>(t: &T, u: &U) -> i32
//  where 在函数返回值后面 用来约束函数的类型
where
    T: Display + Clone,
    U: Clone + Debug,
{
    43
}
//泛型生命周期 'a 的具体生命周期等同于 x 和 y 的生命周期中较小的那一个
// 'a: 'b 'a 必须比 'b 活得久
fn longest<'a: 'b, 'b>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn strtok<'a>(s: &mut &'a str, delimiter: char) -> &'a str {
    if let Some(e) = s.find(delimiter) {
        let prefix = &s[..e];
        let suffix = &s[(e + delimiter.len_utf8())..];
        println!("{}.{}.{}", prefix, suffix, e);
        *s = suffix;
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}

struct S1 {
    a: u8,
    b: u16,
    c: u8,
}
struct S2 {
    a: u8,
    b: u8,
    c: u16,
}

struct S3;

struct MyReader<R> {
    reader: R,
    buf: String,
}
// 实现 new 函数时，我们不需要限制 R
impl<R> MyReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf: String::with_capacity(1024),
        }
    }
}

// 特征对象的限制
// 不是所有特征都能拥有特征对象，只有对象安全的特征才行
// 方法的返回类型不能是 Self
// 方法没有任何泛型参数

// 生命周期
fn get_memory_location() -> (usize, usize) {
    // “Hello World” 是字符串字面量，因此它的生命周期是 `'static`.
    // 但持有它的变量 `string` 的生命周期就不一样了，它完全取决于变量作用域，对于该例子来说，也就是当前的函数范围
    let string = "Hello World!";
    // let string = String::from("Hello World!");
    let pointer = string.as_ptr() as usize;
    let length = string.len();
    (pointer, length)
    // `string` 在这里被 drop 释放
    // 虽然变量被释放，无法再被访问，但是数据依然还会继续存活
}
fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
    // 使用裸指针需要 `unsafe{}` 语句块
    unsafe { from_utf8_unchecked(from_raw_parts(pointer as *const u8, length)) }
}
