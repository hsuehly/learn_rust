use std::fmt::{Debug, Display};
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

// 特征对象的限制
// 不是所有特征都能拥有特征对象，只有对象安全的特征才行
// 方法的返回类型不能是 Self
// 方法没有任何泛型参数
