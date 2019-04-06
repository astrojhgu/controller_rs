extern crate controller_rs;
extern crate num_complex;
extern crate pnet;

use pnet::datalink::interfaces;
use pnet::datalink::{channel, Channel, ChannelType, Config};

extern crate serde_yaml;
use controller_rs::board_cfg::BoardCfg;
use serde_yaml::{from_str, Value};
use std::env;
use std::fs::File;
use std::io::Read;
use std::str;
use std::time::Duration;
use std::io::Write;
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
        read_timeout: Some(Duration::from_millis(10)),
        write_timeout: None,
        channel_type: ChannelType::Layer2,
        bpf_fd_attempts: 1000,
        linux_fanout: None,
    };

    let (mut tx, mut rx) =
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

    bc.store_data(&mut *tx);

    if let Some(fft_data) = bc.fetch_fft_data(&mut *tx, &mut *rx){
        let mut outfile=File::create("fft_data.dat").unwrap();
        for v in &fft_data{
            let nbytes=v.len()*8;
            let raw_data=unsafe{std::slice::from_raw_parts(v.as_ptr() as *const u8, nbytes)};
            //println!("{} {}", nbytes, v.len());
            outfile.write_all(raw_data);
        }
        println!("{}", fft_data.len());
        Ok(())
    }else{
        Err(std::io::Error::new(std::io::ErrorKind::Other, "data incomplete"))
    }
}
