use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

pub struct ThreadPool {
    _handles: Vec<JoinHandle<()>>,
    sender: Sender<Box<dyn Fn() + Send>>,
}

impl ThreadPool {
    fn new(num_threads: u8) -> Self {
        let (sender, receiver) = channel::<Box<dyn Fn() + Send>>();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut _handles = vec![];
        for _ in 0..num_threads {
            let clone = receiver.clone();
            let handle = std::thread::spawn(move || loop {
                let work = match clone.lock().unwrap().recv() {
                    Ok(work) => work,
                    _ => break,
                };
                println!("Start");
                work();
                println!("End");
            });
            _handles.push(handle);
        }
        Self { _handles, sender }
    }

    fn execute<T: Fn() + Send + 'static>(&self, work: T) {
        self.sender.send(Box::new(work)).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let pool = ThreadPool::new(4);
        let foo = || std::thread::sleep(std::time::Duration::from_secs(1));
        pool.execute(foo.clone());
        pool.execute(foo);
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}
