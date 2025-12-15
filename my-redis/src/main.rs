use bytes::Bytes;
use mini_redis::{
    Command::{self, Get, Set},
    Connection, Frame,
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::net::{TcpListener, TcpStream};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

type ShardedDb = Arc<Vec<Mutex<HashMap<String, Vec<u8>>>>>;
#[tokio::main]
async fn main() {
    // let v = vec![1, 2, 3];

    // tokio::task::spawn(async {
    //     // println!("Here's a vec: {:?}", v);
    //     // 语句块的使用强制了 `rc` 会在 `.await` 被调用前就被释放，
    //     // 因此 `rc` 并不会影响 `.await`的安全性
    //     {
    //         let rc = Rc::new("hello");
    //         println!("{}", rc);
    //     }
    //     // `rc` 的作用范围已经失效，因此当任务让出所有权给当前线程时，它无需作为状态被保存起来
    //     yield_now().await;
    // });

    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    /*
    * 锁如果在多个 .await 过程中持有，应该使用 Tokio 提供的锁，原因是 .await的过程中锁可能在线程间转移，若使用标准库的同步锁存在死锁的可能性，例如某个任务刚获取完锁，还没使用完就因为 .await 让出了当前线程的所有权，结果下个任务又去获取了锁，造成死锁
    锁竞争不多的情况下，使用 std::sync::Mutex
    锁竞争多，可以考虑使用三方库提供的性能更高的锁，例如 parking_lot::Mutex
    */
    let db = Arc::new(Mutex::new(HashMap::new()));
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        // 为每一条连接都生成一个新的任务，
        // `socket` 的所有权将被移动到新的任务中，并在那里进行处理
        // 将 handle 克隆一份
        let db = db.clone();
        println!("Accepted");
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {
    // 使用 hashmap 来存储 redis 的数据

    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                // 值被存储为 `Vec<u8>` 的形式
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };
        connection.write_frame(&response).await.unwrap();
    }
}

fn mew_sharded_db(num_shards: usize) -> ShardedDb {
    let mut db = Vec::with_capacity(num_shards);
    for _ in 0..num_shards {
        db.push(Mutex::new(HashMap::new()));
    }
    Arc::new(db)
}
