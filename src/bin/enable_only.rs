extern crate controller_rs;
extern crate num_complex;
extern crate serde_yaml;
extern crate pnet;

use pnet::datalink::interfaces;
use pnet::datalink::{channel, Channel, ChannelType, Config};


use serde_yaml::{from_str, Value};
use std::env;
use std::fs::File;
use std::io::Read;
use std::str;

use num_complex::Complex;


use controller_rs::board_cfg::BoardCfg;

fn main() {
    let dev_name=env::args().nth(1).expect("Dev name not given");
    let dev=interfaces().into_iter().filter(|x|{x.name==dev_name}).nth(0).expect("Cannot find dev");

    let net_cfg = Config {
        write_buffer_size: 65536,
        read_buffer_size: 65536,
        read_timeout: None,
        write_timeout: None,
        channel_type: ChannelType::Layer2,
        bpf_fd_attempts: 1000,
        linux_fanout: None,
    };


    let (mut tx, _) =
        if let Channel::Ethernet(tx, rx) = channel(&dev, net_cfg).expect("canot open channel") {
            (tx, rx)
        } else {
            panic!();
        };

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

    bc.update_phase_factor(&mut *tx, pf);
}
