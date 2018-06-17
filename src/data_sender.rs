use config::Rate;
use std;
use std::io::{self, Write};
use std::net::TcpStream;
use std::{thread, time};

pub fn create(stream: TcpStream, rate: Rate) -> Box<DataSender> {
    match rate.value {
        Some(r) => Box::new(RateSender::new(stream, r)),
        None => panic!("Sending date without rate is not implemented"),
    }
}

pub trait DataSender {
    fn send_data(&mut self, data: &[u8]) -> Result<(), std::io::Error>;
}

pub struct RateSender {
    stream: TcpStream,
    rate: f64,
}

impl RateSender {
    fn new(stream: TcpStream, rate: f64) -> RateSender {
        RateSender { stream, rate }
    }
}

impl DataSender for RateSender {
    fn send_data(&mut self, data: &[u8]) -> Result<(), std::io::Error> {
        println!("Sending data...");

        let interval = time::Duration::from_millis((1024f64 / self.rate) as u64);

        for b in data {
            self.stream.write(&[*b])?;
            thread::sleep(interval);
            io::stdout().write(&[*b])?;
            io::stdout().flush()?;
        }

        Ok(())
    }
}
