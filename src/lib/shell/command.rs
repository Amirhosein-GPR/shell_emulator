use std::env;
use std::fs;
use std::process;
use std::io::{Read};
use std::process::Command;

pub fn pwd() {
    let path = String::from(env::current_dir().unwrap().to_str().unwrap());
    println!("{}", path);
}

pub fn ls() {
    let dircetories = fs::read_dir("./").unwrap();
    for dir in dircetories {
        print!("{}    ", dir.unwrap().path().display());
    }
    println!();
}

pub fn mkdir(source_path: &str) {
    fs::create_dir(source_path).expect("Can't create the directory");
    println!("Dir : {} was created", source_path);
}

pub fn touch(source_path: &str) {
    fs::File::create(source_path).expect("Can't create the file");
    println!("File : {} was created", source_path);
}

pub fn mv(source_path: &str, destination_path: &str) {
    fs::rename(source_path, destination_path).expect("Can't move the file");
    println!("File : {} was moved to : {}", source_path, destination_path);
}

pub fn rm(source_path: &str) {
    match fs::remove_dir_all(source_path) {
        Ok(msg) => {
            
        },
        Err(error) => {
            fs::remove_file(source_path).expect("Cant remove the file or directory");
        }
    }
    println!("File : {} was removed", source_path);
}

pub fn cd(destination_path: &str) {
    env::set_current_dir(destination_path).expect("Can't change directory");
}

pub fn cat(source_path: &str) {
    let mut file_content = String::new();
    fs::File::open(source_path).unwrap().read_to_string(&mut file_content).expect("Can't open the file");
    println!("{}", file_content);
}

pub fn vim(source_path: &str) {
    Command::new("vim").arg(source_path).spawn().expect("Can't open the file by vim");
}

pub fn clear(clear_code: &str) {
    print!("{}", clear_code);
}

pub fn exit() {
    process::exit(0);
}