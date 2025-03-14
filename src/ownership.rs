pub fn run() {
    let mut arr = [1, 2, 3, 4, 5];
    let arr2 = arr;
    arr[0] = 6;
    arr[1] = 7;
    for i in arr2 {
        println!("item {}", i)
    }
    println!("{:?}", arr);
    println!("{:?}", arr2);
}
fn own_fn() {
    // 所有权与函数
    let s = String::from("hello"); // s 进入作用域

    takes_ownership(s); // s 的值移动到函数里 ...
    // ... 所以到这里不再有效

    let x = 5; // x 进入作用域

    makes_copy(x);
} // 这里，x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
// 没有特殊之处
fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{some_string}");
} // 这里，some_string 移出作用域并调用 `drop` 方法。
// 占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{some_integer}");
} // 这里，some_integer 移出作用域。没有特殊之处

// 返回值与作用域

// 下是一些 Copy 的类型：
//
// 所有整数类型，比如 u32。
// 布尔类型，bool，它的值是 true 和 false。
// 所有浮点数类型，比如 f64。
// 字符类型，char。
// 元组，当且仅当其包含的类型也都实现 Copy 的时候。比如，(i32, i32) 实现了 Copy，但 (i32, String) 就没有。
fn ret_val_scope() {
    let s1 = gives_ownership(); // gives_ownership 将返回值
    // 转移给 s1

    let s2 = String::from("hello"); // s2 进入作用域

    let s3 = takes_and_gives_back(s2); // s2 被移动到
    // takes_and_gives_back 中，
    // 它也将返回值移给 s3
} // 这里，s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
// 所以什么也不会发生。s1 离开作用域并被丢弃
fn gives_ownership() -> String {
    // gives_ownership 会将
    // 返回值移动给
    // 调用它的函数

    let some_string = String::from("yours"); // some_string 进入作用域。

    some_string // 返回 some_string
    // 并移出给调用的函数
    //
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String {
    // a_string 进入作用域

    a_string // 返回 a_string 并移出给调用的函数
}
