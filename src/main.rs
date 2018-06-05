extern crate argparse;
extern crate netrat;

use std::io::{self, Write, Read};
use std::net::TcpStream;
use std::{thread, time};
use argparse::{ArgumentParser, Store};
use std::fs::File;
use netrat::Config;


fn main() {
    let mut config = Config::new();

    {
        let mut parser = ArgumentParser::new();

        parser.set_description("netrat is like netcat but not that good.");

        parser.refer(&mut config.host).add_argument("host", Store, "Remote host").required();
        parser.refer(&mut config.port).add_argument("port", Store, "Remote port").required();
        parser.refer(&mut config.rate).add_option(&["-r", "--rate"], Store, "Data transfer rate");
        parser.refer(&mut config.input).add_option(&["-i", "--input"], Store, "Input file");
        parser.parse_args_or_exit();
    }

    let target = format!("{}:{}", config.host, config.port);
    println!("Connecting to {}...", target);

    let mut stream = TcpStream::connect(target).unwrap();

    let interval = match config.rate.value {
        Some(r) => time::Duration::from_millis((1024f64 / r) as u64),
        None => time::Duration::from_millis(0),
    };

    loop {
        let data = read_data(&config.input).unwrap();
        send_data(&mut stream, data.as_slice(), interval);
        break;
    }
}


fn read_data(input: &String) -> Result<Vec<u8>, std::io::Error> {
    if input.is_empty() {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        Ok(line.into_bytes())
    }
    else {
        let mut out = Vec::<u8>::new();
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

