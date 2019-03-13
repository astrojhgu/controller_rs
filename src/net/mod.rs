#![allow(clippy::too_many_arguments)]
#![allow(unused_imports)]
use crate::msg::adc_msg::AdcMsg;
use crate::msg::snap2_msg::Snap2Msg;
use etherparse;
use pnet::datalink::{DataLinkSender};
use std::io::Error;
use std::iter::FromIterator;
use std::time::Duration;
use std::thread;
pub const UDP_HDR_LEN: usize = 42;
pub const MIN_PAYLOAD_LEN: usize = 80;

pub fn send_raw_buffer(
    tx: &mut DataLinkSender,
    msg_type: u8,
    buf: &[u8],
    dst_mac: [u8; 6],
    src_mac: [u8; 6],
    mtu_len: usize,
) -> Result<(), Error> {
    let buffer_header_len = 6 + 6 + 2; //6*mac+6*mac+2*len
    if buf.is_empty() {
        let mut sub_buf = vec![0_u8; 1 + buffer_header_len + MIN_PAYLOAD_LEN];
        sub_buf[0..6].copy_from_slice(&dst_mac);
        sub_buf[6..12].copy_from_slice(&src_mac);
        let payload_len: u16 = MIN_PAYLOAD_LEN as u16 + 1;
        sub_buf[12] = ((payload_len >> 8) & 0xff) as u8;
        sub_buf[13] = (payload_len & 0xff) as u8;
        sub_buf[14] = msg_type;
        thread::sleep(Duration::from_millis(10));
        //cap.sendpacket(&sub_buf[..])?
        tx.send_to(&sub_buf[..], None).expect("send error");
    } else {
        for x in buf.chunks(mtu_len - 1) {
            let mut payload = Vec::from_iter(x.iter().cloned());
            while payload.len() < MIN_PAYLOAD_LEN {
                payload.push(0);
            }
            let mut sub_buf = vec![0_u8; payload.len() + 1 + buffer_header_len];
            sub_buf[0..6].copy_from_slice(&dst_mac);
            sub_buf[6..12].copy_from_slice(&src_mac);
            let payload_len = payload.len() + 1;
            sub_buf[12] = ((payload_len >> 8) & 0xff) as u8;
            sub_buf[13] = (payload_len & 0xff) as u8;
            sub_buf[14] = msg_type;
            sub_buf[15..].copy_from_slice(&payload);
            //println!("len={}", sub_buf.len());
            thread::sleep(Duration::from_millis(10));
            //cap.sendpacket(&sub_buf[..])?
            tx.send_to(&sub_buf[..], None).expect("send error");
        }
    }
    Ok(())
}

pub fn send_adc_msg(
    tx: &mut DataLinkSender,
    msg: &AdcMsg,
    dst_mac: [u8; 6],
    src_mac: [u8; 6],
    mut_len: usize,
) -> Result<(), Error> {
    let msg_type = msg.msg_type_code();
    let buffer = msg.get_raw_data();
    send_raw_buffer(tx, msg_type, &buffer, dst_mac, src_mac, mut_len)
}

pub fn send_udp_buffer(
    tx: &mut DataLinkSender,
    buf: &[u8],
    dst_mac: [u8; 6],
    src_mac: [u8; 6],
    dst_ip: [u8; 4],
    src_ip: [u8; 4],
    dst_port: u16,
    src_port: u16,
) -> Result<(), Error> {
    //println!("udp len={}", buf.len());
    //assert!(buf.len() >= 80);
    let builder = etherparse::PacketBuilder::ethernet2(src_mac, dst_mac)
        .ipv4(src_ip, dst_ip, 0xff)
        .udp(src_port, dst_port);

    let mut sub_buf = Vec::new();
    builder
        .write(&mut sub_buf, &buf)
        .expect("udp packet compose err");
    //cap.sendpacket(&sub_buf[..]).expect("sent error");
    tx.send_to(&sub_buf[..], None).expect("error");
    Ok(())
}
