pub struct Header {
    magic_number: u32,
    version: u16,
    packet_type: u16,
    payload_length: u16,
    packet_id: u16,
}