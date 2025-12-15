use std::{
    fs,
    io::{BufReader, Read, Write, prelude::*},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex, mpsc},
    thread,
    time::Duration,
};

type Job = Box<dyn FnOnce() + Send + 'static>;
struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            // create some threads and store them in the vector
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }
    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        println!("Sending terminate message to all workers.");
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // 尚未实现..
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv();
                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");

                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });
        // 每个 `Worker` 都拥有自己的唯一 id
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

pub fn run() {
    // 监听本地端口 7878 ，等待 TCP 连接的建立
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    // 阻塞等待请求的进入
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // handle_connection(stream);

        pool.execute(|| {
            handle_connection(stream);
        });
        // println!("Connection established!");
    }
}
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();
    // println!("Request: {:#?}", http_request);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }

        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();

    // stream.write_all(response.as_bytes()).unwrap();
    // // 处理HTTP协议头，若不符合则返回404和对应的 `html` 文件
    // let (status_line, filename) = if buffer.starts_with(get) {
    //     ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    // } else {
    //     ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    // };
    // let contents = fs::read_to_string(filename).unwrap();

    // // 将回复内容写入连接缓存中
    // let response = format!("{status_line}{contents}");
    // stream.write_all(response.as_bytes()).unwrap();
    // // 使用 flush 将缓存中的内容发送到客户端
    // stream.flush().unwrap();
}
