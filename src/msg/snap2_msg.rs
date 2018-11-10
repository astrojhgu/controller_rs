use crate::utils::to_u8_slice;

#[derive(Copy, Clone, Debug)]
pub enum XGbePortOp {
    TurnOn,
    TurnOff,
}

impl XGbePortOp {
    pub fn get_raw_data(&self) -> Vec<u8> {
        match self {
            XGbePortOp::TurnOn => vec![0x01, 0, 0, 0],
            XGbePortOp::TurnOff => vec![0; 4],
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct XGbePortParam {
    pub src_mac: [u8; 6],
    pub src_ip: [u8; 4],
    pub src_port: u16,
    pub dst_mac: [u8; 6],
    pub dst_ip: [u8; 4],
    pub dst_port: u16,
    pub pkt_len: u32,
}

impl XGbePortParam {
    pub fn get_raw_data(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.append(&mut self.src_mac.iter().rev().map(|&x| x).collect::<Vec<_>>());
        result.append(&mut self.src_ip.iter().rev().map(|&x| x).collect::<Vec<_>>());
        result.push(((self.src_port >> 0) & 0xff_u16) as u8);
        result.push(((self.src_port >> 8) & 0xff_u16) as u8);

        result.append(&mut self.dst_mac.iter().rev().map(|&x| x).collect::<Vec<_>>());
        result.append(&mut self.dst_ip.iter().rev().map(|&x| x).collect::<Vec<_>>());
        result.push(((self.dst_port >> 0) & 0xff_u16) as u8);
        result.push(((self.dst_port >> 8) & 0xff_u16) as u8);

        for i in 0..4 {
            result.push(((self.pkt_len >> 8 * i) & 0xff_u32) as u8);
        }
        result
    }
}

#[derive(Copy, Clone, Debug)]
pub struct AppParam {
    pub mode_sel: u32,
    pub test_mode_streams: [u64; 8],
    pub num_of_streams_sel: u32,
    pub first_ch: u32,
    pub last_ch: u32,
}

#[derive(Copy, Clone, Debug)]
pub enum Snap2Msg {
    XGbePortParams([XGbePortParam; 9]),
    AppParam(AppParam),
    XGbePortOp(XGbePortOp),
}

impl Snap2Msg {
    pub fn msg_type_code(&self) -> u8 {
        match self {
            Snap2Msg::XGbePortParams(..) => 0xf1,
            Snap2Msg::AppParam { .. } => 0xf2,
            Snap2Msg::XGbePortOp(..) => 0xfb,
        }
    }

    pub fn get_raw_data(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.push(self.msg_type_code());
        result.push(0);
        match self {
            &Snap2Msg::XGbePortParams(ref x) => {
                for i in x {
                    result.append(&mut i.get_raw_data());
                }
            }
            &Snap2Msg::AppParam(AppParam {
                mode_sel,
                ref test_mode_streams,
                num_of_streams_sel,
                first_ch,
                last_ch,
            }) => {
                result.extend_from_slice(to_u8_slice(&mode_sel));
                test_mode_streams
                    .iter()
                    .for_each(|x| result.extend_from_slice(to_u8_slice(x)));
                result.extend_from_slice(to_u8_slice(&num_of_streams_sel));
                result.extend_from_slice(to_u8_slice(&first_ch));
                result.extend_from_slice(to_u8_slice(&last_ch));
            }
            &Snap2Msg::XGbePortOp(x) => result.append(&mut x.get_raw_data()),
        }
        result
    }
}
