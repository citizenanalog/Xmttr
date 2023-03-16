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
    pool::ThreadPool,
};

fn main() {
    #[cfg(target_os = "windows")]
    pub fn get_com_port() -> &'static str {
        let com_list: &str = "COM9";
        com_list
    }
    #[cfg(target_os = "linux")]
    pub fn get_com_port() -> &'static str {
        let com_list = ["/dev/ttyACM0", "/dev/ttyACM1"];
        com_list
    }

    let path = String::from("ModbusMap.csv");
    let my_hmap: HashMap<u16, String> = build_hashmap(&path);

    let pool = ThreadPool::new(1);
    //let com_list = ["/dev/ttyACM0", "/dev/ttyACM1"];

    let com_list = vec![get_com_port()];
    for device in com_list {
        let hmap = my_hmap.clone();
        pool.execute(move || {
            mod_main(&hmap, device.as_ref()).unwrap();
        });
    }

    println!("Shutting down.");
}
