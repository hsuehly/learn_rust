pub fn run() {
    let mut s = String::new();
    let data = "initial contents".to_string();
    let s = String::from("initial contents");
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用

    let s = format!("{s2}-{s3}");
    let s1 = String::from("hello");
    // let h = s1[0];
    //
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{}", s);
    let my_name = "Pascal";
    greet(my_name.to_string());
    // 切片的索引必须落在字符之间的边界位置
    let s = String::from("hello world");
    let s = "Hello, world!";
    // 右半开区间 左端点是包含在内的，而右端点是不包含在内
    let hello = &s[0..5];
    let world = &s[6..11];
}

fn greet(name: String) {
    println!("Hello, {}", name)
}
