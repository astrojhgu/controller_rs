use crate::msg::adc_msg::AdcMsg;
use crate::msg::adc_msg::CtrlParam;
use crate::msg::adc_msg::XGbeIdParam;
use crate::msg::snap2_msg::AppParam;
use crate::msg::snap2_msg::Snap2Msg;
use crate::msg::snap2_msg::XGbePortOp;
use crate::msg::snap2_msg::XGbePortParam;
use crate::net::send_adc_msg;
use crate::net::send_udp_buffer;
use num_complex::Complex;
//use pcap::{Active, Capture};
use pnet::datalink::{DataLinkReceiver, DataLinkSender};
use serde_yaml::Value;
pub const BOARD_NUM: usize = 16;
pub const ADC_PER_BOARD: usize = 4;

macro_rules! fetch_uint_array {
    ($t: ty, $n: expr, $v:expr) => {{
        let mut result = [0; $n];
        $v.as_sequence()
            .expect("aa")
            .iter()
            .enumerate()
            .for_each(|(i, v)| {
                result[i] = v.as_u64().expect("bb") as $t;
            });
        result
    }};
}

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
    pub xgbeid: Vec<XGbeIdParam>,
    pub io_delay: [[u8; ADC_PER_BOARD]; BOARD_NUM],
    pub master_board_id: usize,
    pub snap_mac: [u8; 6],
    pub snap_ip: [u8; 4],
    pub src_mac: [u8; 6],
    pub src_ip: [u8; 4],
    pub ctrl_src_port: u16,
    pub ctrl_dst_port: u16,
    pub snap_xgbe_params: [XGbePortParam; 9],
    pub ch_beg: usize,
    pub ch_end: usize,
    pub snap_app_param: AppParam,
}

impl BoardCfg {
    pub fn from_yaml(param: &Value) -> BoardCfg {
        let ch_beg = param["ch_beg"].as_u64().expect("ch_beg err") as usize;
        let ch_end = param["ch_end"].as_u64().expect("ch_end err") as usize;

        assert!(ch_beg % 2 == 0 && ch_beg < ch_end, "CH Asseration failed");
        assert!(ch_end % 2 == 0 && ch_end <= 2048, "CH Asseration failed");

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
        let master_board_id = param["master_board_id"]
            .as_u64()
            .expect("cannot read master board id") as usize;
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
        let xgbeid: Vec<XGbeIdParam> = param["xgbeid"]
            .as_sequence()
            .expect("error, xgbeid cannot be read")
            .iter()
            .enumerate()
            .map(|(i, v)| {
                if i % 2 == 0 {
                    let mut mac1 = [0; 6];
                    let mut mac2 = [0; 6];
                    v["mac1"]
                        .as_sequence()
                        .expect("mac1 err")
                        .iter()
                        .enumerate()
                        .for_each(|(i, x)| mac1[i] = x.as_u64().expect("mac err") as u8);
                    v["mac2"]
                        .as_sequence()
                        .expect("mac2 err")
                        .iter()
                        .enumerate()
                        .for_each(|(i, x)| mac2[i] = x.as_u64().expect("mac err") as u8);
                    XGbeIdParam::Upper { mac1, mac2 }
                } else {
                    let mut mac1 = [0; 6];
                    let mut mac2 = [0; 6];
                    let mut mac3 = [0; 6];
                    let mut mac4 = [0; 6];
                    v["mac1"]
                        .as_sequence()
                        .expect("mac1 err")
                        .iter()
                        .enumerate()
                        .for_each(|(i, x)| mac1[i] = x.as_u64().expect("mac err") as u8);
                    v["mac2"]
                        .as_sequence()
                        .expect("mac2 err")
                        .iter()
                        .enumerate()
                        .for_each(|(i, x)| mac2[i] = x.as_u64().expect("mac err") as u8);
                    v["mac3"]
                        .as_sequence()
                        .expect("mac3 err")
                        .iter()
                        .enumerate()
                        .for_each(|(i, x)| mac3[i] = x.as_u64().expect("mac err") as u8);
                    v["mac4"]
                        .as_sequence()
                        .expect("mac4 err")
                        .iter()
                        .enumerate()
                        .for_each(|(i, x)| mac4[i] = x.as_u64().expect("mac err") as u8);
                    let mut ip1 = [0; 4];
                    let mut ip2 = [0; 4];
                    v["ip1"]
                        .as_sequence()
                        .expect("ip1 err")
                        .iter()
                        .enumerate()
                        .for_each(|(i, x)| ip1[i] = x.as_u64().expect("ip err") as u8);
                    v["ip2"]
                        .as_sequence()
                        .expect("ip2 err")
                        .iter()
                        .enumerate()
                        .for_each(|(i, x)| ip2[i] = x.as_u64().expect("ip err") as u8);
                    let port1 = v["port1"].as_u64().expect("port1 err") as u16;
                    let port2 = v["port2"].as_u64().expect("port2 err") as u16;
                    XGbeIdParam::Lower {
                        mac1,
                        mac2,
                        mac3,
                        mac4,
                        ip1,
                        ip2,
                        port1,
                        port2,
                    }
                }
            })
            .collect();
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

        let mut snap_mac = [0_u8; 6];
        param["snap2"]["mac"]
            .as_sequence()
            .expect("missing snap mac")
            .iter()
            .enumerate()
            .for_each(|(i, x)| snap_mac[i] = x.as_u64().expect("snap mac err") as u8);

        let mut snap_ip = [0_u8; 4];
        param["snap2"]["ip"]
            .as_sequence()
            .expect("missing snap ip")
            .iter()
            .enumerate()
            .for_each(|(i, x)| snap_ip[i] = x.as_u64().expect("snap ip err") as u8);

        let ctrl_src_port = param["snap2"]["ctrl_src_port"]
            .as_u64()
            .expect("ctrl src port missing") as u16;

        let ctrl_dst_port = param["snap2"]["ctrl_dst_port"]
            .as_u64()
            .expect("ctrl dst port missing") as u16;

        let mut snap_xgbe_params = [XGbePortParam::default(); 9];
        param["snap2"]["xgbeparam"]
            .as_sequence()
            .expect("xgbeparam missing")
            .iter()
            .enumerate()
            .for_each(|(i, v)| {
                snap_xgbe_params[i] = XGbePortParam {
                    src_mac: fetch_uint_array!(u8, 6, v["src_mac"]),
                    src_ip: fetch_uint_array!(u8, 4, v["src_ip"]),
                    src_port: v["src_port"].as_u64().expect("port missing") as u16,
                    dst_mac: fetch_uint_array!(u8, 6, v["dst_mac"]),
                    dst_ip: fetch_uint_array!(u8, 4, v["dst_ip"]),
                    dst_port: v["dst_port"].as_u64().expect("port missing") as u16,
                    pkt_len: if i == 0 {
                        ((ch_end - ch_beg) * (2 + 2) + 8) as u32
                    } else {
                        2048 * 4 + 8
                    },
                }
            });

        let mode_sel = param["snap2"]["app_param"]["mode_sel"]
            .as_u64()
            .expect("mode sel missing") as u32;
        let test_mode_streams =
            fetch_uint_array!(u64, 8, param["snap2"]["app_param"]["test_mode_streams"]);
        let num_of_streams_sel = param["snap2"]["app_param"]["num_of_streams_sel"]
            .as_u64()
            .expect("num streams sel err") as u32;
        let first_ch = ch_beg as u32;
        let last_ch = ch_end as u32 - 1;

        let mut src_mac = [0_u8; 6];
        param["src_mac"]
            .as_sequence()
            .expect("missing src mac")
            .iter()
            .enumerate()
            .for_each(|(i, x)| src_mac[i] = x.as_u64().expect("snap src err") as u8);
        let mut src_ip = [0_u8; 4];
        param["src_ip"]
            .as_sequence()
            .expect("missing src ip")
            .iter()
            .enumerate()
            .for_each(|(i, x)| src_ip[i] = x.as_u64().expect("src_ip err") as u8);

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
            xgbeid,
            io_delay,
            snap_mac,
            snap_ip,
            src_mac,
            src_ip,
            ch_beg,
            ch_end,
            snap_xgbe_params,
            ctrl_src_port,
            ctrl_dst_port,
            snap_app_param: AppParam {
                mode_sel,
                test_mode_streams,
                num_of_streams_sel,
                first_ch,
                last_ch,
            },
        }
    }

    pub fn set_xgbeid(&self, tx: &mut DataLinkSender) {
        for i in 0..BOARD_NUM {
            println!("Initializing xgbeid of board {}", i);
            let msg = AdcMsg::XGbeId(self.xgbeid[i]);
            send_adc_msg(tx, &msg, self.mac[i], self.src_mac, 1500).expect("sent error");
        }
    }

    pub fn set_fft_param(&self, tx: &mut DataLinkSender) {
        for i in 0..BOARD_NUM {
            println!("setting fft params of board {}", i);
            let msg = AdcMsg::FftParam {
                fft_shift: self.fft_shift,
                truncation: self.truncation,
            };
            send_adc_msg(tx, &msg, self.mac[i], self.src_mac, 1500).expect("sent error");
        }
    }

    pub fn update_phase_factor1(
        &self,
        tx: &mut DataLinkSender,
        bid: usize,
        value: Vec<Vec<Complex<i16>>>,
    ) {
        let msg = AdcMsg::PhaseFactor { value };
        send_adc_msg(tx, &msg, self.mac[bid], self.src_mac, 1500).expect("sent error");
    }

    pub fn update_phase_factor(&self, tx: &mut DataLinkSender, value: Vec<Vec<Vec<Complex<i16>>>>) {
        //self.set_xgbeid(tx);
        //self.set_fft_param(tx);
        assert_eq!(value.len(), BOARD_NUM);
        for (bid, pf) in value.into_iter().enumerate() {
            println!("uploading phase factor to board {}", bid);
            self.update_phase_factor1(tx, bid, pf);
        }
        for bid in 0..BOARD_NUM {
            println!("flipping phase factor of board {}", bid);
            let msg = AdcMsg::Ctrl(CtrlParam::SwitchPhaseFactor);
            send_adc_msg(tx, &msg, self.mac[bid], self.src_mac, 1500).expect("sent error");
        }
        println!("enabling new phase factor");
        let msg = AdcMsg::MasterTrig;
        send_adc_msg(tx, &msg, self.mac[self.master_board_id], self.src_mac, 1500)
            .expect("sent error");
    }

    pub fn send_snap_msg(&self, tx: &mut DataLinkSender, msg: Snap2Msg) {
        //let msg = Snap2Msg::XGbePortParams(self.snap_xgbe_params.clone()).get_raw_data();
        send_udp_buffer(
            tx,
            &msg.get_raw_data(),
            self.snap_mac,
            self.src_mac,
            self.snap_ip,
            self.src_ip,
            self.ctrl_dst_port,
            self.ctrl_src_port,
        )
        .expect("sent error");
    }

    pub fn set_snap_xgbe_params(&self, tx: &mut DataLinkSender) {
        println!("setting snap xgbe params");
        self.send_snap_msg(tx, Snap2Msg::XGbePortParams(self.snap_xgbe_params));
    }

    pub fn set_snap_app_params(&self, tx: &mut DataLinkSender) {
        println!("uploading snap app params");
        self.send_snap_msg(tx, Snap2Msg::AppParam(self.snap_app_param));
    }

    pub fn turn_on_snap_xgbe(&self, tx: &mut DataLinkSender) {
        println!("turnning on snap xgbe");
        self.send_snap_msg(tx, Snap2Msg::XGbePortOp(XGbePortOp::TurnOn));
    }

    pub fn turn_off_snap_xgbe(&self, tx: &mut DataLinkSender) {
        println!("turnning off snap xgbe");
        self.send_snap_msg(tx, Snap2Msg::XGbePortOp(XGbePortOp::TurnOff));
    }

    pub fn reset_all(&self, tx: &mut DataLinkSender) {
        for i in 0..BOARD_NUM {
            println!("reseting board {}", i);
            let msg = AdcMsg::Ctrl(CtrlParam::PreRst);
            send_adc_msg(tx, &msg, self.mac[i], self.src_mac, 1500).expect("sent error");
        }

        println!(
            "reseting master board, i.e., board {}",
            self.master_board_id
        );
        send_adc_msg(
            tx,
            &AdcMsg::MasterRst,
            self.mac[self.master_board_id],
            self.src_mac,
            1500,
        )
        .expect("sent error");
    }

    pub fn set_adc_params(&self, tx: &mut DataLinkSender) {
        for i in 0..BOARD_NUM {
            println!("setting adc param of board {}", i);
            let msg = AdcMsg::Cfg {
                io_delay: self.io_delay[i],
                packet_gap: self.packet_gap,
                counter_sync: self.counter_sync,
                counter_wait: self.counter_wait,
                trig_out_delay: self.trig_out_delay,
                optical_delay: self.optical_delay,
            };
            send_adc_msg(tx, &msg, self.mac[i], self.src_mac, 1500).expect("sent error");
        }
    }

    pub fn sync_adc(&self, tx: &mut DataLinkSender) {
        println!("Syncing...");
        for i in 0..BOARD_NUM {
            println!("reseting iddr of board {}", i);
            let msg = AdcMsg::Ctrl(CtrlParam::IddrRst);
            send_adc_msg(tx, &msg, self.mac[i], self.src_mac, 1500).expect("sent error")
        }
        println!(
            "reseting master board, i.e., board {}",
            self.master_board_id
        );
        send_adc_msg(
            tx,
            &AdcMsg::MasterTrig,
            self.mac[self.master_board_id],
            self.src_mac,
            1500,
        )
        .expect("sent error");
        for i in 0..BOARD_NUM {
            println!("sync board {}", i);
            let msg = AdcMsg::Ctrl(CtrlParam::Synchronize);
            send_adc_msg(tx, &msg, self.mac[i], self.src_mac, 1500).expect("sent error")
        }
        println!("sync master board, i.e., board {}", self.master_board_id);
        send_adc_msg(
            tx,
            &AdcMsg::MasterSync,
            self.mac[self.master_board_id],
            self.src_mac,
            1500,
        )
        .expect("sent error");
    }

    pub fn wait_for_trig(&self, tx: &mut DataLinkSender) {
        for i in 0..BOARD_NUM {
            println!("preparing board {} for trig", i);
            let msg = AdcMsg::Ctrl(CtrlParam::StartFft);
            send_adc_msg(tx, &msg, self.mac[i], self.src_mac, 1500).expect("sent error")
        }
    }

    pub fn send_internal_trig(&self, tx: &mut DataLinkSender) {
        println!(
            "asking master board, i.e., board {} to send internal trig sig",
            self.master_board_id
        );
        send_adc_msg(
            tx,
            &AdcMsg::MasterTrig,
            self.mac[self.master_board_id],
            self.src_mac,
            1500,
        )
        .expect("sent error");
    }

    pub fn store_data(&self, tx: &mut DataLinkSender) {
        for i in 0..BOARD_NUM {
            println!("prepare board {} to store data", i);
            let msg = AdcMsg::Ctrl(CtrlParam::StoreData);
            send_adc_msg(tx, &msg, self.mac[i], self.src_mac, 1500).expect("sent error")
        }
        println!(
            "send trig from master board, i.e., board {}",
            self.master_board_id
        );
        send_adc_msg(
            tx,
            &AdcMsg::MasterTrig,
            self.mac[self.master_board_id],
            self.src_mac,
            1500,
        )
        .expect("sent error");
    }

    pub fn fetch_fft_data1(
        &self,
        bid: usize,
        tx: &mut DataLinkSender,
        rx: &mut DataLinkReceiver,
    ) -> Option<Vec<Vec<Complex<i32>>>> {
        let result = vec![vec![Complex::<i32>::new(1, 1); 2048]; 8];
        println!("fetching data from board {}", bid);
        send_adc_msg(tx, &AdcMsg::UploadFft, self.mac[bid], self.src_mac, 1500)
            .expect("sent error");

        let mut cnt = 0;
        while let Ok(packet) = rx.next() {
            if packet.len() != 1024 + 12 + 2 + 4 + 1 {
                println!("a");
                continue;
            }
            let src_mac = &packet[6..13];
            if src_mac
                .iter()
                .zip(self.mac[bid].iter())
                .any(|(&i, &j)| i != j)
            {
                continue;
            }
            let data_order = (packet[18] - 1) as usize;
            let data = &packet[19..];

            let chip_id = (data_order / 16) as usize; //16 packet for one chip
            let ch_chunk_id = (data_order - 16 * chip_id) as usize;

            //println!("{} {} {} {}", data_order, data.len(), chip_id, ch_chunk_id);
            let chunk_len = 2048 / 16;
            println!(
                "{} {}",
                ch_chunk_id * chunk_len,
                (ch_chunk_id + 1) * chunk_len
            );

            let mut dst_data = unsafe {
                std::slice::from_raw_parts_mut(
                    result[chip_id][ch_chunk_id * chunk_len..(ch_chunk_id + 1) * chunk_len].as_ptr()
                        as *mut u8,
                    1024,
                )
            };

            dst_data.copy_from_slice(data);
            cnt += 1;
        }
        if cnt == 128 {
            Some(result)
        } else {
            None
        }
    }

    pub fn fetch_fft_data(
        &self,
        tx: &mut DataLinkSender,
        rx: &mut DataLinkReceiver,
    ) -> Vec<Vec<Complex<i32>>> {
        let mut result = vec![Vec::<Complex<i32>>::new(); 128];
        for bid in 0..BOARD_NUM {
            if let Some(x) = self.fetch_fft_data1(bid, tx, rx) {
                for (d, s) in result[bid * 8..(bid + 1) * 8].iter_mut().zip(x.into_iter()) {
                    *d = s;
                }
            } else {
                println!("Error: no data fetched from board {}", bid);
            }
        }
        result
    }
}
