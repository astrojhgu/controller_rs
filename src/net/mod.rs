 #![allow(unused_imports)]
use etherparse;
use msg::adc_msg::AdcMsg;
use pcap::Active;
use pcap::Capture;
use pcap::Error;
use std::iter::FromIterator;
const UDP_HDR_LEN: usize = 42;
const MIN_PAYLOAD_LEN: usize = 80;

pub fn send_raw_buffer(
    cap: &mut Capture<Active>,
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

        cap.sendpacket(&sub_buf[..])?
    } else {
        for x in buf.chunks(mtu_len - 1) {
            let mut payload = Vec::from_iter(x.iter().map(|x| *x));
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
            println!("len={}", sub_buf.len());
            cap.sendpacket(&sub_buf[..])?
        }
    }
    Ok(())
}

pub fn send_adc_msg(
    cap: &mut Capture<Active>,
    msg: &AdcMsg,
    dst_mac: [u8; 6],
    src_mac: [u8; 6],
    mut_len: usize,
) -> Result<(), Error> {
    let msg_type = msg.msg_type_code();
    let buffer = msg.get_raw_data();
    send_raw_buffer(cap, msg_type, &buffer, dst_mac, src_mac, mut_len)
}

pub fn compose_udp_header(
    dst_mac: &[u8; 6],
    src_mac: &[u8; 6],
    dst_ip: &[u8; 4],
    src_ip: &[u8; 4],
    dst_port: u16,
    src_port: u16,
    payload_len: u16,
) -> Vec<u8> {
    let len = payload_len + UDP_HDR_LEN as u16;
    let ip_len = len - 14;
    let udp_len = len - 34;

    let dst_ip_high: u16 = ((dst_ip[0] as u16) << 8) + dst_ip[1] as u16;
    let dst_ip_low: u16 = ((dst_ip[2] as u16) << 8) + dst_ip[3] as u16;
    let src_ip_high: u16 = ((src_ip[0] as u16) << 8) + src_ip[1] as u16;
    let src_ip_low: u16 = ((src_ip[2] as u16) << 8) + src_ip[3] as u16;

    let ip_checksum_fixed_0 = 0x8412 as u32 + src_ip_high as u32 + src_ip_low as u32;
    let ip_checksum_fixed_1 = (ip_checksum_fixed_0 & 0xffff) + (ip_checksum_fixed_0 >> 16);
    let ip_checksum_fixed = (ip_checksum_fixed_1 & 0xffff) + (ip_checksum_fixed_1 >> 16);
    let ip_checksum_0 = ip_checksum_fixed + ip_len as u32 + dst_ip_high as u32 + dst_ip_low as u32;
    let ip_checksum_1 = (ip_checksum_0 & 0xffff) + (ip_checksum_0 >> 16);
    let ip_checksum = !(ip_checksum_1 & 0xffff + (ip_checksum_1 >> 16));
    println!("{:x}", ip_checksum);

    let mut udp_header = vec![0_u8; UDP_HDR_LEN];
    udp_header[0..6].copy_from_slice(&dst_mac[..]);
    udp_header[6..12].copy_from_slice(&src_mac[..]);
    udp_header[12] = 0x08;
    udp_header[13] = 0x00;
    udp_header[14] = 0x45;
    udp_header[15] = 0x00;
    udp_header[16] = (ip_len >> 8) as u8;
    udp_header[17] = (ip_len & 0xff) as u8;
    udp_header[18] = 0x00;
    udp_header[19] = 0x00;
    udp_header[20] = 0x40;
    udp_header[21] = 0x00;
    udp_header[22] = 0xff;
    udp_header[23] = 0x11;
    udp_header[24] = ((ip_checksum & 0xff00) >> 8) as u8;
    udp_header[25] = (ip_checksum & 0xff) as u8;
    udp_header[26..30].copy_from_slice(&src_ip[..]);
    udp_header[30..34].copy_from_slice(&dst_ip[..]);
    udp_header[34] = (src_port >> 8) as u8;
    udp_header[35] = (src_port & 0xff) as u8;
    udp_header[36] = (dst_port >> 8) as u8;
    udp_header[37] = (dst_port & 0xff) as u8;
    udp_header[38] = (udp_len >> 8) as u8;
    udp_header[39] = (udp_len & 0xff) as u8;
    udp_header[40] = 0;
    udp_header[41] = 0;

    udp_header
}

pub fn send_udp_buffer(
    cap: &mut Capture<Active>,
    buf: &[u8],
    dst_mac: [u8; 6],
    src_mac: [u8; 6],
    dst_ip: [u8; 4],
    src_ip: [u8; 4],
    dst_port: u16,
    src_port: u16,
    mtu_len: usize,
) -> Result<(), Error> {
    if buf.is_empty() {
        let header =
            compose_udp_header(&dst_mac, &src_mac, &dst_ip, &src_ip, dst_port, src_port, 0);
        let mut sub_buf = Vec::new();
        sub_buf.extend_from_slice(&header[..]);
        cap.sendpacket(&sub_buf[..])?
    } else {
        for x in buf.chunks(mtu_len) {
            let header = compose_udp_header(
                &dst_mac,
                &src_mac,
                &dst_ip,
                &src_ip,
                dst_port,
                src_port,
                x.len() as u16,
            );
            let mut sub_buf = Vec::new();
            sub_buf.extend_from_slice(&header[..]);
            sub_buf.extend_from_slice(x);
            cap.sendpacket(&sub_buf[..])?
        }
    }

    Ok(())
}
