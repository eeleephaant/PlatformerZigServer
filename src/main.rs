mod server;
use server::{Server, udp::UdpServer};

fn main() -> std::io::Result<()> {
    let udp_server = UdpServer::new("127.0.0.1:34254")?;
    udp_server.run();
    Ok(())
}
