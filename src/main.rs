use std::{path::PathBuf, env};



mod read_file;

fn main() {
    println!("Hello, world!");
    let p = PathBuf::from("./username.txt");
    
    let dir = env::current_dir().expect("Error couldnt get current dir");
    println!("Current dir is: {:?}", dir);
    

    let name = read_file::read(p);
    println!("The name is {}", name.unwrap());

    let info = read_file::get_files(&dir);

    //println!("info = {:?}", info.as_ref().unwrap());

    let mut file_names: Vec<&String> = Vec::new();
    
    for entry in info.iter() {
        println!("Entry: {:?}", entry);
        for el in entry {
           println!("El: {:?}", el);
           file_names.push(&el.name); 
        }
    }
    
    println!("File Names: {:?}", file_names);


}
