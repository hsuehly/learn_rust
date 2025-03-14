pub(crate) fn run() {
    {
        // 所有权，对 Rust 而言，一个值如果没有实现 Copy，在赋值、传参以及函数返回时会被 Move
        // 引用-借用
        let s1 = String::from("hello");
        let len = calculate_length(&s1);
        println!("The length of '{}' is {}.", s1, len);
    }
    {
        let mut s2 = String::from("hello");
        //可变引用
        // 在同一时间，只能有一个对某一特定数据的可变引用。尝试创建两个可变引用的代码将会失败
        // 一个作用域内，仅允许一个活跃的可变引用。所谓活跃，就是真正被使用来修改数据
        // 的可变引用，如果只是定义了，却没有使用或者当作只读引用使用，不算活跃。
        // 在一个作用域内，活跃的可变引用（写）和只读引用（读）是互斥的，不能同时存。
        change(&mut s2);
    }
    {
        let mut s = String::from("hello");
        // r1 可变借用了 s
        let r1 = &mut s;
        // r2 也可变借用了s

        // let r2 = &mut s;
        // 两个都同时是可变借用，会导致数据竞争 而出现问题
        // 数据竞争（data race）类似于竞态条件，它由这三个行为造成：
        // 1.两个或更多指针同时访问同一数据。
        // 2.至少有一个指针被用来写入数据。
        // 3.没有同步数据访问的机制。
        // 以上三个行为同时发生才会造成数据竞争，而不是单一行为。
        // println!("r1={} r2={}", r1, r2);
    }

    {
        let mut s = String::from("hello");

        let r1 = &s; // 没问题
        let r2 = &s; // 没问题
        // let r3 = &mut s; // 大问题
        //也不能在拥有不可变引用的同时拥有可变引用
        // println!("{}, {}, and {}", r1, r2, r3);
        // 注意一个引用的作用域从声明的地方开始一直持续到最后一次使用为止。例如，因为最后一次使用不可变引用（println!)，发生在声明可变引用之前 let r1 = &s; // 没问题
        let r2 = &s; // 没问题
        println!("{} and {}", r1, r2);
        // 此位置之后 r1 和 r2 不再使用

        let r3 = &mut s; // 没问题
        println!("{}", r3);
        // 不可变引用 r1 和 r2 的作用域在 println! 最后一次使用之后结束，这也是创建可变引用 r3 的地方。它们的作用域没有重叠，所以代码是可以编译的。
    }
    {
        // let reference_to_nothing = dangle();
    }
    // 引用的规则
    // 让我们概括一下之前对引用的讨论：
    //
    // 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
    // 引用必须总是有效的。
    {
        let data = vec![1, 2, 3, 4];
        let data1 = &data;
        // 值的地址是什么？引用的地址又是什么？
        println!(
            "addr of value: {:p}({:p}), addr of data {:p}, data1: {:p}",
            &data, data1, &&data, &data1
        );
        println!("sum of data1: {}", sum(data1));
        // 堆上数据的地址是什么？
        println!(
            "addr of items: [{:p}, {:p}, {:p}, {:p}]",
            &data[0], &data[1], &data[2], &data[3]
        );
    }
}
fn sum(data: &Vec<u32>) -> u32 {
    // 值的地址会改变么？引用的地址会改变么？
    println!("addr of value: {:p}, addr of ref: {:p}", data, &data);
    data.iter().fold(0, |acc, x| acc + x)
}
fn calculate_length(s: &String) -> usize {
    s.len()
}
fn change(some_string: &mut String) {
    // 修改操作 需要加上mut
    some_string.push_str(", world");
}

// 悬垂指针
// fn dangle() -> &String {
//     let s = String::from("hello");
//
//     &s
//     // s 在 } 执行后会导致指向的内容被释放 此时返回指针 会导致悬垂指针（其指向的内存可能已经被分配给其它持有者）
// }// 这里 s 离开作用域并被丢弃。其内存被释放。
//   // 危险！
