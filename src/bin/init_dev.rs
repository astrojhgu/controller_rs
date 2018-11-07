extern crate controller_rs;
extern crate serde_yaml;
extern crate pcap;

use pcap::{Capture, Device};

use controller_rs::board_cfg::{BoardCfg, BOARD_NUM};
use controller_rs::msg::adc_msg::AdcMsg;
use controller_rs::msg::adc_msg::CtrlParam;
use controller_rs::net::send_adc_msg;
use serde_yaml::{from_reader, from_str, Value};
use std::clone::Clone;
use std::env;
use std::fs::File;
use std::io::Read;
use std::str;

fn main() ->Result<(), std::io::Error>{
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

    for i in 0..BOARD_NUM{
        let msg=AdcMsg::Ctrl(CtrlParam::PreRst);
        send_adc_msg(&mut cap, &msg, bc.mac[i].clone(), [0x66, 0x55, 0x44, 0x33, 0x22, 0x11], 1500);
    }

    send_adc_msg(&mut cap, &AdcMsg::MasterRst, bc.mac[bc.master_board_id].clone(), [0x66, 0x55, 0x44, 0x33, 0x22, 0x11], 1500);

    for i in 0..BOARD_NUM {
        let msg = AdcMsg::Cfg {
            io_delay: bc.io_delay[i].clone(),
            packet_gap: bc.packet_gap,
            counter_sync: bc.counter_sync,
            counter_wait: bc.counter_wait,
            trig_out_delay: bc.trig_out_delay,
            optical_delay: bc.optical_delay,
        };
        send_adc_msg(&mut cap, &msg, bc.mac[i].clone(), [0x66, 0x55, 0x44, 0x33, 0x22, 0x11], 1500);
    }
    Ok(())
}
