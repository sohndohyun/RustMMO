/*
packet_type: u16,       2byte
payload_length: u16,    2byte
                        4byte
*/

use std::collections::VecDeque;

pub fn push_message_with_header(packet_type: u16, message: &Vec<u8>, ring: &mut VecDeque<u8>) {
    // message 크기를 첫 두 바이트에 기록
    let payload_length = message.len() as u16; // 최대 65535 (u16 범위)
    ring.push_back((packet_type >> 8) as u8); // 상위 바이트
    ring.push_back((packet_type & 0xFF) as u8); // 하위 바이트
    ring.push_back((payload_length >> 8) as u8); // 상위 바이트
    ring.push_back((payload_length & 0xFF) as u8); // 하위 바이트

    ring.extend(message);
}

pub fn from_ring_to_array(ring: &VecDeque<u8>, buffer: &mut Vec<u8>) -> usize {
    buffer.clear();
    let (bufa, bufb) = ring.as_slices();
    buffer.extend(bufa);
    buffer.extend(bufb);

    bufa.len() + bufb.len()
}
