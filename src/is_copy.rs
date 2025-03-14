fn is_copy<T: Copy>() {}
fn types_impl_copy_trait() {
    is_copy::<bool>();
    is_copy::<char>();
    // all iXX and uXX, usize/isize, fXX implement Copy trait
    is_copy::<i8>();
    is_copy::<u64>();
    is_copy::<i64>();
    is_copy::<usize>();
    // function (actually a pointer) is Copy
    is_copy::<fn()>();
    // raw pointer is Copy
    is_copy::<*const String>();
    is_copy::<*mut String>();
    // immutable reference is Copy
    is_copy::<&[Vec<u8>]>();
    is_copy::<&String>();
    // array/tuple with values which is Copy is Copy
    is_copy::<[u8; 4]>();
    is_copy::<(&str, &str)>();
}
fn types_not_impl_copy_trait() {
    // unsized or dynamic sized type is not Copy
    // is_copy::<str>();
    // is_copy::<[u8]>();
    // is_copy::<Vec<u8>>();
    // is_copy::<String>();
    // mutable reference is not Copy
    // is_copy::<&mut String>();
    // array / tuple with values that not Copy is not Copy
    // is_copy::<[Vec<u8>; 4]>();
    // is_copy::<(String, u32)>();
}
pub fn run() {
    types_impl_copy_trait();
    types_not_impl_copy_trait()
    /*
    推荐你动手运行这段代码，并仔细阅读编译器错误，加深印象。我也总结一下：
        原生类型，包括函数、不可变引用和裸指针实现了 Copy；
        数组和元组，如果其内部的数据结构实现了 Copy，那么它们也实现了 Copy；
        可变引用没有实现 Copy；非固定大小的数据结构，没有实现 Copy
    另外，官方文档介绍 Copy trait 的页面包含了 Rust 标准库中实现 Copy trait 的所有数据结构

    当我们进行变量赋值、传参和函数返回时，如果涉及的数据结构没有实现 Copy trait，就会默认使用 Move 语义转移值的所有权，失去所有权的变量将无法继续访问原来的数据；
    */
}
