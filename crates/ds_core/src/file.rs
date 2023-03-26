
use miniquad::fs::Error as Error;

#[derive(Debug)]
pub struct FileError {
    pub kind: Error,
    pub path: String,
}

impl std::error::Error for FileError {}

impl std::fmt::Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Couldn't load file {}: {}", self.path, self.kind)
    }
}

impl FileError {
    pub fn new(kind: miniquad::fs::Error, path: &str) -> FileError {
        FileError { kind, path: path.to_string() }
    }
}

pub fn load_bytes(path: &str) -> Result<Vec<u8>, Error> {
    use std::fs::File;
    use std::io::Read;

    let mut response = vec![];
    let mut file = File::open(path)?;
    file.read_to_end(&mut response)?;
    Ok(response)
}

pub fn load_bytes_async<F: Fn(Result<Vec<u8>, Error>) + 'static>(path: &str, on_loaded: F) {
    miniquad::fs::load_file(path, on_loaded);
}