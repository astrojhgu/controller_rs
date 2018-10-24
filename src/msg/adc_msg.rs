use num_complex::Complex;

const PORT_PER_BOARD: usize = 8;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CtrlParam {
    Sync,
    PreRst,
    StartFft,
    SwitchPhaseFactor,
    StoreData,
}

impl CtrlParam {
    pub fn param_code(&self) -> u8 {
        match self {
            &CtrlParam::Sync => 0,
            &CtrlParam::PreRst => 1,
            &CtrlParam::StartFft => 2,
            &CtrlParam::SwitchPhaseFactor => 3,
            &CtrlParam::StoreData => 4,
        }
    }
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
        value: [Vec<Complex<i16>>; PORT_PER_BOARD],
    },
    QueryState,
    XGbeId {
        value: Vec<u8>,
    },
}

impl AdcMsg {
    pub fn msg_type_code(&self) -> u8 {
        match self {
            &AdcMsg::MasterSync => 0xf0,
            &AdcMsg::MasterRst => 0xf1,
            &AdcMsg::MasterTrig => 0xf2,
            &AdcMsg::Ctrl(..) => 0xf3,
            &AdcMsg::Cfg { .. } => 0xf4,
            &AdcMsg::UploadData => 0xf5,
            &AdcMsg::UploadFft => 0xf6,
            &AdcMsg::FftParam { .. } => 0xf8,
            &AdcMsg::PhaseFactor { .. } => 0xf9,
            &AdcMsg::QueryState { .. } => 0xfa,
            &AdcMsg::XGbeId { .. } => 0xfb,
        }
    }

    pub fn get_raw_data(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.append(&mut match self {
            &AdcMsg::Ctrl(p) => vec![p.param_code()],
            &AdcMsg::Cfg {
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
                result
            }
            &AdcMsg::FftParam {
                fft_shift,
                truncation,
            } => vec![
                (fft_shift & 0x00ff) as u8,
                (fft_shift >> 8) as u8,
                ((truncation >> 0) & 0x000000ff) as u8,
                ((truncation >> 8) & 0x000000ff) as u8,
                ((truncation >> 16) & 0x000000ff) as u8,
                ((truncation >> 24) & 0x000000ff) as u8,
            ],
            &AdcMsg::PhaseFactor { ref value } => {
                let data = value
                    .iter()
                    .fold(Vec::<i16>::new(), |mut a, b| {
                        for p in b {
                            a.push(p.im);
                            a.push(p.re);
                        }
                        a
                    }).into_boxed_slice();

                let cap = data.len() * 2;
                unsafe { Vec::from_raw_parts(Box::into_raw(data) as *mut u8, cap, cap) }
            }
            &AdcMsg::XGbeId { ref value } => value.clone(),
            _ => vec![],
        });
        result
    }
}
