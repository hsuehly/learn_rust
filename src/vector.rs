enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
pub fn run() {
    let mut v: Vec<i32> = Vec::new();

    v.push(3);
    v.push(4);
    let third = &v[1];

    println!("The third element is {third}");
    // let third = v.get(2);
    let third = v.get(1);
    match third {
        Some(x) => println!("The third element is {}", x),
        None => panic!("There is no third element!"),
    }
    let mut v2 = vec![1, 2, 3];
    for i in &mut v2 {
        *i += 50;
        println!("{}", i);
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for i in &row {
        match i {
            SpreadsheetCell::Float(v) => {
                println!("{}", v);
            }
            _ => {}
        }
    }
}
