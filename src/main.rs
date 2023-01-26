use std::{path::PathBuf, env};



mod read_file;

fn main() {
    println!("Hello, world!");
    let p = PathBuf::from("./username.txt");
    
    let dir = env::current_dir().expect("Error couldnt get current dir");
    println!("Current dir is: {:?}", dir);
    

    let name = read_file::read(p);
    println!("The name is {}", name.unwrap());
}
