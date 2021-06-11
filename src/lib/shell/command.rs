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
    if let Err(_error) = fs::create_dir(source_path) {
        println!("Can't create the directory");
    } else {
        println!("Dir : {} was created", source_path);
    }
}

pub fn touch(source_path: &str) {
    if let Err(_error) = fs::File::create(source_path) {
        println!("Can't create the file");
    } else {
        println!("File : {} was created", source_path);
    }
}

pub fn mv(source_path: &str, destination_path: &str) {
    if let Err(_error) = fs::rename(source_path, destination_path) {
        println!("Can't move the file (Maybe invalid input?)");
    } else {
        println!("File : {} was moved to : {}", source_path, destination_path);
    }
}

pub fn rm(source_path: &str) {
    if let Err(_error) = fs::remove_dir_all(source_path) {
        if let Err(_error) = fs::remove_file(source_path) {
            println!("Cant remove the file or directory (Maybe invalid input?)");
        }
    } else {
        println!("File : {} was removed", source_path);
    }
}

pub fn cd(destination_path: &str) {
    if let Err(_error) = env::set_current_dir(destination_path) {
        println!("Invalid Path");
    }
}

pub fn cat(source_path: &str) {
    let mut file_content = String::new();
    if let Err(_error) = fs::File::open(source_path).unwrap().read_to_string(&mut file_content) {
        println!("Can't open the file (Maybe invalid path ?)");
    } else {
        println!("{}", file_content);
    }
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