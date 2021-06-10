use std::env;
use std::io;
use std::io::{Write};

mod command;
mod pcb;

const CLEAR_CODE: &str = "\x1B[2J\x1B[1;1H";

pub fn run() {
    let mut shell_text = initilize();
    
    loop {
        manage_commands(&mut shell_text);
        print!("{}", shell_text);
        io::stdout().flush().expect("Error flushing stdout");
    }
}

fn initilize() -> String{
    let mut shell_text = String::from("OS@");
    shell_text.push_str(env::current_dir().unwrap().to_str().unwrap());
    shell_text.push_str(">> ");

    print!("{}", CLEAR_CODE);
    print!("{}", shell_text);
    io::stdout().flush().expect("Error flushing stdout");
    shell_text
}

fn manage_commands(shell_text: &mut String) {
    let mut full_command = String::new();
    let mut extracted_command: String;
    io::stdin().read_line(&mut full_command).expect("Error reading command from terminal");
    extracted_command = full_command.clone();

    let mut command_first_argument: &str = "";
    let mut command_second_argument: &str = "";

    if full_command.contains(' ') {
        let vec_str = full_command.split(' ').collect::<Vec<&str>>();
        extracted_command.truncate(0);
        extracted_command.push_str(vec_str.first().unwrap().trim());
        command_first_argument = vec_str.get(1).unwrap().trim();
        if vec_str.len() == 3 {
            command_second_argument = vec_str.last().unwrap().trim();
        }
    }

    match extracted_command.trim() {
        "pwd" => {
            command::pwd();
        },
        "history" => {
            pcb::log();
        },
        "ls" => {
            command::ls();
        },
        "mkdir" => {
            command::mkdir(command_first_argument);
        },
        "touch" => {
            command::touch(command_first_argument);
        },
        "mv" => {
            command::mv(command_first_argument, command_second_argument);
        },
        "rm" => {
            command::rm(command_first_argument);
        },
        "cd" => {
            command::cd(command_first_argument);
            shell_text.truncate(0);
            shell_text.push_str("OS@");
            shell_text.push_str(env::current_dir().unwrap().to_str().unwrap());
            shell_text.push_str(">> ");
        },
        "cat" => {
            command::cat(command_first_argument);
        },
        "vim" => {
            command::vim(command_first_argument);
        },
        "clear" => {
            command::clear(CLEAR_CODE);
        },
        "exit" => {
            command::exit();
        },
        _ => {
            return;
        }
    }
    pcb::record(extracted_command);
}