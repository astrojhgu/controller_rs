extern crate controller_rs;
extern crate pcap;
use controller_rs::msg::adc_msg;
use controller_rs::net::{send_adc_msg, send_raw_buffer, send_udp_buffer};
use pcap::{Capture, Device};

fn main() {
    let mut cap = Capture::from_device(Device {
        name: "enp0s20f0u1u4".to_string(),
        desc: None,
    }).unwrap()
    .open()
    .unwrap();
    //let msg=adc_msg::AdcMsg::Ctrl(adc_msg::CtrlParam::PreRst);
    let msg = adc_msg::AdcMsg::Cfg {
        io_delay: [6, 6, 6, 6],
        packet_gap: 2000,
        counter_wait: 639,
        trig_out_delay: 5,
        counter_sync: 10,
        optical_delay: 15,
    };
    //let msg=adc_msg::AdcMsg::MasterRst;
    //send_raw_buffer(&mut cap, )
    send_adc_msg(
        &mut cap,
        &msg,
        [0x11, 0x22, 0x33, 0x44, 0x55, 0x66],
        [0x66, 0x55, 0x44, 0x33, 0x22, 0x11],
        1500,
    );
}
