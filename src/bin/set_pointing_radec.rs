extern crate controller_rs;
extern crate num_complex;
extern crate pnet;
extern crate serde_yaml;
extern crate astroalgo;
extern crate chrono;
extern crate num_traits;

use num_traits::float::FloatConst;

use chrono::offset::Utc;

use astroalgo::earth_position::LonLat;
use astroalgo::hzpoint::HzPoint;
use astroalgo::eqpoint::EqPoint;
use astroalgo::coord_trans;
use astroalgo::quant::Angle;

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
        eprintln!("Usage: {} <ra> <dec> <outfile>", args[0]);
        std::process::exit(-1);
    }

    let ra = args[1].parse::<f64>().unwrap();
    let dec = args[2].parse::<f64>().unwrap();
    let eqp=EqPoint::from_radec(Angle::from_deg(ra), Angle::from_deg(dec));
    let obs=LonLat::from_ll(Angle::from_deg(86.430254), Angle::from_deg(42.552728));
    let now=Utc::now().naive_utc();
    let hzp=eqp.hzpoint_at(obs, now);

    let az=hzp.az.0;
    let ze=f64::PI()/2.0-hzp.alt.0;

    eprintln!("az={} ze={}", az.to_degrees(), ze.to_degrees());

    let nx = ze.sin() * az.sin();
    let ny = ze.sin() * az.cos();
    let nz = ze.cos();

    println!("dir={} {} {}", nx, ny, nz);

    let ants = controller_rs::ant_pos::ant_pos();
    let mut pf = vec![vec![vec![Complex::<i16>::new(0, 0); 2048]; 8]; 16];
    let mut aid = 0;

    for (i, b) in pf[0..16].iter_mut().enumerate() {
        for p in &mut b[0..8] {
            if aid >= ants.len() {
                break;
            }
            if aid % 8 == 0 /*|| aid < 86*/ {
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
