use std::net::{SocketAddr, TcpStream};
use std::time::Duration;
use std::io::Read;
use std::str::FromStr;
use anyhow::Result;



struct RawDebugger {
    stream: TcpStream,
}

impl RawDebugger {
    async fn new(addr: &str, discard_first_one: bool) -> Result<Self> {
        let addr = addr.parse::SocketAddr()?;
        let rdebugger = RawDebugger {
            stream: TcpStream::connect_timeout(&addr, Duraion::from_secs(2))?
        };
        if discard_first_one {
            rdebugger.read();
        }
        Ok(rdebugger)
    }

    async fn read(&mut self) -> Result<String> {
        let mut buffer = String::new();
        self.stream.read_to_string(&mut buffer);
        if let Some((size, rest)) = buffer.split_once(':') {
            let size: usize = usize::from_str_radix(size, 10)?;
            if rest.len() < size {
            }
        }
    }
}

fn with_len(packet: String) -> String {
    let len = packet.len();
    format!("{len}:{packet}")
}


#[tokio::main]
fn main() -> Result<()> {
    let addr = "127.0.0.1:6000".parse::<SocketAddr>()?;
    let mut socket = TcpStream::connect_timeout(&addr, Duration::from_secs(2)).expect("Cannot connect");
    let mut buffer: [u8; 512] = [0; 512];
    let mut packet: String = "".to_string();
    let mut total_read = 0;
    let size: usize = loop {
        match socket.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    if total_read == 0 {
                        panic!("Cannot read anything");
                    } else {
                        if packet.last
                    }
                } else {
                    total_read += n;
                    packet.push_str(&String::from_utf8(buffer.to_vec()).expect("oops"));
                }
            }
            Err(_) => {
                panic!("dang!");
            }
        }
    };
    if let Some(the_remainder) = packet.split_once(":") {
        packet = the_remainder.to_string();
    } else {
        packet.clear();
    }
    while packet.len() < size {
        let read_bytes = socket.read(&mut buffer).expect("cannot read");
        packet.push_str(&String::from_utf8_unchecked(buffer[0..read_bytes].to_vec()));
    }
    buffer.iter_mut().for_each(|c| { *c = 0; });
    /*
    let mut size;
    let written_size = socket.write(with_len("{\"to\":\"root\",\"type\":\"listTabs\"}".to_string()).as_bytes()).expect("Cannot write");
    println!("just wrote {written_size} byte(s)");
    size = loop {
        socket.read_to_string(&mut buffer).expect("cannot read");
        println!("{buffer}");
        if buffer.ends_with(':') {
            buffer.pop();
            break usize::from_str_radix(&buffer, 10).expect("incorrect base 10 number");
        }
    };
    println!("just read size");
    buffer.iter_mut().for_each(|c| { *c = 0; });
    while buffer.len() < size {
        socket.read_to_string(&mut buffer).expect("cannot read");
    }*/
    println!("{packet}");
}
