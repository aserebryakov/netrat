use std;
use std::fs::File;
use std::io::{self, Read};

pub fn create(filename: String) -> Box<DataReader> {
    if filename.is_empty() {
        Box::new(StandardInputReader::new())
    } else {
        Box::new(FileReader::new(filename))
    }
}

pub trait DataReader {
    fn read_data(&mut self) -> Result<Vec<u8>, std::io::Error>;
}

pub struct FileReader {
    filename: String,
    read: bool,
}

impl FileReader {
    pub fn new(filename: String) -> FileReader {
        FileReader {
            filename,
            read: false,
        }
    }
}

impl DataReader for FileReader {
    fn read_data(&mut self) -> Result<Vec<u8>, std::io::Error> {
        if self.read == true {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Already Read",
            ));
        }

        let mut out = Vec::<u8>::new();
        let mut file = File::open(self.filename.as_str())?;
        file.read_to_end(&mut out)?;
        self.read = true;
        Ok(out)
    }
}

pub struct StandardInputReader {}

impl StandardInputReader {
    pub fn new() -> StandardInputReader {
        StandardInputReader {}
    }
}

impl DataReader for StandardInputReader {
    fn read_data(&mut self) -> Result<Vec<u8>, std::io::Error> {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        Ok(line.into_bytes())
    }
}
