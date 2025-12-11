#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
// 元组结构体
#[derive(Debug)]
struct Color(String, i32, i32);
#[derive(Debug)]
struct Point(String, i32, i32);

// 没有任何字段的类单元结构体 空结构体
struct AlwaysEqual;

struct Rectangle {
    width: u32,
    height: u32,
}
// 可以多个 impl 块
impl Rectangle {
    // 关联函数使用::调用 方法实用.调用
    // :: 语法用于关联函数和模块创建的命名空间 和范型::<> 涡轮鱼。
    //                                      尾巴身子/头

    // new 因为它的第一个参数不是self函数被称为关联函数（associated function） 关联函数使用 :: 调用
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
    //结构体的上下文中被定义 或者是枚举或 trait 对象的上下文
    // 第一个参数含有 self的是方法 方法使用 . 调用
    fn area(&self) -> u32 {
        // &self 的理由跟在函数版本中使用 &Rectangle 是相同的：我们并不想获取所有权，只希望能够读取结构体中的数据，而不是写入。如果想要在方法中改变调用方法的实例，需要将第一个参数改为 &mut self。通过仅仅使用 self 作为第一个参数来使方法获取实例的所有权是很少见的；这种技术通常用在当方法将 self 转换成别的实例的时候，这时我们想要防止调用者在转换之后使用原始的实例
        self.width * self.height
    }
    // self 表示 Rectangle 的所有权转移到该方法中，这种形式用的较少
    // &self 表示该方法对 Rectangle 的不可变借用
    // &mut self 表示可变借用
    fn width(&self) -> bool {
        self.width > 0
    }
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle { x, y, radius }
    }
}
pub fn run() {
    {
        let mut user1 = User {
            active: false,
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            sign_in_count: 1,
        };
        user1.email = String::from("axxxxxil@example.com");
        user1.username = String::from("hseuhly");
        println!("user1 {:?}", user1);
        let user2 = User {
            email: String::from("another@example.com"),
            ..user1 // .. 语法指定了剩余未显式设置值的字段应有与给定实例对应字段相同的值。
                    // 实现Copy trait 的类型 的值被复制到user2 那么user1的变量依然可用，但是当 为引用类型的值被移动到user2中 那么user1 不能使用 被回收
        };
        // println!("user1 {:?}", user1);
        println!("user1 {:?}", user1.sign_in_count);
        println!("user2 {:#?}", user2)
    }
    {
        // 元组结构体（tuple struct）
        let mut black = Color(String::from("hello"), 0, 0);
        println!("black {:?}", black);
        black.0 = String::from("world");
        println!("black {:?}", black);

        let origin = Point(black.0, 0, 0);
        println!("origin {:?}", origin);
        // println!("black {:?}", black);
    }
    {
        let subject = AlwaysEqual;
    }
    {
        let rect = Rectangle::new(10, 20);

        // println!("rect {:?}", rect.area());
        println!("rect {:?}", &rect.area());

        // println!("rect {:?}", rect);
        if rect.width() {
            println!("The rectangle has a nonzero width; it is {}", rect.width);
        }
    }
}
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
