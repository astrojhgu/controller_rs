extern crate controller_rs;
extern crate serde_yaml;

use serde_yaml::{from_reader, from_str, Value};
use controller_rs::msg::adc_msg::AdcMsg;
use std::io::Read;
use std::fs::File;
use std::str;
use std::env;

fn main(){
    let mut fparam=File::open(env::args().nth(1).unwrap()).unwrap();
    let mut bytes = Vec::new();
    fparam.read_to_end(&mut bytes).expect("Cannot read file");
    let msg_str = str::from_utf8(&bytes).unwrap().to_string();
    for s in msg_str.split("---") {
        let _ = from_str::<Value>(s).map(|v| {
            let msg=AdcMsg::from_yaml(&v);
            println!("{:?}", msg);
        });
    }
}