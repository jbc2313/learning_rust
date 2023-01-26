
use std::path::PathBuf;



type FileResult = Result<String, std::io::Error>;


pub fn read(path: PathBuf) -> FileResult {
    let name = match std::fs::read_to_string(path) {
        Ok(name) => name,
        Err(e) => return Err(e)
    };
    Ok(name)
}
