use std::net::{SocketAddr, TcpStream};
use std::time::Duration;
use std::io::{Read, Write};



struct RawDebugger {
    stream: TcpStream,
}

impl RawDebugger {
    async fn new(addr: &str, discard_first_one: bool) -> anyhow::Result<Self> {
        let addr = addr.parse::<SocketAddr>()?;
        let rdebugger = RawDebugger {
            stream: TcpStream::connect_timeout(&addr, Duration::from_secs(2))?
        };
        if discard_first_one {
            rdebugger.read().await?;
        }
        Ok(rdebugger)
    }

    async fn read(&self) -> anyhow::Result<String> {
        let mut packet = String::new();
        let mut size_string = String::new();
        for byte in self.stream.bytes() {
            let byte = byte?;
            if char::from(byte) == ':' {
                break;
            } else {
                size_string.push(char::from(byte));
            }
        }
        let size: usize = usize::from_str_radix(&size_string, 10)?;
        for byte in self.stream.bytes() {
            if packet.len() == size {
                break;
            } else {
                let byte = byte?;
                packet.push(char::from(byte));
            }
        }
        Ok(packet)
    }

    async fn write(&mut self, packet: String) -> anyhow::Result<usize> {
        let packet = format!("{}:{packet}", packet.len());
        Ok(self.stream.write(packet.as_bytes())?)
    }
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut rdebugger = RawDebugger::new("127.0.0.1:6000", true).await?;
}
