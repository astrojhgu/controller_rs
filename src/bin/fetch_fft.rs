extern crate controller_rs;
extern crate num_complex;
extern crate pcap;
extern crate serde_yaml;
use controller_rs::board_cfg::{BoardCfg, BOARD_NUM};
use controller_rs::msg::adc_msg::AdcMsg;
use controller_rs::msg::adc_msg::CtrlParam;
use controller_rs::net::send_adc_msg;
use num_complex::Complex;
use pcap::{Capture, Device};
use serde_yaml::{from_str, Value};
use std::env;
use std::fs::File;
use std::io::Read;
use std::str;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), std::io::Error> {
    let mut cap = Capture::from_device(Device {
        name: env::args()
            .nth(1)
            .expect("iface name not found")
            .to_string(),
        desc: None,
    })
        .unwrap()
        .timeout(10)
    .open()
    .unwrap();

    let mut fparam = File::open(env::args().nth(2).unwrap()).unwrap();
    let mut bytes = Vec::new();
    fparam.read_to_end(&mut bytes).expect("Cannot read file");
    let msg_str = str::from_utf8(&bytes).unwrap().to_string();
    let param = from_str::<Value>(&msg_str).expect("Unable to read param");
    let bc = BoardCfg::from_yaml(&param);

    bc.store_data(&mut cap);

    bc.fetch_fft_data1(0, &mut cap);

    let mut cnt=0;
    while let Ok(packet)=cap.next(){
        cnt+=1;
        println!("{} {}",cnt, packet.len());
    }

    Ok(())
}
