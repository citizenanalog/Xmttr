pub mod connect_and_read;
pub mod modbusmap;

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
        use modbusmap::read_map;
        println!("Running Main");
        //let res = vec![0 as u16];
        let path = String::from("ModbusMap.csv");
        let my_map: Vec<ModbusReg> = read_map(&path);
        //for loop if we want to call mod_main multiple times
        for i in 1..2 {
            //match connect::read(252) {
            match mod_main(&my_map) {
                Ok(_res) => {
                    println!("success!");
                    //println!("ok {:?}", res);
                }
                Err(e) => println!("error {} {:?}", i, e),
            }
        }
    }
}
