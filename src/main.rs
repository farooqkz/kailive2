use std::net::{SocketAddr, TcpStream};
use std::time::Duration;
use std::io::Read;
use std::str::FromStr;
use anyhow::Result;



struct RawDebugger {
    stream: TcpStream,
}

impl RawDebugger {
    async fn new(addr: &str, discard_first_one: bool) -> anyhow::Result<Self> {
        let addr = addr.parse::<SocketAddr>()?;
        let mut rdebugger = RawDebugger {
            stream: TcpStream::connect_timeout(&addr, Duration::from_secs(2))?
        };
        if discard_first_one {
            rdebugger.read().await?;
        }
        Ok(rdebugger)
    }

    async fn read(&mut self) -> anyhow::Result<String> {
        let mut buffer = String::new();
        self.stream.read_to_string(&mut buffer);
        if let Some((size, rest)) = buffer.split_once(':') {
            let size: usize = usize::from_str_radix(size, 10)?;
            if rest.len() < size {
                todo!()
            }
        }
    }
}

fn with_len(packet: String) -> String {
    let len = packet.len();
    format!("{len}:{packet}")
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut rdebugger = RawDebugger::new("127.0.0.1:6000", true).await?;
}
