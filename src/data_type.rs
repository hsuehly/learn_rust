/*
* 关键字是const而不是let
定义常量必须指明类型（如 i32）不能省略
定义常量时变量的命名规则一般是全部大写
常量可以在任意作用域进行定义，其生命周期贯穿整个程序的生命周期。编译时编译器会尽可能将其内联到代码中，所以在不同地方对同一常量的引用并不能保证引用到相同的内存地址
常量的赋值只能是常量表达式/数学表达式，也就是说必须是在编译期就能计算出的值，如果需要在运行时才能得出结果的值比如函数，则不能赋值给常量表达式
对于变量出现重复的定义(绑定)会发生变量遮盖，后面定义的变量会遮住前面定义的变量，常量则不允许出现重复的定义
*/
const MAX_ID: usize = usize::MAX / 2;
// 静态变量允许声明一个全局的变量，常用于全局数据统计
// Rust 要求必须使用unsafe语句块才能访问和修改static变量
// 静态变量的时候必须赋值为在编译期就可以计算出的值(常量表达式/数学表达式)，不能是运行时才能计算出的值(如函数)
static mut REQUEST_RECV: usize = 0;
// {
//     unsafe {
//           REQUEST_RECV += 1;
//           assert_eq!(REQUEST_RECV, 1);
//      }
// }
/*
* 静态变量和常量的区别

静态变量不会被内联，在整个程序中，静态变量只有一个实例，所有的引用都会指向同一个地址
存储在静态变量中的值必须要实现 Sync trait

*/
pub fn run() {
    // main 线程的栈大小是 8MB，普通线程是 2MB
    /*
    * 小型数据，在栈上的分配性能和读取性能都要比堆上高
    中型数据，栈上分配性能高，但是读取性能和堆上并无区别，因为无法利用寄存器或 CPU 高速缓存，最终还是要经过一次内存寻址
    大型数据，只建议在堆上分配和使用
    */

    // 标量类型
    // 标量（scalar）类型表示单个值。Rust 有 4 个基本的标量类型：整型、浮点型、布尔型和字符 。
    // 有符号类型可存储的数字范围是 -2^n-1 ~ 2^n-1 -1 其中 n 是该定义形式的位长度。因此 i8 可存储数字范围是 -(27) ~ 27 - 1，即 -128 ~ 127
    // 无符号类型可存储的数字范围是 0 ~ 2^n - 1 其中 n 是该定义形式的位长度。因此 u8 可存储数字范围是 0 ~ 28 - 1，即 0 ~ 255
    // isize 和 usize 位数是取决于cpu 架构 动态改变 如果cpu 32位 那么 他们就为32 如果64位 那么他们就为64
    // Rust 整形默认使用i32
    let _i = 2;
    let a: u8 = 255;
    let b = a.wrapping_add(20);
    println!("{}", b);
    // 浮点类型 默认浮点类型f64
    // 避免在浮点数上测试相等性
    // 当结果在数学上可能存在未定义时，需要格外小心
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess: {}", guess);
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // 字符串类型
    // '' 用来表示字符 "" 用来表示字符串
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    let x = '中';
    println!("字符大小{}", size_of_val(&x));

    let x = (-42.0_f32).sqrt();
    if x.is_nan() {
        println!("NaN");
    }
    // 数字运算
    // 编译器自动推导 默认i32
    let twenty = 20;
    // 类型标注
    let twenty_one: i32 = 21;
    // 通过类型后缀来进行标注
    let twenty_two = 32_i32;
    // 同类型可以进行运算
    let addition = twenty + twenty_one + twenty_two;
    // 较长的数字可以进行_分割
    let _one_million: i64 = 1_000_000;
    println!(
        "{} + {} + {} = {}",
        twenty, twenty_one, twenty_two, addition
    );
    // 无符号8位整数 二进制为 00000010
    let a: u8 = 2;
    let b: u8 = 3;

    // {:08b}: 左高右低输出二进制01 不足8位则高位补0
    //
    println!("a value is {:08b}", a);
    println!("b value is {:08b}", b);
    println!("(a & b) value is {:08b}", a & b);
    println!("(a | b) value is {:08b}", a | b);
    println!("(a ^ b) value is {:08b}", a ^ b);
    println!("!b value is {:08b}", !b);
    println!("(a << b) value is {:08b}", a << b);
    println!("(a >> b) value is {:08b}", a >> b);

    let mut a = a;
    a <<= b;
    println!("(a << b) value is {:08b}", a);
    // let a: u8 = 255;
    // let b = a >> 7; // ok
    // let b = a << 7; // ok
    // let b = a >> 8; // overflow
    // let b = a << 8; // overflow
    // 不包含5
    for i in 1..5 {
        println!("{}", i)
    }
    // 包含5
    for i in 1..=5 {
        println!("{}", i)
    }

    // 复合类型
    // 复合类型（compound type）可以将多个值组合成一个类型。Rust 有两种基本的复合类型：元组（tuple）和数组（array）。
    let tup = (500, 6.4, 1);
    println!("tup0 {}", tup.0);
    // 解构
    let (x, y, z) = tup;

    // === 数组 ===
    let arr_a = [String::from("rust"), String::from("java")];
    let arr_b = arr_a; // 发生 Move！所有权移交给了 arr_b

    // println!("{:?}", arr_a); // ❌ 报错：arr_a 的值已经被移动了

    // === 元组 ===
    let tup_a = (String::from("hello"), 100);
    let tup_b = tup_a; // 发生 Move！
    // println!("{:?}", tup_a); // ❌ 报错：tup_a 已经被移动
    //
    //  局部移动
    let t = (String::from("A"), String::from("B"));
    // 我们只把第一个字段的所有权移动给 a
    let (a, _) = t;

    println!("a is {}", a); // ✅ 可以使用 a
    // println!("{:?}", t); // ❌ t 作为一个整体已经部分失效了，不能再用了

    let arr = [String::from("A"), String::from("B")];

    // ❌ 下面这行会报错：cannot move out of index of array
    // let first = arr[0];

    // 解决方案 1：把整个数组的所有权都拿走（使用迭代器）
    // for s in arr {
    //     println!("{}", s);
    // }

    // 解决方案 2：如果你只想拿一个，不想销毁数组，必须用引用
    let first_ref = &arr[0];

    // 解决方案 3：如果你必须拿走其中一个的所有权，可以用 Option + replace 大法
    // (需要把数组定义为 [Option<String>; 2])
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", a);
    let a = [3; 5];
    println!("Array: {}", a[2]);
    // 表达式 表达式不能包含分号。 语句不需要最后设置分号
    // 表达式如果不返回任何值，会隐式地返回一个 ()
    let y = {
        let x = 3;
        x + 1
    };
    // let v = (let x = 3);
    array_type()
}

fn add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1; // 语句
    let y = y + 5; // 语句
    x + y // 表达式
}

fn array_type() {
    // 数组是 长度固定 元素必须相同的类型 依次线形排列
    // 数组是存储在栈上 动态数组是堆上
    let a = [1, 2, 3, 4, 5];
    // 数组的类型声明 i32 是元素类型，分号后面的数字 5 是数组长度
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    // 代表数组里面有5个元素为3的数组初始化
    let c = [3; 5];
    println!("{:?}", c);
    // 报错， 数组的array=[3;5] 写法 底层是copy进行复制的而String没有实现copy
    // let array = [String::from("rust is good!"); 8];
    // 快捷写法
    let array: [String; 4] = std::array::from_fn(|_| String::from("rust is good"));

    // 数组切片
    let slice: &[i32] = &a[1..3];
}
