use bytes::{Buf, BufMut, BytesMut};
use uuid::Uuid;

pub trait Packet {
    fn read(buffer: &mut BytesMut) -> Self where Self: Sized;
    fn write(&self, buffer: &mut BytesMut);
}

pub struct HandshakePacket {
    pub uuid: Uuid,
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: i32,
}

impl Packet for HandshakePacket {
    fn read(buffer: &mut BytesMut) -> Self {
        let mut uuid_bytes = [0u8; 16];
        let protocol_version = buffer.get_i32();
        let server_address = buffer.get_string();
        let server_port = buffer.get_u16();
        let next_state = buffer.get_i32();

        buffer.advance(16);

        HandshakePacket {
            uuid,
            protocol_version,
            server_address,
            server_port,
            next_state,
        }
    }

    fn write(&self, buffer: &mut BytesMut) {
        buffer.put_i32(self.protocol_version);
        buffer.put_string(&self.server_address);
        buffer.put_u16(self.server_port);
        buffer.put_i32(self.next_state);
    }
}