use serde_yaml::{from_reader, from_str, Value};
use pcap::{Capture, Active};
use num_complex::Complex;
use crate::msg::adc_msg::AdcMsg;
use crate::msg::adc_msg::CtrlParam;
use crate::net::send_adc_msg;
pub const BOARD_NUM: usize = 16;
pub const ADC_PER_BOARD: usize = 4;

#[derive(Clone, Debug)]
pub struct BoardCfg {
    pub packet_gap: u16,
    pub counter_sync: u8,
    pub counter_wait: u16,
    pub trig_out_delay: u8,
    pub optical_delay: u8,
    pub fft_shift: u16,
    pub truncation: u32,
    pub mac: [[u8; 6]; BOARD_NUM],
    pub xgbid: Vec<Vec<u8>>,
    pub io_delay: [[u8; ADC_PER_BOARD]; BOARD_NUM],
    pub master_board_id:usize,
}

impl BoardCfg {
    pub fn from_yaml(param: &Value) -> BoardCfg {
        let packet_gap = param["packet_gap"]
            .as_u64()
            .expect("Unable to get packet_gap") as u16;
        let counter_wait = param["counter_wait"]
            .as_u64()
            .expect("Unable to get counter_wait") as u16;
        let counter_sync = param["counter_sync"]
            .as_u64()
            .expect("Unable to get counter_sync") as u8;
        let trig_out_delay = param["trig_out_delay"]
            .as_u64()
            .expect("Unable to get trig_out_delay") as u8;
        let optical_delay = param["optical_delay"]
            .as_u64()
            .expect("Unable to get optical_delay") as u8;
        let fft_shift = param["fft_shift"]
            .as_u64()
            .expect("Unable to get fft_shift") as u16;
        let truncation = param["truncation"]
            .as_u64()
            .expect("Unable to get truncation") as u32;
        let master_board_id=param["master_board_id"].as_u64().expect("cannot read master board id") as usize;
        let mut mac = [[0_u8; 6]; BOARD_NUM];
        param["mac"]
            .as_sequence()
            .expect("error, mac cannot be read")
            .iter()
            .enumerate()
            .for_each(|(i, v)| {
                v.as_sequence()
                    .expect("error mac cannot be read")
                    .iter()
                    .enumerate()
                    .for_each(|(j, v)| {
                        mac[i][j] = v.as_u64().expect("mac not u8") as u8;
                    })
            });
        let xgbid: Vec<Vec<u8>> = param["xgbid"]
            .as_sequence()
            .expect("error, xgbid cannot be read")
            .iter()
            .map(|v| {
                v.as_sequence()
                    .expect("error xgbid1 cannot be read")
                    .iter()
                    .map(|v| v.as_u64().expect("xgbid cannot be read") as u8)
                    .collect()
            }).collect();
        let mut io_delay = [[0_u8; ADC_PER_BOARD]; BOARD_NUM];

        param["io_delay"]
            .as_sequence()
            .expect("error, iodelay cannot be read")
            .iter()
            .enumerate()
            .for_each(|(i, v)| {
                v.as_sequence()
                    .expect("error iodelay1 cannot be read")
                    .iter()
                    .enumerate()
                    .for_each(|(j, v)| {
                        io_delay[i][j] = v.as_u64().expect("iodelay not u8") as u8;
                    })
            });
        BoardCfg {
            packet_gap,
            counter_sync,
            counter_wait,
            trig_out_delay,
            optical_delay,
            master_board_id,
            fft_shift,
            truncation,
            mac,
            xgbid,
            io_delay,
        }
    }

    pub fn set_xgbid(&self, cap:&mut Capture<Active>, src_mac:[u8;6]){
        for i in 0..BOARD_NUM{
            let msg=AdcMsg::XGbeId {value:self.xgbid[i].clone()};
            send_adc_msg(cap, &msg, self.mac[i].clone(), src_mac, 1500);
        }
    }

    pub fn set_fft_param(&self, cap:&mut Capture<Active>, src_mac:[u8;6]){
        for i in 0..BOARD_NUM{
            let msg=AdcMsg::FftParam {fft_shift:self.fft_shift, truncation:self.truncation};
            send_adc_msg(cap, &msg, self.mac[i].clone(), src_mac, 1500);
        }
    }

    pub fn update_phase_factor1(&self, cap:&mut Capture<Active>, bid:usize, value:Vec<Vec<Complex<i16>>>, src_mac:[u8;6]){
        let msg=AdcMsg::PhaseFactor {value};
        send_adc_msg(cap, &msg, self.mac[bid].clone(), src_mac, 1500);
    }

    pub fn update_phase_factor(&self, cap:&mut Capture<Active>, value:Vec<Vec<Vec<Complex<i16>>>>, src_mac:[u8;6]){
        self.set_xgbid(cap, src_mac.clone());
        self.set_fft_param(cap, src_mac.clone());
        assert_eq!(value.len(), BOARD_NUM);
        for (bid, pf) in value.into_iter().enumerate(){
            self.update_phase_factor1(cap, bid, pf, src_mac.clone());
        }
        for bid in 0..BOARD_NUM{
            let msg=AdcMsg::Ctrl(CtrlParam::SwitchPhaseFactor);
            send_adc_msg(cap, &msg, self.mac[bid].clone(), src_mac.clone(), 1500);
        }
        let msg=AdcMsg::MasterTrig;
        send_adc_msg(cap, &msg, self.mac[self.master_board_id].clone(), src_mac.clone(), 1500);
    }
}

