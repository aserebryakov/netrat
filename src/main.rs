extern crate argparse;

use std::io::{self, Write, Read, ErrorKind};
use std::net::TcpStream;
use std::{thread, time};
use argparse::{ArgumentParser, Store};
use std::fs::File;
use std::num::ParseFloatError;


fn main() {
    let mut host = String::from("127.0.0.1");
    let mut port = String::from("40000");
    let mut rate = Rate{value: None};
    let mut input = String::new();

    {
        let mut parser = ArgumentParser::new();

        parser.set_description("netrat is like netcat but not that good.");

        parser.refer(&mut host).add_argument("host", Store, "Remote host").required();
        parser.refer(&mut port).add_argument("port", Store, "Remote port").required();
        parser.refer(&mut rate).add_option(&["-r", "--rate"], Store, "Data transfer rate");
        parser.refer(&mut input).add_option(&["-i", "--input"], Store, "Input file");
        parser.parse_args_or_exit();
    }

    let target = format!("{}:{}", host, port);
    println!("Connecting to {}...", target);

    let mut stream = TcpStream::connect(target).unwrap();

    loop {
        let data = read_data(&input).unwrap();
        let interval = match rate.value {
            Some(r) => time::Duration::from_millis((1024f64 / r) as u64),
            None => time::Duration::from_millis(0),
        };

        send_data(&mut stream, data.as_slice(), interval);
    }
}


fn read_data(input: &String) -> Result<Vec<u8>, std::io::Error> {
    let mut out = Vec::<u8>::new();

    if input.is_empty() {
        Err(std::io::Error::new(ErrorKind::Other, "Not Implemented"))
    }
    else {
        let mut file = File::open(input.as_str())?;
        file.read_to_end(&mut out)?;
        Ok(out)
    }
}


fn send_data(stream: &mut TcpStream, data: &[u8], interval: time::Duration) {
    println!("Sending data...");

    for b in data {
        stream.write(&[*b]).unwrap();
        thread::sleep(interval);
        io::stdout().write(&[*b]).unwrap();
        io::stdout().flush().unwrap();
    }
}


struct Rate {
    pub value: Option<f64>,
}

impl std::str::FromStr for Rate {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Rate, ParseFloatError> {
        Ok(Rate{value: f64::from_str(&s).ok()})
    }
}
