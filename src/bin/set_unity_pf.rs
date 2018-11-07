extern crate controller_rs;
extern crate num_complex;
extern crate pcap;
extern crate serde_yaml;

use std::io::Read;
use std::fs::File;
use std::str;
use std::env;
use serde_yaml::{Value, from_str};


use num_complex::Complex;
use pcap::{Capture, Device};

use controller_rs::board_cfg::BoardCfg;


fn main(){
    let mut cap = Capture::from_device(Device {
        name: env::args().nth(1).expect("iface name not found").to_string(),
        desc: None,
    }).unwrap()
        .open()
        .unwrap();

    let mut fparam = File::open(env::args().nth(2).unwrap()).unwrap();
    let mut bytes = Vec::new();
    fparam.read_to_end(&mut bytes).expect("Cannot read file");
    let msg_str = str::from_utf8(&bytes).unwrap().to_string();
    let param = from_str::<Value>(&msg_str).expect("Unable to read param");
    let bc = BoardCfg::from_yaml(&param);

    let pf=vec![vec![vec![Complex::<i16>::new(1,0);2048];8];16];
    bc.update_phase_factor(&mut cap, pf, [0x11, 0x22, 0x33, 0x44, 0x55, 0x66]);
}