use std::{thread, sync::mpsc};
use std::sync::{Mutex, Arc};

type Job = Box<dyn FnOnce()+ Send + 'static>;

struct Worker{
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

pub struct ThreadPool{
    sender: Option<mpsc::Sender<Job>>,
    workers : Vec<Worker>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let mut workers = Vec::with_capacity(size);
        let (tx, rx) =mpsc::channel();

        let arc_rx = Arc::new(Mutex::new(rx));
        for idx in 0..size {
            let new_rx = Arc::clone(&arc_rx);
            let worker: Worker = Worker::new(idx, new_rx);
            workers.push(worker);
        }
        ThreadPool {workers, sender:Some(tx)}
    }

    pub fn execute<F>(&self, func : F)
    where 
        F : FnOnce()+ Send + 'static
    {
        let func_obj = Box::new(func);
        self.sender.as_ref().unwrap().send(func_obj).unwrap()
    }
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {

        let thread = thread::spawn(move || {
            loop {
                // receiver receive from the sender
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(func) => {
                        println!("Handle by thread {id}");
                        func();
                    },
                    Err(_) => {
                        println!("Shutdown thread due to error: {id}");
                        break;
                    },
                }
            }
        });
        Worker { id, thread: Some(thread) }

    }
}

impl Drop for ThreadPool{
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers{
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
        
    }
}