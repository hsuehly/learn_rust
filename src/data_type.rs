pub fn run() {
    // 标量类型
    // 标量（scalar）类型表示单个值。Rust 有 4 个基本的标量类型：整型、浮点型、布尔型和字符 。

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess: {}", guess);
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // 复合类型
    // 复合类型（compound type）可以将多个值组合成一个类型。Rust 有两种基本的复合类型：元组（tuple）和数组（array）。
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of tup is: {}", tup.0);
    println!("The value of y is: {}", y);
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", a);
    let a = [3; 5];
    println!("Array: {}", a[2]);
}
