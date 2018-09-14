use pcap::Capture;
use pcap::Error;
use pcap::Active;
use std::borrow::Borrow;
pub fn send_buffer(cap:&mut Capture<Active>, msg_type:u8, buf: &[u8],  dst_mac:[u8;6], src_mac:[u8;6], mtu_len:usize)->Result<(), Error>{
    let buffer_header_len=6+6+2;//6*mac+6*mac+2*len
    if buf.is_empty(){
        let mut sub_buf=vec![0_u8;1+buffer_header_len];
        sub_buf[0..6].copy_from_slice(&dst_mac);
        sub_buf[6..12].copy_from_slice(&src_mac);
        let payload_len:u16=0;
        sub_buf[12]=(payload_len>>8) as u8;
        sub_buf[13]=(payload_len&0xff) as u8;
        sub_buf[14]=msg_type;
        cap.sendpacket(&sub_buf[..])?
    }
    else{
        buf.chunks(mtu_len-1).for_each(|x|{
            let mut sub_buf=vec![0_u8;x.len()+1+buffer_header_len];
            sub_buf[0..6].copy_from_slice(&dst_mac);
            sub_buf[6..12].copy_from_slice(&src_mac);
            let payload_len=x.len()+1;
            sub_buf[12]=(payload_len>>8) as u8;
            sub_buf[13]=(payload_len&0xff) as u8;
            sub_buf[14]=msg_type;
            sub_buf[15..].copy_from_slice(x);
            println!("{}", sub_buf.len());
            cap.sendpacket(&sub_buf[..]);
        })
    }
    Ok(())
}
