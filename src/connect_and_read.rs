use super::*;
use std::{process, thread, time::Duration};
use tokio_modbus::client::Context;
use tokio_serial::SerialPortBuilder;

#[cfg(target_os = "windows")]
pub fn get_com_port() -> &'static str {
    let tty_path: &str = "COM9";
    tty_path
}
#[cfg(target_os = "linux")]
//this could find both ACM0 and ACM1
pub fn get_com_port() -> &'static str {
    let tty_path: &str = "/dev/ttyACM0";
    tty_path
}

#[tokio::main(flavor = "current_thread")]
pub async fn mod_main(
    my_map: &HashMap<u16, String>,
    tty_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    use tokio_modbus::prelude::*;
    use tokio_serial::SerialStream;

    //let tty_path: &str = get_com_port();
    //let tty_path = "/dev/ttyACM0";
    // modbus slave address 111
    let slave: Slave = Slave(0x6F);
    let builder: SerialPortBuilder = tokio_serial::new(tty_path, 38400);
    //error handling for no device
    let port: SerialStream = SerialStream::open(&builder).expect("No device detected");
    let mut ctx: Context = rtu::connect_slave(port, slave).await?; //this connects to modem but not device
    println!("Connected to device at {}", &tty_path);
    //read float register
    let mut count = 0;
    while count < 10 {
        let addr: u16 = 1199;
        // this will hang if no device connected.
        let rsp: Result<Vec<u16>, std::io::Error> = ctx.read_holding_registers(addr, 1).await;
        match rsp {
            Ok(data) => {
                println!("device: {:?} readcount: {}", &tty_path, &count);
                println!("Reg {} type {} returned raw bytes: {:?}", addr, "U16", data);
                //println!("Float value: {}", read_f32_reg(data));
            }
            Err(e) => println!("Reg {} type {} produced: {:?}", addr, "U16", e),
        }
        thread::sleep(Duration::from_millis(1000));
        count = count + 1;
    }

    Ok(())
}
//convert u16 words from xmttr to float
pub fn read_f32_reg(read_bytes: Vec<u16>) -> f32 {
    let msb_word: u16 = read_bytes[0];
    let first_byte: u8 = (msb_word >> 8) as u8;
    let second_byte: u8 = msb_word as u8;
    let lsb_word: u16 = read_bytes[1];
    let third_byte: u8 = (lsb_word >> 8) as u8;
    let fourth_byte: u8 = lsb_word as u8;
    //byte order is (3-4-1-2)
    let new_bytes: [u8; 4] = [third_byte, fourth_byte, first_byte, second_byte];
    //convert be_bytes to float
    let float_value: f32 = f32::from_be_bytes(new_bytes);
    float_value
}

// Asynchronous TCP client 

#[tokio::main(flavor = "current_thread")]
pub async fn tcp_main(tty_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    use tokio_modbus::prelude::*;

    let socket_addr = tty_path.parse().unwrap();

    let mut ctx = tcp::connect(socket_addr).await?;

    let mut count = 0;
    while count < 10 {
        let addr: u16 = 1199;
        // this will hang if no device connected.
        let rsp: Result<Vec<u16>, std::io::Error> = ctx.read_holding_registers(addr, 1).await;
        match rsp {
            Ok(data) => {
                println!("device: {:?} readcount: {}", &tty_path, &count);
                println!("Reg {} type {} returned raw bytes: {:?}", addr, "U16", data);
                //println!("Float value: {}", read_f32_reg(data));
            }
            Err(e) => println!("Reg {} type {} produced: {:?}", addr, "U16", e),
        }
        thread::sleep(Duration::from_millis(1000));
        count = count + 1;
    }

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
pub async fn coriolis_cli(my_map: &HashMap<u16, String>) -> Result<(), Box<dyn std::error::Error>> {
    use std::io;
    use tokio_modbus::prelude::*;
    use tokio_serial::SerialStream;
    let tty_path: &str = get_com_port();
    //let tty_path = "/dev/ttyACM0";
    // modbus slave address 111
    let slave: Slave = Slave(0x6F);
    let builder: SerialPortBuilder = tokio_serial::new(tty_path, 38400);
    //error handling for no device
    let port: SerialStream = SerialStream::open(&builder).expect("No device detected");
    let mut ctx: Context = rtu::connect_slave(port, slave).await?; //this connects to modem but not device
    println!("Connected to device at {}", &tty_path);
    //do cli here

    let input = String::new();
    while input != "x" {
        let mut input = "".to_string();
        println!("\nWhat reg to read? x to quit.");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        println!("user typed: {}", &input);
        match &input.trim().to_string().to_lowercase() == "x" {
            true => {
                println!("user pressed X.");
                //panic!("user exit.");
                break;
            }
            false => (),
        }
        let addr: u16 = input.trim().parse::<u16>().unwrap_or_else(|err| {
            println!("Problem parsing input: {}", err);
            process::exit(1);
        });
        if my_map.contains_key(&addr) {
            //do read on my_map.get
            //addr -= addr;
            match my_map.get(&addr).unwrap().as_str() {
                "F32" => {
                    let rsp: Result<Vec<u16>, std::io::Error> =
                        ctx.read_holding_registers(addr - 1, 2).await;
                    match rsp {
                        Ok(data) => {
                            println!("Reg {} type {} returned raw bytes: {:?}", addr, "F32", data);
                            println!("Float value: {}", read_f32_reg(data));
                        }
                        Err(e) => println!("Reg {} type {} produced: {:?}", addr, "F32", e),
                    }
                }
                "U16" => {
                    let rsp: Result<Vec<u16>, std::io::Error> =
                        ctx.read_holding_registers(addr - 1, 1).await;
                    match rsp {
                        Ok(data) => {
                            println!("Reg {} type {} returned raw bytes: {:?}", addr, "U16", data);
                        }
                        Err(e) => println!("Reg {} type {} produced: {:?}", addr, "U16", e),
                    }
                }
                "U8" => {
                    let rsp: Result<Vec<u16>, std::io::Error> =
                        ctx.read_holding_registers(addr - 1, 1).await;
                    match rsp {
                        Ok(data) => {
                            println!("Reg {} type {} returned raw bytes: {:?}", addr, "U8", data);
                        }
                        Err(e) => println!("Reg {} type {} produced: {:?}", addr, "U8", e),
                    }
                }
                _ => println!(
                    "Not handling type {} for reg {}",
                    my_map.get(&addr).unwrap().as_str(),
                    &addr
                ),
            };
        } else {
            println!("Didn't find {} in map.", addr);
        }
    }

    Ok(())
}
