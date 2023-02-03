use std::path::PathBuf;
use std::fs;

type FileResult = Result<String, std::io::Error>;

#[derive(Debug)]
pub struct DirElement {
    pub name: String,
    pub file: Option<PathBuf>,
}

#[derive(Debug)]
pub struct Directory {
    pub name: String,
    pub dir: Option<PathBuf>,
}

pub fn read(path: PathBuf) -> FileResult {
    let name = match std::fs::read_to_string(path) {
        Ok(name) => name,
        Err(e) => return Err(e)
    };
    Ok(name)
}


pub fn get_files(dir: &PathBuf) -> Result<Vec<DirElement>, std::io::Error> {
    let mut file_vec: Vec<DirElement> = Vec::new();
    let mut dir_vec: Vec<Directory> = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                dir_vec.push(Directory {
                    name: entry.file_name().into_string().unwrap_or_else(|_| "".to_string()),
                    dir: Some(path),
               });
            } else if path.is_file() {
               file_vec.push(DirElement {
                    name: entry.file_name().into_string().unwrap_or_else(|_| "".to_string()),
                    file: Some(path),
               });

            }
        }
    }
    return Ok(file_vec)


}
