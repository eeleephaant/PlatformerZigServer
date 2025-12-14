pub mod udp;

pub trait Server {
    fn run(&self) -> std::io::Result<()>;
}
