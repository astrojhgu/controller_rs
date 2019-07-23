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

const PF_AMPL: f64 = 16384.0;
const LIGHT_SPEED: f64 = 2.99792458e8;
fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <az> <zenith> <outfile>", args[0]);
        std::process::exit(-1);
    }

    let az = args[1].parse::<f64>().unwrap().to_radians();
    let ze = args[2].parse::<f64>().unwrap().to_radians();
    let nx = ze.sin() * az.sin();
    let ny = ze.sin() * az.cos();
    let nz = ze.cos();

    println!("dir={} {} {}", nx, ny, nz);

    let ants = controller_rs::ant_pos::ant_pos();
    let mut pf = vec![vec![vec![Complex::<i16>::new(0, 0); 2048]; 8]; 16];
    let mut aid = 0;

    for b in &mut pf[0..16] {
        for p in &mut b[0..8] {
            if aid >= ants.len() {
                break;
            }
            if aid % 8 == 0 {
                aid += 1;
                continue;
            }
            let ant = ants[aid].clone();

            let delay = ant[0] * nx + ant[1] * ny + ant[2] * nz;

            for (ch, c) in p.iter_mut().enumerate() {
                //*c=Complex::new(1,0);
                //if(ch%2==0){
                let freq = 250e6 / 2048.0 * ch as f64;

                let lambda = LIGHT_SPEED / freq;

                let p = delay / lambda * 2.0 * 3.14159265358979323846;

                *c = Complex::<i16>::new((PF_AMPL * p.cos()) as i16, (PF_AMPL * p.sin()) as i16);
                //}
            }
            aid += 1;
        }
    }

    let mut phase_file = File::create(args[3].clone()).expect("Failed to create phase file");

    for bid in 0..16 {
        for pid in 0..8 {
            let raw =
                unsafe { std::slice::from_raw_parts(pf[bid][pid].as_ptr() as *const u8, 2048 * 4) };
            match phase_file.write(raw) {
                Ok(s) if s == 2048 * 4 => {}
                _ => panic!("Error in writing phase file"),
            }
        }
    }
}
