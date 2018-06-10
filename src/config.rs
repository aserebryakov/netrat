use std;
use std::num::ParseFloatError;

pub struct Rate {
    pub value: Option<f64>,
}

impl std::str::FromStr for Rate {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Rate, ParseFloatError> {
        Ok(Rate{value: f64::from_str(&s).ok()})
    }
}

pub struct Config {
    pub host: String,
    pub port: String,
    pub rate: Rate,
    pub input: String,
}

impl Config {
    pub fn new() -> Config {
        Config {
            host: String::from("127.0.0.1"),
            port: String::from("40000"),
            rate: Rate{value: None},
            input: String::new(),
        }
    }
}
