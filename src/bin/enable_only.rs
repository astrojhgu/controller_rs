extern crate controller_rs;
extern crate num_complex;
extern crate pcap;
extern crate serde_yaml;

use serde_yaml::{from_str, Value};
use std::env;
use std::fs::File;
use std::io::Read;
use std::str;

use num_complex::Complex;
use pcap::{Capture, Device};

use controller_rs::board_cfg::BoardCfg;

fn main() {
    let mut cap = Capture::from_device(Device {
        name: env::args()
            .nth(1)
            .expect("iface name not found")
            .to_string(),
        desc: None,
    }).unwrap()
    .open()
    .unwrap();

    let bid: usize = env::args()
        .nth(3)
        .expect("bid miss")
        .parse()
        .expect("bad bid");
    let pid: usize = env::args()
        .nth(4)
        .expect("pid miss")
        .parse()
        .expect("bad pid");

    let mut fparam = File::open(env::args().nth(2).unwrap()).unwrap();
    let mut bytes = Vec::new();
    fparam.read_to_end(&mut bytes).expect("Cannot read file");
    let msg_str = str::from_utf8(&bytes).unwrap().to_string();
    let param = from_str::<Value>(&msg_str).expect("Unable to read param");
    let bc = BoardCfg::from_yaml(&param);

    let mut pf = vec![vec![vec![Complex::<i16>::new(0, 0); 2048]; 8]; 16];
    pf[bid][pid] = vec![Complex::<i16>::new(1, 0); 2048];

    bc.update_phase_factor(&mut cap, pf);
}
