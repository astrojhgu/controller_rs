extern crate controller_rs;
extern crate num_complex;
extern crate pcap;
extern crate serde_yaml;
use controller_rs::board_cfg::{BoardCfg, BOARD_NUM};
use controller_rs::msg::adc_msg::AdcMsg;
use controller_rs::msg::adc_msg::CtrlParam;
use controller_rs::net::send_adc_msg;
use num_complex::Complex;
use pcap::{Capture, Device};
use serde_yaml::{from_str, Value};
use std::env;
use std::fs::File;
use std::io::Read;
use std::str;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), std::io::Error> {
    let mut cap = Capture::from_device(Device {
        name: env::args()
            .nth(1)
            .expect("iface name not found")
            .to_string(),
        desc: None,
    })
    .unwrap()
    .open()
    .unwrap();

    let mut fparam = File::open(env::args().nth(2).unwrap()).unwrap();
    let mut bytes = Vec::new();
    fparam.read_to_end(&mut bytes).expect("Cannot read file");
    let msg_str = str::from_utf8(&bytes).unwrap().to_string();
    let param = from_str::<Value>(&msg_str).expect("Unable to read param");
    let bc = BoardCfg::from_yaml(&param);

    bc.reset_all(&mut cap);

    /*
    //rst each board f3 01
    for i in 0..BOARD_NUM {
        let msg = AdcMsg::Ctrl(CtrlParam::PreRst);
        send_adc_msg(&mut cap, &msg, bc.mac[i], bc.src_mac, 1500).expect("sent error");
    }

    //rst master board f1 01
    send_adc_msg(
        &mut cap,
        &AdcMsg::MasterRst,
        bc.mac[bc.master_board_id],
        bc.src_mac,
        1500,
    )
        .expect("sent error");

     */

    bc.sync_adc(&mut cap);
    /*

    //each board Iddr rst f3 05
    for i in 0..BOARD_NUM {
        let msg = AdcMsg::Ctrl(CtrlParam::IddrRst);
        send_adc_msg(&mut cap, &msg, bc.mac[i], bc.src_mac, 1500).expect("sent error");
    }


    //master trig f2 01..
    send_adc_msg(
        &mut cap,
        &AdcMsg::MasterTrig,
        bc.mac[bc.master_board_id],
        bc.src_mac,
        1500,
    )
        .expect("sent error");

    //sync f3 00
    for i in 0..BOARD_NUM {
        let msg = AdcMsg::Ctrl(CtrlParam::Synchronize);
        send_adc_msg(&mut cap, &msg, bc.mac[i], bc.src_mac, 1500).expect("sent error");
    }

    //master sync f0 01 ...
    send_adc_msg(
        &mut cap,
        &AdcMsg::MasterSync,
        bc.mac[bc.master_board_id],
        bc.src_mac,
        1500,
    )
        .expect("sent error");
     */

    bc.set_adc_params(&mut cap);
    /*


        for i in 0..BOARD_NUM {
            let msg = AdcMsg::Cfg {
                io_delay: bc.io_delay[i],
                packet_gap: bc.packet_gap,
                counter_sync: bc.counter_sync,
                counter_wait: bc.counter_wait,
                trig_out_delay: bc.trig_out_delay,
                optical_delay: bc.optical_delay,
            };
            send_adc_msg(&mut cap, &msg, bc.mac[i], bc.src_mac, 1500).expect("sent error");
        }
    //return Ok(());

         */

    bc.turn_off_snap_xgbe(&mut cap);
    bc.set_snap_xgbe_params(&mut cap);
    bc.set_snap_app_params(&mut cap);
    bc.turn_on_snap_xgbe(&mut cap);

    bc.set_xgbeid(&mut cap);

    bc.set_fft_param(&mut cap);

    let init_phase_factors = vec![vec![vec![Complex::<i16>::new(1, 0); 2048]; 8]; 16];

    bc.update_phase_factor(&mut cap, init_phase_factors);

    bc.wait_for_trig(&mut cap);

    thread::sleep(Duration::from_millis(5000));

    bc.send_internal_trig(&mut cap);

    Ok(())
}
