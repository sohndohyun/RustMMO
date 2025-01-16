/*
payload_length: u16,    2byte
packet_type: u16,       2byte
                        4byte
*/

const HEADER_SIZE: usize = 4;

use std::collections::VecDeque;

pub fn check_and_pop_packet(ring: &mut VecDeque<u8>) -> Option<(u16, Vec<u8>)>{
    if ring.len() <= HEADER_SIZE {
        return None;
    }

    let payload_length = u16::from_be_bytes([ring[0], ring[1]]) as usize;
    if ring.len() < HEADER_SIZE + payload_length {
        return None;
    }
    let packet_type = u16::from_be_bytes([ring[2], ring[3]]);

    let (front, back) = ring.as_slices();

    let mut message = Vec::with_capacity(payload_length);

    let front_remaining = front.len() - HEADER_SIZE;
    if front_remaining > 0 {
        let take_from_front = front_remaining.min(payload_length);
        message.extend_from_slice(&front[HEADER_SIZE..HEADER_SIZE + take_from_front]);
    }

    let remaining_payload = payload_length - message.len();
    if remaining_payload > 0 {
        message.extend_from_slice(&back[..remaining_payload]);
    }

    ring.drain(..HEADER_SIZE + payload_length);

    Some((packet_type, message))
}

pub fn push_message_with_header(packet_type: u16, message: Vec<u8>, ring: &mut VecDeque<u8>) -> usize {
    if message.is_empty() {
        0
    } else {
        // message 크기를 첫 두 바이트에 기록
        let payload_length = message.len() as u16; // 최대 65535 (u16 범위)
        ring.push_back((payload_length >> 8) as u8); // 상위 바이트
        ring.push_back((payload_length & 0xFF) as u8); // 하위 바이트
        ring.push_back((packet_type >> 8) as u8); // 상위 바이트
        ring.push_back((packet_type & 0xFF) as u8); // 하위 바이트

        ring.extend(message);

        payload_length as usize + HEADER_SIZE
    }
}

pub fn from_ring_to_vec<'a>(ring: &'a VecDeque<u8>, buffer: &'a mut Vec<u8>) -> &'a [u8] {
    let (front, back) = ring.as_slices();

    if back.is_empty() {
        front
    } else {
        buffer.clear();
        buffer.reserve(front.len() + back.len());
        buffer.extend(front);
        buffer.extend(back);

        buffer.as_slice()
    }
}
