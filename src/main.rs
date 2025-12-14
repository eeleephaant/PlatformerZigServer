mod models;
mod server;

use models::lobby::Lobby;
use models::player::Player;
use server::{Server, udp::UdpServer};

fn main() -> std::io::Result<()> {
    let udp_server = UdpServer::new("127.0.0.1:34254")?;

    _ = udp_server.run();
    let player = Player::new(12, "Hello".to_string());
    Ok(())
}
