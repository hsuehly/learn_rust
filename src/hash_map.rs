use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, ErrorKind, Read};
// 使用 HashMap 必须要从标准库里进行引用
pub fn run() {
    // 如果预先知道要存储的 KV 对个数，可以使用 HashMap::with_capacity(capacity) 创建指定大小的 HashMap，避免频繁的内存分配和拷贝，提升性能。
    //所有的键必须是相同类型，值也必须都是相同类型
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    // let score = scores.get(&team_name).copied().unwrap_or(0);
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // 这里 field_name 和 field_value 不再有效，
    // 尝试使用它们看看会出现什么编译错误！

    {
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 25);
        // 只在键没有对应值时插入键值对
        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);

        println!("{scores:?}");
    }
    {
        let text = "hello world wonderful world";

        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        println!("{map:?}");
    }
    {
        let greeting_file_result = File::open("hello.txt");

        let greeting_file = greeting_file_result.unwrap_or_else(|error| match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        });
    }
    {
        let teams_list = vec![
            ("中国队".to_string(), 100),
            ("美国队".to_string(), 10),
            ("日本队".to_string(), 50),
        ];
        let teams_map: HashMap<_, _> = teams_list.into_iter().collect();
        println!("{:?}", teams_map)
    }
}
//? 运算符只能被用于返回值与 ? 作用的值相兼容的函数
fn read_username_from_file() -> Result<String, Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
