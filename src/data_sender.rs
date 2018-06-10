use std;
use config::Rate;


pub trait DataSender {
    fn send_data(&self) -> Result<(), std::io::Error>;
}


pub struct RateSender {
}
