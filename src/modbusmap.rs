use super::*;
use csv::{Reader, Writer, WriterBuilder};
use std::error::Error;
use std::fs::File;

pub fn read_map(path: &str) -> Vec<ModbusReg> {
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
pub fn build_hashmap(path: &str) -> HashMap<u16, String> {
    let file: File = File::open(path).unwrap();
    let mut rdr: Reader<File> = Reader::from_reader(file);
    let mut hmap: HashMap<u16, String> = HashMap::new();
    for result in rdr.records() {
        let record = result.unwrap();
        let reg_type = record.get(0);
        //change this to match empty string
        let reg = match reg_type {
            Some(reg) => {
                if "" == reg.to_string() {
                    "U8".to_string() //default type
                } else {
                    reg.to_string()
                }
            }
            None => {
                println!("reg_type: {:?}", reg_type);
                "U8".to_string() //default type
            }
        };

        let addr = record.get(1);
        match addr {
            Some(a) => {
                let address = a.to_string().parse::<u16>().ok().unwrap();
                hmap.entry(address).or_insert(reg);
            }
            None => continue, //ignore cells w/no address
        };
    }
    return hmap;
}

pub fn map_to_csv(hmap: HashMap<u16, String>) -> Result<(), Box<dyn Error>> {
    // Open a file to write the CSV data to
    let file = File::create("hmap.csv")?;

    // Create a CSV writer with a default delimiter of ","
    let mut writer = WriterBuilder::new().delimiter(b',').from_writer(file);

    // Write the HashMap data to the CSV file
    for (address, register) in hmap {
        writer.write_record(&[address.to_string(), register])?;
    }

    Ok(())
}
