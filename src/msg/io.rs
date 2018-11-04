use serde_yaml::{Mapping, Value};
use std::default::Default;
use num_complex::Complex;
use ::msg::adc_msg::{AdcMsg, CtrlParam};

pub fn load_vec_u64(data: &Value, k: &str) -> Option<Vec<u64>> {
    data[k]
        .as_sequence()
        .map(|x| x.iter().map(|ref x| x.as_u64().unwrap()).collect())
}

pub fn load_vec_u32(data: &Value, k: &str) -> Option<Vec<u32>> {
    data[k]
        .as_sequence()
        .map(|x| x.iter().map(|ref x| x.as_u64().unwrap() as u32).collect())
}

pub fn load_vec_u16(data: &Value, k: &str) -> Option<Vec<u16>> {
    data[k]
        .as_sequence()
        .map(|x| x.iter().map(|ref x| x.as_u64().unwrap() as u16).collect())
}

pub fn load_vec_i16(data: &Value, k: &str) -> Option<Vec<i16>> {
    data[k]
        .as_sequence()
        .map(|x| x.iter().map(|ref x| x.as_i64().unwrap() as i16).collect())
}


pub fn load_vec_u8(data: &Value, k: &str) -> Option<Vec<u8>> {
    data[k]
        .as_sequence()
        .map(|x| x.iter().map(|ref x| x.as_u64().unwrap() as u8).collect())
}

pub fn load_u64(data: &Value, k: &str) -> Option<u64> {
    data[k].as_u64()
}

pub fn load_u32(data: &Value, k: &str) -> Option<u32> {
    data[k].as_u64().map(|x| x as u32)
}

pub fn load_u16(data: &Value, k: &str) -> Option<u16> {
    data[k].as_u64().map(|x| x as u16)
}

pub fn load_u8(data: &Value, k: &str) -> Option<u8> {
    data[k].as_u64().map(|x| x as u8)
}

pub fn store_u64(data: &mut Value, k: &str, v: u64) {
    data[k] = From::from(v);
}

pub fn store_u32(data: &mut Value, k: &str, v: u32) {
    data[k] = From::from(v);
}

pub fn store_u16(data: &mut Value, k: &str, v: u16) {
    data[k] = From::from(v);
}

pub fn store_u8(data: &mut Value, k: &str, v: u8) {
    data[k] = From::from(v);
}

pub fn load_ctrl_param(data:&Value, k:&str)->Option<CtrlParam>{
    match data[k].as_str(){
        Some("Synchronize")=>Some(CtrlParam::Synchronize),
        Some("PreRst")=>Some(CtrlParam::PreRst),
        Some("StartFft")=>Some(CtrlParam::StartFft),
        Some("SwitchPhaseFactor")=>Some(CtrlParam::SwitchPhaseFactor),
        Some("StoreData")=>Some(CtrlParam::StoreData),
        _=>None
    }
}

pub fn store_ctrl_param(data:&mut Value, k:&str, v:CtrlParam){
    match v{
        CtrlParam::Synchronize=>data[k]=From::from("Synchronize"),
        CtrlParam::PreRst=>data[k]=From::from("PreRst"),
        CtrlParam::StartFft=>data[k]=From::from("StartFft"),
        CtrlParam::SwitchPhaseFactor=>data[k]=From::from("SwitchPhaseFactor"),
        CtrlParam::StoreData=>data[k]=From::from("StoreData"),
    }
}

#[allow(unused_macros)]
macro_rules! yaml_io{
    ($(($name:ident, $storer:ident, $loader:ident)),*)=>{
        fn from_yaml(cfg:&Value)->Self{
            let mut result:Self=Default::default();
            $(
            result.$name=$loader(cfg, stringify!($name)).unwrap_or_else(||{
                eprintln!("WARNING: {} not found, use 0", stringify!($name) );
                0
            });
            )*
            result
        }

        fn to_yaml(&self)->Value{
            let mut result=Value::Mapping(Mapping::new());
            $(
                $storer(&mut result, stringify!($name), self.$name);
            )*
            result
        }
    }
}

impl AdcMsg{
    pub fn from_yaml(cfg: &Value)->Option<AdcMsg>{
        let typecode=cfg["MsgType"].as_str();
        match typecode{
            Some("MasterSync")=>Some(AdcMsg::MasterSync),
            Some("MasterRst")=>Some(AdcMsg::MasterRst),
            Some("MasterTrig")=>Some(AdcMsg::MasterTrig),
            Some("Ctrl")=>{
                match load_ctrl_param(cfg, "param"){
                    Some(p)=>Some(AdcMsg::Ctrl(p)),
                    _=>None,
                }
            }
            Some("UploadData")=>Some(AdcMsg::UploadData),
            Some("FftParam")=>{
                let fft_shift=cfg["fft_shift"].as_u64().unwrap() as u16;
                let truncation=cfg["truncation"].as_u64().unwrap() as u32;
                Some(AdcMsg::FftParam{fft_shift, truncation})
            }
            Some("PhaseFactor")=>{                
                match cfg["value"].as_sequence(){
                    Some(vv)=>{
                        let mut result=Vec::<Vec<Complex<i16> > >::new();
                        for v in vv{
                            let tmp:Vec<i16>=v.as_sequence().unwrap().iter().map(|x|{x.as_i64().unwrap() as i16}).collect();
                            result.push(tmp.chunks(2).map(|x|{
                                Complex::<i16>::new(x[0], x[1])
                            }).collect());
                        }
                        Some(AdcMsg::PhaseFactor{value:result})
                    }
                    _=>None,
                }
            }
            Some("QueryState")=>Some(AdcMsg::QueryState),
            Some("XGbeId")=>{
                match cfg["value"].as_sequence(){
                    Some(v)=>{
                        Some(AdcMsg::XGbeId{value:v.iter().map(|v|{v.as_u64().unwrap() as u8}).collect()})
                    }
                    _=>None
                }
            }
            _=>None,
        }
    }
}