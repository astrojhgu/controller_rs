extern crate controller_rs;
extern crate pcap;
use controller_rs::net::{send_raw_buffer, send_udp_buffer};
use pcap::{Capture, Device};

fn main() {
    let aa=vec![0;6];
    let mut cap=Capture::from_device(Device{name:"lo".to_string(), desc:None}).unwrap().open().unwrap();
    //send_buffer(&mut cap, 0xfa, &aa, [0;6], [0;6], 1500);
    send_udp_buffer(&mut cap, &aa[..], [0x11,0x22,0x33,0x44,0x55,0x66], [0x22,0x33,0x44,0x55,0x66,0x77],
    [100,0,0,228],[100,0,0,100],1234,1234,1500);

}
