use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::Receiver;
use std::thread;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

#[test]
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(10);
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(||{
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}


#[allow(dead_code)]
pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool{

    pub fn new(size: usize) -> Self{
        assert!( size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size{
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool{
            workers,
            sender,
        }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

#[allow(dead_code)]
struct Worker{
    id: usize,
    thread: thread::JoinHandle<()>,
}
trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}
type Job = Box<dyn FnBox + Send + 'static>;

impl Worker{
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self{
        let thread = thread::spawn(move ||{
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {} got a job; executing.", id);
                job.call_box();
            }
        });

        Worker{
            id,
            thread,
        }
    }
}