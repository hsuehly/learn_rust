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
}
