use std::env;
use std::fs;
use std::process;
use std::io::{Read};
use std::process::Command;

pub fn pwd(first_argument: &str) {
    if first_argument == "--help" {
        println!("pwd: Shows the current working directory");
        return;
    }
    let path = String::from(env::current_dir().unwrap().to_str().unwrap());
    println!("{}", path);
}

pub fn ls(first_argument: &str) {
    if first_argument == "--help" {
        println!("ls (path): lists files and directories at gieven path or default path");
        return;
    }
    let dircetories = fs::read_dir("./").unwrap();
    for dir in dircetories {
        print!("{}    ", dir.unwrap().path().display());
    }
    println!();
}

pub fn mkdir(first_argument: &str) {
    if first_argument == "--help" {
        println!("mkdir (path_and_directory_name): makes a directory at the given path");
        return;
    }
    if let Err(_error) = fs::create_dir(first_argument) {
        println!("Can't create the directory");
    } else {
        println!("Dir : {} was created", first_argument);
    }
}

pub fn touch(first_argument: &str) {
    if first_argument == "--help" {
        println!("touch (path_and_file_name): makes a file at the given path");
        return;
    }
    if let Err(_error) = fs::File::create(first_argument) {
        println!("Can't create the file");
    } else {
        println!("File : {} was created", first_argument);
    }
}

pub fn mv(first_argument: &str, second_argument: &str) {
    if first_argument == "--help" {
        println!("mv (from) (to): move a file or directory to another one");
        return;
    }
    if let Err(_error) = fs::rename(first_argument, second_argument) {
        println!("Can't move the file (Maybe invalid input?)");
    } else {
        println!("File : {} was moved to : {}", first_argument, second_argument);
    }
}

pub fn rm(first_argument: &str) {
    if first_argument == "--help" {
        println!("rm (path_and_file_name): removes a file at the given path");
        return;
    }
    if let Err(_error) = fs::remove_dir_all(first_argument) {
        if let Err(_error) = fs::remove_file(first_argument) {
            println!("Cant remove the file or directory (Maybe invalid input?)");
        }
    } else {
        println!("File : {} was removed", first_argument);
    }
}

pub fn cd(first_argument: &str) {
    if first_argument == "--help" {
        println!("cd (path): changes current directory to the given path");
        return;
    }
    if let Err(_error) = env::set_current_dir(first_argument) {
        println!("Invalid Path");
    }
}

pub fn cat(first_argument: &str) {
    if first_argument == "--help" {
        println!("cat (path_and_file_name): reads contents of a file");
        return;
    }
    let mut file_content = String::new();
    if let Ok(mut file) = fs::File::open(first_argument) {
        if let Err(_error) = file.read_to_string(&mut file_content) {
            println!("Error can't read file to string");
        } else {
            println!("{}", file_content);
        }
    } else {
        println!("Can't open the file (Maybe invalid path ?)");
    }
}

pub fn vim(first_argument: &str) {
    if first_argument == "--help" {
        println!("vim (path_and_file_name): opens a file by vim editor");
        return;
    }
    if let Err(_error) = Command::new("vim").arg(first_argument).spawn() {
        println!("Can't open the file by vim (Is it installed ?)");
    }
}

pub fn clear(clear_code: &str, first_argument: &str) {
    if first_argument == "--help" {
        println!("clear: clears the screen");
        return;
    }
    print!("{}", clear_code);
}

pub fn exit(first_argument: &str) {
    if first_argument == "--help" {
        println!("exits the shell program");
        return;
    }
    process::exit(0);
}