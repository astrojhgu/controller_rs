extern crate controller_rs;
extern crate num_complex;
extern crate pnet;
extern crate serde_yaml;
use controller_rs::board_cfg::BoardCfg;
use num_complex::Complex;
use pnet::datalink::interfaces;
use pnet::datalink::{channel, Channel, ChannelType, Config};
use serde_yaml::{from_str, Value};
use std::env;
use std::fs::File;
use std::io::Read;
use std::str;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), std::io::Error> {
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

    let mut fparam = File::open(env::args().nth(2).unwrap()).unwrap();
    let mut bytes = Vec::new();
    fparam.read_to_end(&mut bytes).expect("Cannot read file");
    let msg_str = str::from_utf8(&bytes).unwrap().to_string();
    let param = from_str::<Value>(&msg_str).expect("Unable to read param");
    let bc = BoardCfg::from_yaml(&param);

    bc.turn_off_snap_xgbe(&mut *tx);
    thread::sleep(Duration::from_millis(500));

    bc.set_snap_xgbe_params(&mut *tx);
    thread::sleep(Duration::from_millis(500));

    bc.set_snap_app_params(&mut *tx);
    thread::sleep(Duration::from_millis(500));

    bc.turn_on_snap_xgbe(&mut *tx);

    Ok(())
}
