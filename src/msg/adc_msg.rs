#![allow(clippy::needless_range_loop)]
#![allow(clippy::identity_op)]
use num_complex::Complex;

const PORT_PER_BOARD: usize = 8;
//const NUM_CH: usize = 2048;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CtrlParam {
    Synchronize,
    PreRst,
    StartFft,
    SwitchPhaseFactor,
    StoreData,
    IddrRst,
}

impl CtrlParam {
    pub fn param_code(self) -> u8 {
        match self {
            CtrlParam::Synchronize => 0,
            CtrlParam::PreRst => 1,
            CtrlParam::StartFft => 2,
            CtrlParam::SwitchPhaseFactor => 3,
            CtrlParam::StoreData => 4,
            CtrlParam::IddrRst => 5,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum XGbeIdParam {
    Upper {
        mac1: [u8; 6],
        mac2: [u8; 6],
    },
    Lower {
        mac1: [u8; 6],
        mac2: [u8; 6],
        mac3: [u8; 6],
        mac4: [u8; 6],
        ip1: [u8; 4],
        ip2: [u8; 4],
        port1: u16,
        port2: u16,
    },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AdcMsg {
    MasterSync,
    MasterRst,
    MasterTrig,
    Ctrl(CtrlParam),
    Cfg {
        io_delay: [u8; 4],
        packet_gap: u16,
        counter_sync: u8,
        counter_wait: u16,
        trig_out_delay: u8,
        optical_delay: u8,
    },
    UploadData,
    UploadFft,
    FftParam {
        fft_shift: u16,
        truncation: u32,
    },
    PhaseFactor {
        value: Vec<Vec<Complex<i16>>>,
    },
    XGbeId(XGbeIdParam),
    QueryState,
}

impl AdcMsg {
    pub fn msg_type_code(&self) -> u8 {
        match *self {
            AdcMsg::MasterSync => 0xf0,
            AdcMsg::MasterRst => 0xf1,
            AdcMsg::MasterTrig => 0xf2,
            AdcMsg::Ctrl(..) => 0xf3,
            AdcMsg::Cfg { .. } => 0xf4,
            AdcMsg::UploadData => 0xf5,
            AdcMsg::UploadFft => 0xf6,
            AdcMsg::FftParam { .. } => 0xf8,
            AdcMsg::PhaseFactor { .. } => 0xf9,
            AdcMsg::QueryState { .. } => 0xfa,
            AdcMsg::XGbeId { .. } => 0xfb,
        }
    }

    pub fn get_raw_data(&self) -> Vec<Vec<u8>> {
        match *self {
            AdcMsg::Ctrl(p) => vec![vec![p.param_code()]],
            AdcMsg::Cfg {
                ref io_delay,
                packet_gap,
                counter_sync,
                counter_wait,
                trig_out_delay,
                optical_delay,
            } => {
                let mut result = Vec::new();
                result.extend_from_slice(io_delay);
                result.push((packet_gap & 0x00ff) as u8);
                result.push((packet_gap >> 8) as u8);
                result.push(counter_sync);
                result.push((counter_wait & 0x00ff) as u8);
                result.push((counter_wait >> 8) as u8);
                result.push(trig_out_delay);
                result.push(optical_delay);
                result.append(&mut vec![0; 89]); //make it compatible to GUI App
                vec![result]
            }
            AdcMsg::FftParam {
                fft_shift,
                truncation,
            } => {
                let mut result = vec![
                    (fft_shift & 0x00ff) as u8,
                    (fft_shift >> 8) as u8,
                    ((truncation >> 0) & 0x0000_00ff) as u8,
                    ((truncation >> 8) & 0x0000_00ff) as u8,
                    ((truncation >> 16) & 0x0000_00ff) as u8,
                    ((truncation >> 24) & 0x0000_00ff) as u8,
                ];
                result.append(&mut vec![0; 94]);
                vec![result]
            }
            AdcMsg::PhaseFactor { ref value } => {
                let mut result = Vec::new();
                for i in 0..PORT_PER_BOARD {
                    for n in 0..8 {
                        let mut phase_data = Vec::<i16>::new();
                        for j in n * 256..(n + 1) * 256 {
                            //phase_data.push(value[i][j].im);
                            //phase_data.push(value[i][j].re);
                            phase_data.push(value[i][j].re);
                            phase_data.push(value[i][j].im);
                        }
                        let phase_data = phase_data.into_boxed_slice();
                        let cap = phase_data.len() * 2;
                        let mut phase_data = unsafe {
                            Vec::from_raw_parts(Box::into_raw(phase_data) as *mut u8, cap, cap)
                        };
                        let mut data = vec![i as u8];
                        let addr: [u8; 2] = unsafe { std::mem::transmute((n * 1024) as u16) };
                        data.push(addr[0]);
                        data.push(addr[1]);
                        data.append(&mut phase_data);
                        result.push(data);
                    }
                }
                result
            }
            //&AdcMsg::XGbeId { ref value } => value.clone(),
            AdcMsg::XGbeId(XGbeIdParam::Upper { ref mac1, ref mac2 }) => {
                let mut result = vec![];
                mac1.iter().rev().for_each(|&x| {
                    result.push(x);
                });
                mac2.iter().rev().for_each(|&x| {
                    result.push(x);
                });
                result.append(&mut vec![0; 88]);
                vec![result]
            }
            AdcMsg::XGbeId(XGbeIdParam::Lower {
                ref mac1,
                ref mac2,
                ref mac3,
                ref mac4,
                ref ip1,
                ref ip2,
                ref port1,
                ref port2,
            }) => {
                let mut result = vec![];
                mac1.iter().rev().for_each(|&x| {
                    result.push(x);
                });
                mac2.iter().rev().for_each(|&x| {
                    result.push(x);
                });
                mac3.iter().rev().for_each(|&x| {
                    result.push(x);
                });
                mac4.iter().rev().for_each(|&x| {
                    result.push(x);
                });
                ip1.iter().rev().for_each(|&x| {
                    result.push(x);
                });
                ip2.iter().rev().for_each(|&x| {
                    result.push(x);
                });
                result.push((port1 & 0xff_u16) as u8);
                result.push(((port1 >> 8) & 0xff_u16) as u8);
                result.push((port2 & 0xff_u16) as u8);
                result.push(((port2 >> 8) & 0xff_u16) as u8);
                result.append(&mut vec![0; 64]);
                vec![result]
            }
            AdcMsg::MasterRst => vec![vec![0x01; 10]],
            AdcMsg::MasterTrig => vec![vec![0x01; 10]],
            AdcMsg::MasterSync => vec![vec![0x01; 10]],
            _ => vec![vec![]],
        }
    }
}
