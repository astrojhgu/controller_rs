pub enum AdcMsg{
    Trig(u8),
    Cfg{
        io_delay:[u8;4],
        packet_gap:u16,
        counter_sync:u8,
        counter_wait:u16,
        trig_out_delay:u8,
    },
    StartDaq,
    StartFft,
    FftParam{
        fft_shift:u16,
        truncation:u32,
    },
    PhaseFactor{value:Vec<u16>},
    QueryState,
    XGbeId{value:Vec<u8>}
}


impl AdcMsg{
    pub fn msg_type_code(&self)->u8{
        match self{
            &AdcMsg::Trig(..)=>0xf3,
            &AdcMsg::Cfg {..}=>0xf4,
            &AdcMsg::StartDaq=>0xf5,
            &AdcMsg::StartFft=>0xf6,
            &AdcMsg::FftParam {..}=>0xf8,
            &AdcMsg::PhaseFactor {..}=>0xf9,
            &AdcMsg::QueryState{..}=>0xfa,
            &AdcMsg::XGbeId {..}=>0xfb,
        }
    }
}