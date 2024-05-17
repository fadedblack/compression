use std::{fs::File, io::Read, vec};

struct RockyFile {
    file_path : String,
    contents : String,
}

impl RockyFile {
    pub fn new(file_path : String) -> Self {
        let contents = String::new();
        Self {
            file_path,
            contents,
        }
    }


    fn open_file(&mut self) -> Result<usize, std::io::Error> {
        let mut file =  File::open(&self.file_path)?;
        let file_size = file.read_to_string(&mut self.contents)?;
    
        Ok(file_size)
    }


    pub fn get_RKY_file() {
        RockyFile::rocky_file();
    }


    fn rocky_file(&self) {
        let mut header = vec![];
        let mut index = vec![];
        let mut buffer = String::new();
        for chars in &self.contents.chars() {
            if chars != ' ' {
                buffer.push(chars);
            }
            let Some(value) = if header.get(buffer) {
                index
            }
        }
    }
}


struct RockyFileError;

impl RockyFileError {


}
