use std::{
    io::{self, Write},
    net::UdpSocket,
};

use crate::server::Server;

pub struct UdpServer {
    socket: UdpSocket,
}
impl UdpServer {
    pub fn new(addr: &str) -> std::io::Result<Self> {
        let socket = UdpSocket::bind(addr)?;
        Ok(Self { socket })
    }
}

impl Server for UdpServer {
    fn run(&self) -> std::io::Result<()> {
        print!("UDP Server runned");
        _ = io::stdout().flush();
        let mut buf = [0; 1024];
        loop {
            let (amt, src) = self.socket.recv_from(&mut buf)?;
            let data = &mut buf[..amt];
            data.reverse();
            self.socket.send_to(data, &src)?;
        }
    }
}
