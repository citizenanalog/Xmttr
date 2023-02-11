use super::*;
use std::{thread, time::Duration};
use tokio_modbus::client::Context;
use tokio_serial::SerialPortBuilder;
#[tokio::main(flavor = "current_thread")]
pub async fn mod_main(my_map: &Vec<ModbusReg>) -> Result<(), Box<dyn std::error::Error>> {
    use tokio_modbus::prelude::*;
    use tokio_serial::SerialStream;
    //let tty_path: &str = "COM9";
    let tty_path = "/dev/ttyACM0";
    // modbus slave address 111
    let slave: Slave = Slave(0x6F);
    let builder: SerialPortBuilder = tokio_serial::new(tty_path, 38400);
    //error handling for no device
    let port: SerialStream = SerialStream::open(&builder).expect("No device detected");
    let mut ctx: Context = rtu::connect_slave(port, slave).await?;
    println!("Connected to device at {}", &tty_path);
    //read float register
    let mut count = 0;
    while count < 10 {
        let addr: u16 = 246;
        let rsp: Result<Vec<u16>, std::io::Error> = ctx.read_holding_registers(addr, 2).await;
        match rsp {
            Ok(data) => {
                println!("Reg {} type {} returned raw bytes: {:?}", addr, "U16", data);
                println!("Float value: {}", read_f32_reg(data));
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
