extern crate argparse;
extern crate netrat;

use argparse::{ArgumentParser, Store};
use netrat::config::Config;
use std::net::TcpStream;

fn main() {
    let mut config = Config::new();

    {
        let mut parser = ArgumentParser::new();

        parser.set_description("netrat is like netcat but not that good.");

        parser
            .refer(&mut config.host)
            .add_argument("host", Store, "Remote host")
            .required();
        parser
            .refer(&mut config.port)
            .add_argument("port", Store, "Remote port")
            .required();
        parser
            .refer(&mut config.rate)
            .add_option(&["-r", "--rate"], Store, "Data transfer rate");
        parser
            .refer(&mut config.input)
            .add_option(&["-i", "--input"], Store, "Input file");
        parser.parse_args_or_exit();
    }

    let target = format!("{}:{}", config.host, config.port);
    println!("Connecting to {}...", target);

    let stream = TcpStream::connect(target).unwrap();
    let mut data_reader = netrat::data_reader::create(config.input);
    let mut data_sender = netrat::data_sender::create(stream, config.rate);

    loop {
        let data = data_reader.read_data().unwrap();
        data_sender.send_data(data.as_slice()).unwrap();
        break;
    }
}
