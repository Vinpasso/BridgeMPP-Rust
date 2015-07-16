use std::net::TcpStream;
use std::io::{Write, BufWriter, Result};
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

    pub fn write(&self, s: &str) -> Result<()> {
        let mut local_bw = self.bw.lock().unwrap();
        try!(writeln!(local_bw, "{}", s));
        try!(local_bw.flush());
        Ok(())
    }
}
