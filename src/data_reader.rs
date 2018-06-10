use std;
use std::fs::File;
use std::io::Read;


pub trait DataReader {
    fn read_data(&mut self) -> Result<Vec<u8>, std::io::Error>;
}


pub struct FileReader {
    filename: String,
    read: bool
}


impl FileReader {
    pub fn new(filename: String) -> FileReader {
        FileReader {filename, read: false}
    }
}

impl DataReader for FileReader {
    fn read_data(&mut self) -> Result<Vec<u8>, std::io::Error> {
        if self.read == true {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Already Read"));
        }

        let mut out = Vec::<u8>::new();
        let mut file = File::open(self.filename.as_str())?;
        file.read_to_end(&mut out)?;
        self.read = true;
        Ok(out)
    }
}
