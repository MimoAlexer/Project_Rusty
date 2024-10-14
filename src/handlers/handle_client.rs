use std::net::TcpStream;
use bytes::BytesMut;
use crate::handlers::handshake::handle_handshake;

async fn handle_client(mut stream: TcpStream) {
    let mut buffer = BytesMut::with_capacity(1024);

    match read_packet(&mut stream, &mut buffer).await {
        Ok(packet) => {
            handle_handshake(&mut stream, )
        }
    }
}