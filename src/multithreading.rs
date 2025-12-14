use std::{sync::mpsc, thread, time::Duration};
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
}
