use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
    time::Duration,
};

#[derive(Debug)]
struct MyBox(*const u8);
unsafe impl Send for MyBox {}
unsafe impl Sync for MyBox {}
pub fn run() {
    // Rust 的线程模型是 1:1 模型
    // main 线程若是结束，则所有子线程都将被终止
    let handle = thread::spawn(|| {
        for i in 1..=10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
    // 在线程中使用 move
    {
        let v = vec![1, 2, 3, 4];

        let handle = thread::spawn(move || {
            println!("Here's a vector: {:?}", v);
        });
        handle.join().unwrap();
        // 下面代码会报错borrow of moved value: `v`
        // println!("{:?}", v);
    }
    // {
    //     let new_thread = thread::spawn(|| {
    //         thread::spawn(|| {
    //             loop {
    //                 println!("I am a new thread.");
    //             }
    //         });
    //     });
    //     // 等待新创建的线程执行完成
    //     new_thread.join().unwrap();
    //     println!("Child thread is finish!");
    //     // 睡眠一段时间，看子线程创建的子线程是否还在运行
    //     thread::sleep(Duration::from_millis(100));
    // }
    {
        // 通道
        // 创建一个消息通道, 返回一个元组：(发送者，接收者)
        // 若值的类型实现了Copy特征，则直接复制一份该值，然后传输过去，例如之前的i32类型
        // 若值没有实现Copy，则它的所有权会被转移给接收端，在发送端继续使用该值将报错
        // 异步通道：无论接收者是否正在接收消息，消息发送者在发送消息时都不会阻塞:
        let (tx, rx) = mpsc::channel();
        // 可以通过克隆获取多个发送端
        // let tx1 = tx.clone();
        // 需要使用move将tx的所有权转移到子线程的闭包中
        thread::spawn(move || {
            // 发送一个数字1, send方法返回Result<T,E>，通过unwrap进行快速错误处理
            tx.send(1).unwrap();
            tx.send(2).unwrap();
            tx.send(3).unwrap();
            tx.send(4).unwrap();
            tx.send(5).unwrap();

            // 下面代码将报错，因为编译器自动推导出通道传递的值是i32类型，那么Option<i32>类型将产生不匹配错误
            // tx.send(Some(1)).unwrap()
        });
        // 在主线程中接收子线程发送的消息并输出
        // 接收消息的操作rx.recv()会阻塞当前线程，直到读取到值，或者通道被关闭
        // println!("receive {}", rx.recv().unwrap());
        for received in rx {
            println!("Got: {}", received);
        }
        // 可以使用try_recv尝试接收一次消息，该方法并不会阻塞线程
    }
    {
        // 同步通道发送消息是阻塞的，只有在消息被接收后才解除阻塞
        let (tx, rx) = mpsc::sync_channel(0);
        let handle = thread::spawn(move || {
            println!("发送之前");
            tx.send(1).unwrap();
            println!("发送之后");
        });

        println!("睡眠之前");
        thread::sleep(Duration::from_secs(3));
        println!("睡眠之后");

        println!("receive {}", rx.recv().unwrap());
        handle.join().unwrap();
    }
    {
        // 互斥锁 数据被Mutex所拥有 访问内部的数据，需要使用方法m.lock()向m申请一个锁, 该方法会阻塞当前线程，直到获取到锁
        let m = Mutex::new(5);
        {
            // 获取锁，然后deref为`m`的引用
            // lock返回的是Result
            // 与lock方法不同，try_lock会尝试去获取一次锁，如果无法获取会返回一个错误，因此不会发生阻塞
            let mut num = m.lock().unwrap();
            *num += 1;
            // 锁自动被drop
        }
        println!("num = {:?}", m);
    }
    {
        // Mutex<T>可以支持修改内部数据 Rc<T>/RefCell<T>用于单线程内部可变性， Arc<T>/Mutex<T>用于多线程内部可变性
        // 通过`Rc`实现`Mutex`的多所有权
        let counter = Arc::new(Mutex::new(5));
        let mut handles = vec![];
        for _ in 1..10 {
            let counter = Arc::clone(&counter);
            // 创建子线程，并将`Mutex`的所有权拷贝传入到子线程中
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();

                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
        // 输出最终的计数结果
        println!("Result: {}", *counter.lock().unwrap());
    }
    /*
    * RwLock:
    同时允许多个读，但最多只能有一个写
    读和写不能同时存在
    读可以使用read、try_read，写write、try_write, 在实际项目中，try_xxx会安全的多

    */
    /*
    * 实现Send的类型可以在线程间安全的传递其所有权
    实现Sync的类型可以在线程间安全的共享(通过引用)
    潜在的依赖：一个类型要在线程间安全的共享的前提是，指向它的引用必须能在线程间传递。因为如果引用都不能被传递，我们就无法在多个线程间使用引用去访问同一个数据了。
    */
    /*
    * 裸指针两者都没实现，因为它本身就没有任何安全保证
    UnsafeCell不是Sync，因此Cell和RefCell也不是
    Rc两者都没实现(因为内部的引用计数器不是线程安全的)
    自定义复合类型 只要复合类型中有一个成员不是Send或Sync，那么该复合类型也就不是Send或Sync。
    手动实现 Send 和 Sync 是不安全的，通常并不需要手动实现 Send 和 Sync trait，实现者需要使用unsafe小心维护并发安全保证
    */
    {
        // let p = 5 as *mut u8;
        // let t = thread::spawn(move || println!("{:?}", p));
        // t.join().unwrap();
        // *mut u8` cannot be sent between threads safely
        //
        let p = &MyBox(5 as *const u8);
        let v = Arc::new(Mutex::new(p));
        let t = thread::spawn(move || {
            let _v1 = v.lock().unwrap();
        });
        t.join().unwrap();
        /*
        * 实现Send的类型可以在线程间安全的传递其所有权, 实现Sync的类型可以在线程间安全的共享(通过引用)
        绝大部分类型都实现了Send和Sync，常见的未实现的有：裸指针、Cell、RefCell、Rc 等
        可以为自定义类型实现Send和Sync，但是需要unsafe代码块
        可以为部分 Rust 中的类型实现Send、Sync，但是需要使用newtype，例如文中的裸指针例子
        */
    }
}
