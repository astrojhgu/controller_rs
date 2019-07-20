extern crate controller_rs;
extern crate num_complex;
extern crate pnet;
extern crate serde_yaml;

use pnet::datalink::interfaces;
use pnet::datalink::{channel, Channel, ChannelType, Config};

use num_complex::Complex;
use serde_yaml::{from_str, Value};
use std::env;
use std::fs::File;
use std::io::Read;
use std::str;

use controller_rs::board_cfg::BoardCfg;

fn main() {
    let dev_name = env::args().nth(1).expect("Dev name not given");
    let dev = interfaces()
        .into_iter()
        .filter(|x| x.name == dev_name)
        .nth(0)
        .expect("Cannot find dev");

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

    let mut phase_file = File::open(env::args().nth(3).expect("phase file not given"))
        .expect("phase file open failed");

    let mut fparam = File::open(env::args().nth(2).unwrap()).unwrap();
    let mut bytes = Vec::new();
    fparam.read_to_end(&mut bytes).expect("Cannot read file");
    let msg_str = str::from_utf8(&bytes).unwrap().to_string();
    let param = from_str::<Value>(&msg_str).expect("Unable to read param");
    let bc = BoardCfg::from_yaml(&param);

    let mut pf = vec![vec![vec![Complex::<i16>::new(0, 0); 2048]; 8]; 16];

    for bid in 0..16 {
        for pid in 0..8 {
            let raw = unsafe {
                std::slice::from_raw_parts_mut(pf[bid][pid].as_mut_ptr() as *mut u8, 2048 * 4)
            };
            match phase_file.read(raw) {
                Ok(s) if s == 2048 * 4 => {}
                _ => panic!("Error in read phase file"),
            }
        }
    }

    //pf[bid][pid] = vec![Complex::<i16>::new(16384, 0); 2048];

    bc.update_phase_factor(&mut *tx, pf);
    //bc.send_internal_trig(&mut *tx);
}
