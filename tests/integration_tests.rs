pub mod common;
mod tests {
    extern crate xmttr;
    use std::collections::HashMap;
    use xmttr::connect_and_read::mod_main;
    use xmttr::connect_and_read::read_f32_reg;
    use xmttr::modbusmap::build_hashmap;

    #[test]
    fn float_read() {
        //common::setup();
        let my_word: Vec<u16> = vec![51867, 15673];
        let my_zero: Vec<u16> = vec![0, 0];
        assert_eq!(read_f32_reg(my_word), 0.045359235);
        assert_eq!(read_f32_reg(my_zero), 0.0);
    }

    #[test]
    fn connection_test() {
        //common::setup();
        let path = String::from("ModbusMap.csv");
        let my_hmap: HashMap<u16, String> = build_hashmap(&path);
        //match connect::read(252) {
        match mod_main(&my_hmap) {
            Ok(_res) => {
                println!("success!");
                assert_eq!(1, 1);
                //println!("ok {:?}", res);
            }
            Err(e) => println!("error  {:?}", e),
        }
    }
    #[test]
    fn hashmap() {
        //common::setup();
        let path = String::from("ModbusMap.csv");
        let my_hmap: HashMap<u16, String> = build_hashmap(&path);
        let reg: u16 = 247; //mass flow rate should return type F32
        let reg_type: String = my_hmap.get(&reg).unwrap().to_string();
        assert_eq!("F32", reg_type);
    }
}
