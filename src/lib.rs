pub mod connect_and_read;
pub mod modbusmap;
use std::collections::HashMap;

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
    use crate::connect_and_read::read_f32_reg;
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
        let my_hmap: HashMap<u16,String> = build_hashmap(&path);
        //for loop if we want to call mod_main multiple times
        
            //match connect::read(252) {
            match mod_main(&my_hmap) {
                Ok(_res) => {
                    println!("success!");
                    assert_eq!(1,1);
                    //println!("ok {:?}", res);
                }
                Err(e) => println!("error  {:?}", e),
            }
    }
    #[test]
    fn build_hashmap() {
        use std::collections::HashMap;
        use super::modbusmap::build_hashmap;
        let path = String::from("ModbusMap.csv");
        let my_hmap: HashMap<u16,String> = build_hashmap(&path);
        let reg: u16 = 247; //mass flow rate should return type F32
        let reg_type: String = my_hmap.get(&reg).unwrap().to_string();
        assert_eq!("F32", reg_type);
    }
}
