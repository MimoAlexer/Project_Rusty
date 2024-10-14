use std::io::Read;
use std::net::{SocketAddr, TcpListener, TcpStream};
use bytes::{Buf, BytesMut};
use uuid::Uuid;
use crate::packet::{HandshakePacket, Packet};

mod handlers;
mod packet;

#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:25565").await.unwrap();

    let uuid = Uuid::;
    stream.write_all(uuid.as_bytes()).await.unwrap();
    //Start Listener
    let listener = TcpListener::bind("0.0.0.0:25565".await).unwrap();
    println!("Server is running on 25565");

    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        println!("Accepted connection from {}", addr);

        tokio::spawn(async move {
            handle_client(socket).await;
        });
    }
}

async fn handle_client(mut socket: TcpStream) {
    let mut buf = BytesMut::with_capacity(1024);

    loop {
        let n = match socket.read(buf.as_mut()).await {
            Ok(0) => {
                println!("Connection closed");
                break;
            }
            Ok(n) => n,
            Err(e) => {
                eprintln!("Error reading from socket: {}", e);
                break;
            }
        };

        buf.advance(n);
        println!("Received data");
        process_packet(&mut socket, &mut buf).await;
    }
}

async fn process_packet(socket: &mut TcpStream, buffer: &mut BytesMut) {
    let handshake = HandshakePacket::read(buffer);

    println!("Client UUID: {}", handshake.uuid);

    match handshake.next_state {
        1 => {
            //Status Request
        }
        2 => {
            //login process
        }
        _ => {
            eprintln!("Unknown packet type");
        }
    }
}
