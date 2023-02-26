pub mod connect_and_read;
pub mod modbusmap;
use std::collections::HashMap;
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
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
        let thread = thread::spawn(move || loop {
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
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ModbusReg {
    reg_type: String,
    addr: u16,
    descrip: String,
}
impl ModbusReg {
    fn new(reg_type: String, addr: u16, descrip: String) -> ModbusReg {
        ModbusReg {
            reg_type,
            addr,
            descrip,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{connect_and_read::read_f32_reg, modbusmap::map_to_csv};
    #[test]
    fn float_read() {
        let my_word: Vec<u16> = vec![51867, 15673];
        let my_zero: Vec<u16> = vec![0, 0];
        assert_eq!(read_f32_reg(my_word), 0.045359235);
        assert_eq!(read_f32_reg(my_zero), 0.0);
    }

    #[test]
    fn connection_test() {
        use super::*;
        use connect_and_read::mod_main;
        use modbusmap::build_hashmap;
        println!("Running Main");
        //let res = vec![0 as u16];
        let path = String::from("ModbusMap.csv");
        let my_hmap: HashMap<u16, String> = build_hashmap(&path);
        //for loop if we want to call mod_main multiple times

        let tty_path: &str = "/dev/ttyACM0";
        match mod_main(&my_hmap, &tty_path) {
            Ok(_res) => {
                println!("success!");
                assert_eq!(1, 1);
                //println!("ok {:?}", res);
            }
            Err(e) => println!("error  {:?}", e),
        }
    }
    #[test]
    fn build_hashmap() {
        use super::modbusmap::build_hashmap;
        use std::collections::HashMap;
        //let path = String::from("ModbusMap.csv");
        let path = String::from("map3.csv");
        let my_hmap: HashMap<u16, String> = build_hashmap(&path);
        let reg: u16 = 247; //mass flow rate should return type F32
        let reg_type: String = my_hmap.get(&reg).unwrap().to_string();
        assert_eq!("F32", reg_type);
        println!("built map of length: {:?}", my_hmap.len());
        let mut sorted_values: Vec<(&u16, &String)> = my_hmap.iter().collect();
        sorted_values.sort_by_key(|k| k.0);
        println!("sorted by address: {:?}", sorted_values);
        if let Err(e) = map_to_csv(my_hmap) {
            println!("error: {}", e);
        }
    }

    #[test]
    //#[ignore] //call this test alone
    fn cli_test() {
        use super::*;
        use connect_and_read::coriolis_cli;
        use modbusmap::build_hashmap;
        println!("Running Main");
        //let res = vec![0 as u16];
        let path = String::from("ModbusMap.csv");
        let my_hmap: HashMap<u16, String> = build_hashmap(&path);
        let x = String::from("247").parse::<u16>().unwrap();
        println!("reg 247: {:?}", my_hmap.get(&x).unwrap().as_str());
        //match connect::read(252) {
        match coriolis_cli(&my_hmap) {
            Ok(_res) => {
                println!("success!");
                assert_eq!(1, 1);
                //println!("ok {:?}", res);
            }
            Err(e) => println!("error  {:?}", e),
        }
    }
}
