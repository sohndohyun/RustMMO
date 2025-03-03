/*
payload_length: u16,    2byte
packet_type: u16,       2byte
                        4byte
*/

const HEADER_SIZE: usize = 4;

use core::slice;
use std::{collections::VecDeque, io::IoSlice, sync::Arc};

pub fn check_and_pop_packet(ring: &mut VecDeque<u8>) -> Option<(u16, Vec<u8>)>{
    if ring.len() < HEADER_SIZE {
        return None;
    }

    // read header
    let payload_length = u16::from_be_bytes([ring[0], ring[1]]) as usize;
    if ring.len() < HEADER_SIZE + payload_length {
        return None;
    }
    let packet_type = u16::from_be_bytes([ring[2], ring[3]]);
    ring.drain(..HEADER_SIZE);


    // read others;
    let mut message = Vec::with_capacity(payload_length);


    let (front, back) = ring.as_slices();

    let take_from_front = front.len().min(payload_length);
    message.extend_from_slice(&front[..take_from_front]);

    let remaining_payload = payload_length - take_from_front;
    if remaining_payload > 0 {
        message.extend_from_slice(&back[..remaining_payload]);
    }

    ring.drain(..payload_length);

    Some((packet_type, message))
}

pub fn push_message_with_header(packet_type: u16, message: Arc<[u8]>, slice_queue: &mut VecDeque<Arc<[u8]>>) -> usize {
    if message.is_empty() {
        0
    } else {
        let payload_size = message.len();
        let mut header = [0; 4];
        header[..2].copy_from_slice(&(payload_size as u16).to_be_bytes());
        header[2..].copy_from_slice(&packet_type.to_be_bytes());

        slice_queue.push_back(header.into());
        slice_queue.push_back(message);

        HEADER_SIZE + payload_size
    }
}

pub fn slice_queue_to_io_slice<'a>(
    slice_queue: &'a VecDeque<Arc<[u8]>>, 
    offset: usize
) -> Vec<IoSlice<'a>> {
    let mut io_slices = Vec::with_capacity(slice_queue.len());
    let mut first = true;
    for slice in slice_queue {
        let data = slice.as_ref();

        let slice = if first {
            first = false;
            if offset >= data.len() {
                continue;
            }
            &data[offset..]
        } else {
            data
        };

        io_slices.push(IoSlice::new(slice));
    }
    io_slices
}
