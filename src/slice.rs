pub fn run() {
    {
        let mut s = String::from("hello world");
        let word = first_word(&s); // word 的值为 5
        println!("the first word is: {}", word);

        s.clear(); // 这清空了字符串，使其等于 ""

        // word 在此处的值仍然是 5，
        // 但是没有更多的字符串让我们可以有效地应用数值 5。word 的值现在完全无效！
        println!("the first word is: {}", s);
    }
    {
        let mut s = String::from("hello world");
        // let hello = &s[0..5];
        // let world = &s[6..11];
        // println!("the first word is: {}", hello);
        // println!("the first word is: {}", world);
        let word = first_word2(&s);

        // s.clear(); // error!

        println!("the first word is: {}", word);
    }
}
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
