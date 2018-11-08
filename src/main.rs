#![allow(unused_imports)]

extern crate controller_rs;
extern crate num_complex;
extern crate pcap;

use controller_rs::msg::adc_msg;
use controller_rs::net::{send_adc_msg, send_raw_buffer, send_udp_buffer};
use num_complex::Complex;
use pcap::{Capture, Device};

fn main() {
    let mut cap = Capture::from_device(Device {
        name: "enp0s20f0u1u4".to_string(),
        desc: None,
    }).unwrap()
    .open()
    .unwrap();
    //let msg=adc_msg::AdcMsg::Ctrl(adc_msg::CtrlParam::PreRst);
    /*
    let msg = adc_msg::AdcMsg::Cfg {
        io_delay: [6, 6, 6, 6],
        packet_gap: 2000,
        counter_wait: 639,
        trig_out_delay: 5,
        counter_sync: 10,
        optical_delay: 15,
    };
    */
    let mut phase_phases = vec![Vec::<Complex<i16>>::new(); 8];

    for i in 0..8 {
        for _j in 0..2048 {
            phase_phases[i].push(Complex::<i16>::new(1, 0));
        }
    }

    let msg = adc_msg::AdcMsg::PhaseFactor {
        value: phase_phases,
    };

    //let msg=adc_msg::AdcMsg::MasterRst;
    //send_raw_buffer(&mut cap, )
    send_adc_msg(
        &mut cap,
        &msg,
        [0x11, 0x22, 0x33, 0x44, 0x55, 0x66],
        [0x66, 0x55, 0x44, 0x33, 0x22, 0x11],
        1500,
    ).expect("sent error");
}
