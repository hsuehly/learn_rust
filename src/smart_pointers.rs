use std::{
    cell::{Cell, RefCell},
    ops::Deref,
    rc::Rc,
    sync::Arc,
    thread,
};

#[derive(Debug)]
struct Server {
    count: u32,
}
//语法规则是：dyn 后面必须接一个 Trait。
enum List {
    Cons(i32, Box<List>),
    Nil,
}
// 元组结构体
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
// Drop
struct HasDrop1;
struct HasDrop2;

impl Drop for HasDrop1 {
    fn drop(&mut self) {
        println!("Dropping HasDrop1");
    }
}
impl Drop for HasDrop2 {
    fn drop(&mut self) {
        println!("Dropping HasDrop2!");
    }
}
// 结构体中的字段按照定义中的顺序依次 drop
struct HasTwoDrops {
    one: HasDrop1,
    two: HasDrop2,
}

impl Drop for HasTwoDrops {
    fn drop(&mut self) {
        println!("Dropping HasTwoDrops!");
    }
}

#[derive(Debug)]
struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Dropping Foo!")
    }
}
pub fn run() {
    // 指针 是一个包含了内存地址的变量，该内存地址引用或者指向了另外的数据
    // Deref 可以让智能指针像引用那样工作，这样你就可以写出同时支持智能指针和引用的代码，例如 *T
    // Drop 允许你指定智能指针超出作用域后自动执行的代码，例如做一些数据清除等收尾工作
    /*
    * Box<T>，可以将值分配到堆上
    Rc<T>，引用计数类型，允许多所有权存在
    Ref<T> 和 RefMut<T>，允许将借用规则检查从编译期移动到运行期进行
    */

    // Box<T> 允许你将一个值分配到堆上
    // Box 的使用场景
    /*
     * 特意的将数据分配在堆上
     * 数据较大时又不想在转移所有权时进行数据拷贝
     * 类型大小在编译期间无法确定，但我们又需要固定大小的类型时
     * 特征对象，用于说明对象实现了一个特征，而不是某个特定的类型
     */
    println!("智能指针");
    //Box<T> 只能移动（Move），不能复制（Copy），不管里面的 T 是什么
    let a = Box::new(12);

    // let e = Box::new(String::from("Hello"));
    // let f = *e + " World";

    // println!("e = {}", e);
    // 在堆上创建一个长度为1000的数组，然后使用一个智能指针指向它

    /*
    * println! 可以正常打印出 a 的值，是因为它隐式地调用了 Deref 对智能指针 a 进行了解引用
    最后一行代码 let b = a + 1 报错，是因为在表达式中，我们无法自动隐式地执行 Deref 解引用操作，你需要使用 * 操作符 let b = *a + 1，来显式的进行解引用
    a 持有的智能指针将在作用域结束（main 函数结束）时，被释放掉，这是因为 Box<T> 实现了 Drop 特征
    */
    println!("a = {}", a); // a = 12

    // let b = *a + 1;
    // let b = *a + 1;
    // let c = &12;
    // // 自动解引用
    // let d = *c + 3;
    // let d = c + 3;
    // println!("c = {}", c);
    /*
     * 一个简单的场景，你需要一个在运行期初始化的值，但是可以全局有效，也就是和整个程序活得一样久，那么就可以使用 Box::leak，
     * 例如有一个存储配置的结构体实例，它是在运行期动态插入内容，那么就可以将其转为全局有效，
     * 虽然 Rc/Arc 也可以实现此功能，但是 Box::leak 是性能最高的。
     */
    {
        let s = gen_static_str();
        println!("{}", s);
    }

    {
        let y = MyBox::new(5);
        assert_eq!(5, *y);
    }
    {
        let _x = HasTwoDrops {
            two: HasDrop2,

            one: HasDrop1,
        };
        let _foo = Foo;
        println!("Running!");
        // _foo.drop();
        // println!("Running!:{:?}", _foo);
        drop(_foo);
        // 以下代码会报错：借用了所有权被转移的值
        // println!("Running!:{:?}", _foo);
    }
    {
        // rc arc 1vN所有权机制 Rc 适用于单线程 Arc 适用于多线程 Rc 和 Arc 默认是只读的。
        // Rc 引用计数 希望在堆上分配一个对象供程序的多个部分使用且无法确定哪个部分最后一个结束时，就可以使用 Rc 成为数据值的所有者，

        // let a = Rc::new(String::from("hello world"));
        // // 这里的 clone 仅仅复制了智能指针并增加了引用计数，并没有克隆底层数据，因此 a 和 b 是共享了底层的字符串 s，这种复制效率是非常高的。
        // // 迭代器的clone 也是浅克隆
        // let b = Rc::clone(&a);

        // assert_eq!(2, Rc::strong_count(&a));
        // assert_eq!(Rc::strong_count(&a), Rc::strong_count(&b));
        let a = Rc::new(String::from("test ref counting"));
        println!("count after creating a = {}", Rc::strong_count(&a));
        let b = Rc::clone(&a);
        println!("count after creating b = {}", Rc::strong_count(&a));
        {
            let c = Rc::clone(&a);
            println!("count after creating c = {}", Rc::strong_count(&c));
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));

        let mut d = String::from("hello");
        let f = Rc::new(&mut d);
        // f.push_str("oop");
    }
    {
        // Arc使用CPU 原子指令，速度稍慢（因为要排队锁总线），但保证了多线程下的计数绝对准确
        let s = Arc::new(String::from("多线程漫游者"));
        for _ in 0..10 {
            let s = Arc::clone(&s);
            let handle = thread::spawn(move || println!("{}", s));
            handle.join().unwrap();
        }
    }
    {
        // Cell<T> 适用于 T 实现 Copy 的情况
        let c = Cell::new("asdf");
        let d = &c;
        let one = d.get();
        c.set("qwer");
        let two = c.get();
        println!("one: {}, two: {}", one, two);
        // code snipet 1
        let x = Cell::new(1);
        let y = &x;
        let z = &x;
        x.set(2);
        y.set(3);
        z.set(4);
        println!("{}", x.get());

        // code snipet 2
        // let mut x = 1;
        // // 同一时间不能有多个可变引用
        // let y = &mut x;
        // let z = &mut x;
        // x = 2;
        // *y = 3;
        // *z = 4;
        // println!("{}", x);
    }
    {
        /*
        * 与 Cell 用于可 Copy 的值不同，RefCell 用于引用
        RefCell 只是将借用规则从编译期推迟到程序运行期，并不能帮你绕过这个规则
        RefCell 适用于编译期误报或者一个引用被在多处代码使用、修改以至于难于管理借用关系时
        使用 RefCell 时，违背借用规则会导致运行期的 panic
        */
        /*
         * Cell 只适用于 Copy 类型，用于提供值，而 RefCell 用于提供引用
        Cell 不会 panic，而 RefCell 会 */
        let s = RefCell::new(String::from("hello, world"));
        let s1 = s.borrow();
        let s2 = s.borrow_mut();

        println!("{},{}", s1, s2);
    }
}
fn gen_static_str() -> &'static str {
    let mut s = String::new();
    s.push_str("hello, world");

    Box::leak(s.into_boxed_str())
}
