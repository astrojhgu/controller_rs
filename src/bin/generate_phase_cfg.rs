#![allow(unused_imports)]

extern crate controller_rs;
extern crate serde_yaml;

use controller_rs::msg::adc_msg::AdcMsg;
use serde_yaml::{from_reader, from_str, Value};
use std::env;
use std::fs::File;
use std::io::Read;
use std::str;

fn main() {}
