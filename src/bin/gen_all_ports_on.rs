extern crate controller_rs;
extern crate num_complex;
extern crate pnet;
extern crate serde_yaml;

use pnet::datalink::interfaces;
use pnet::datalink::{channel, Channel, ChannelType, Config};

use serde_yaml::{from_str, Value};
use std::env;
use std::fs::File;
use std::io::Write;
use std::str;

use num_complex::Complex;

use controller_rs::board_cfg::BoardCfg;

fn main() {
    let bid: usize = env::args()
        .nth(1)
        .expect("bid miss")
        .parse()
        .expect("bad bid");
    let pid: usize = env::args()
        .nth(2)
        .expect("pid miss")
        .parse()
        .expect("bad pid");

    let mut pf = vec![vec![vec![Complex::<i16>::new(0, 0); 2048]; 8]; 16];
    for b in &mut pf[0..16]{
        for p in &mut b[0..8]{
            for (ch,c) in p.iter_mut().enumerate(){
                //*c=Complex::new(1,0);
                //if(ch%2==0){
                *c=Complex::<i16>::new(16384, 0);
                //}
            }
        }
    }

    let mut phase_file=File::create(env::args().nth(3).expect("out file name not given")).expect("Failed to create phase file");

    for bid in 0..16{
        for pid in 0..8{
            let raw = unsafe {
                std::slice::from_raw_parts(pf[bid][pid].as_ptr() as *const u8, 2048*4)
            };
            match phase_file.write(raw) {
                Ok(s) if s == 2048*4 => {},
                _ => panic!("Error in writing phase file"),
            }
        }
    }
}
