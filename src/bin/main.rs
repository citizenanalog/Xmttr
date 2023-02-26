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
    connect_and_read::{get_com_port, mod_main},
    modbusmap::{self, build_hashmap},
    ThreadPool,
};

fn main() {
    let path = String::from("ModbusMap.csv");
    let my_hmap: HashMap<u16, String> = build_hashmap(&path);

    let pool = ThreadPool::new(2);
    let com_list = ["/dev/ttyACM0", "/dev/ttyACM1"];
    for device in com_list {
        let hmap = my_hmap.clone();
        pool.execute(move || {
            mod_main(&hmap, device);
        });
    }

    println!("Shutting down.");
}
