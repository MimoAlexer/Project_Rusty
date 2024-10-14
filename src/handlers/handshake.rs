use std::net::TcpStream;

pub async fn handle_handshake(packet: &mut TcpStream) {
    match packet.next_state {
        1 => {
            //Status_Handler
        }
        2 => {
            //Login Handler
        }
        _ => {
            //Error Handling
        }
    }
}