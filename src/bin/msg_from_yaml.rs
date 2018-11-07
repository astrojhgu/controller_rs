extern crate controller_rs;
extern crate serde_yaml;

use controller_rs::msg::adc_msg::AdcMsg;
use serde_yaml::{from_reader, from_str, Value};
use std::env;
use std::fs::File;
use std::io::Read;
use std::str;

fn main() {
    let mut fparam = File::open(env::args().nth(1).unwrap()).unwrap();
    let mut bytes = Vec::new();
    fparam.read_to_end(&mut bytes).expect("Cannot read file");
    let msg_str = str::from_utf8(&bytes).unwrap().to_string();
    for s in msg_str.split("---") {
        let _ = from_str::<Value>(s).map(|v| {
            let dest: Vec<_> = v["dest"]
                .as_sequence()
                .expect("Error, no dest is assigned")
                .iter()
                .map(|x| x.as_u64().expect("not u8") as u8)
                .collect();
            println!("{:?}", dest);

            let msg = AdcMsg::from_yaml(&v);
            println!("{:?}", msg);
        });
    }
}
