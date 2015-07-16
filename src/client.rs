use std::net::TcpStream;
use std::io::{Write, BufWriter};
use std::sync::Mutex;

pub struct Client {
    bw: Mutex<BufWriter<TcpStream>>,
}

impl Client {
    pub fn new(stream: TcpStream) -> Client {
        Client {
            bw: Mutex::new(BufWriter::new(stream)),
        }
    }

    pub fn write(&self, s: &str) {
        let mut local_bw = self.bw.lock().unwrap();
        writeln!(local_bw, "{}", s);
        local_bw.flush();
    }
}
