pub fn run() {
    /*
    * unsafe 能赋予我们 5 种超能力，这些能力在安全的 Rust 代码中是无法获取的：
    解引用裸指针
    调用一个 unsafe 或外部的函数
    访问或修改一个可变的静态变量
    实现一个 unsafe 特征
    访问 union 中的字段
    */
    // unsafe 并不能绕过 Rust 的借用检查，也不能关闭任何 Rust 的安全检查规则，例如当你在 unsafe 中使用引用时，该有的检查一样都不会少。
    // 裸指针长这样: *const T 和 *mut T
    /*
    * 裸指针：
    可以绕过 Rust 的借用规则，可以同时拥有一个数据的可变、不可变指针，甚至还能拥有多个可变的指针
    并不能保证指向合法的内存
    可以是 null
    没有实现任何自动的回收 (drop)
    */

    {
        // 创建裸指针是安全的行为，而解引用裸指针才是不安全的行为 :

        let mut num = 5;
        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;
        unsafe {
            println! {"r1 is: {}", *r1};
        }
    }
}
