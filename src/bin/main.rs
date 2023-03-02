use core::num;
use std::{
    collections::HashMap,
    fs,
    io::{prelude::*, BufReader},
    thread,
    time::Duration,
};
extern crate xmttr;
use xmttr::{
    connect_and_read::{get_com_port, mod_main, tcp_main},
    modbusmap::{self, build_hashmap},
    ThreadPool,
};

fn main() {
    let path = String::from("ModbusMap.csv");
    let my_hmap: HashMap<u16, String> = build_hashmap(&path);

    let pool = ThreadPool::new(1);
    //use $/sys/class/tty* to find USB devices
    //let com_list = ["/dev/ttyACM0", "/dev/ttyACM2"];
    let com_list = ["10.27.27.22:502"];
    for device in com_list {
        //let hmap = my_hmap.clone();
        pool.execute(move || {
            tcp_main(device).unwrap();
        });
    }
    
    //println!("Shutting down.");
}
