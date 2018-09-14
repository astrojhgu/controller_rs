extern crate controller_rs;
extern crate pcap;
use std::slice;
use controller_rs::net::send_buffer;
use pcap::{Capture, Device};

fn main() {
    let aa=vec![0;0];
    let mut cap=Capture::from_device(Device{name:"lo".to_string(), desc:None}).unwrap().open().unwrap();
    send_buffer(&mut cap, 0xfa, &aa, [0;6], [0;6], 1500);

}
