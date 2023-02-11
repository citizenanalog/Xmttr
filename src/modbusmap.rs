use super::*;
use csv::Reader;
//use std::env;
use std::fs::File;
//use std::path::PathBuf;
pub fn read_map(path: &str) -> Vec<ModbusReg> {
    //let path: PathBuf = std::env::current_dir().unwrap();
    //println!("dir = {:?}", path.display());
    let file: File = File::open(path).unwrap();
    let mut rdr: Reader<File> = Reader::from_reader(file);
    let mut mapdata: Vec<ModbusReg> = Vec::new();
    for result in rdr.records() {
        let record = result.unwrap();
        let reg_type = record.get(0).unwrap().to_string();
        let addr = record
            .get(1)
            .unwrap()
            .to_string()
            .parse::<u16>()
            .ok()
            .unwrap();
        let descrip = record.get(2).unwrap().to_string();
        if reg_type != "" {
            let temp: ModbusReg = ModbusReg::new(reg_type, addr, descrip);
            mapdata.push(temp);
        }
    }
    return mapdata;
}
