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
    connect_and_read::*,
    modbusmap::{self, build_hashmap},
    pool::ThreadPool,
    setup,
};

fn main() {
    //read the config file
    let new_config = setup::read_config();

    let path = String::from("ModbusMap.csv");
    let my_hmap: HashMap<u16, String> = build_hashmap(&path);

    //let pool = ThreadPool::new(1);
    //let com_list = ["/dev/ttyACM0", "/dev/ttyACM1"];

    //unpack the config here
    let com_list = new_config.ComPort;
    let mb_addr = new_config.ModbusAddress;
    let interval = new_config.log_interval;
    let hmap = my_hmap.clone();
    let regs = new_config.Regs.clone();
    if let Err(e) = logger(&hmap, &com_list[0], mb_addr.clone(), regs, interval.clone()) {
        println!("error occurred. {:?}", e);
    }

    println!("Shutting down.");
}
