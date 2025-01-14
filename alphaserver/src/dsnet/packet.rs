/* 
packet_type: u16,       2byte
payload_length: u16,    2byte
                        4byte
*/                      

const HEADER_SIZE: usize =  4;

pub fn message_to_packet(packet_type: u16, message: &Vec<u8>, buffer: &mut [u8; 1024]) -> Result<usize, &'static str> {
    
    if message.len() > 1022 {
        return Err("Message is too large to fit in the buffer");
    }
    
    // message 크기를 첫 두 바이트에 기록
    let payload_length = message.len() as u16; // 최대 65535 (u16 범위)
    buffer[0] = (packet_type >> 8) as u8; // 상위 바이트
    buffer[1] = (packet_type & 0xFF) as u8; // 하위 바이트
    buffer[2] = (payload_length >> 8) as u8;
    buffer[3] = (payload_length & 0xFF) as u8;

    // message를 buffer에 복사
    buffer[HEADER_SIZE..(HEADER_SIZE + payload_length as usize)].copy_from_slice(message);

    Ok(payload_length as usize + HEADER_SIZE)
}