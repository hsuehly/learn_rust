use std::fmt::{Debug, Display};

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
    // 生命周期
    {
        let string1 = String::from("long string is long");
        // let result;

        // {
        //     let string2 = String::from("xyz");
        //     result = longest(string1.as_str(), string2.as_str());
        // }
        // println!("The longest string is {result}");
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
pub trait Summary {
    fn summarize_author(&self) -> String;
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

impl Summary for Tweet {
    // 重载 直接覆盖原有实现
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// 适合短小的列子
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// Trait Bound 语法 更直观
// 通过 + 指定多个 trait bound
pub fn notify2<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    34
}
fn some_function2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    43
}
//泛型生命周期 'a 的具体生命周期等同于 x 和 y 的生命周期中较小的那一个
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
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
