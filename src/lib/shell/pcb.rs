use std::fs::File;
use std::io::{Read, Write};
use std::fs::OpenOptions;

pub fn record(command: String, pcb_file_location: &String) {
    let mut pcb_records = String::new();

    let mut file = File::open(pcb_file_location);
    if let Err(_error) = file {
        create_new_pcb_file(pcb_file_location);
        file = File::open(pcb_file_location);
    }
    let mut file = file.unwrap();

    file.read_to_string(&mut pcb_records).expect("Error reading from pcb.conf file");

    let pcb_records = pcb_records.split('-').collect::<Vec<&str>>();

    let last_pcb_record: Vec<&str>;
    last_pcb_record = pcb_records.last().unwrap().split(' ').collect::<Vec<&str>>();

    let mut last_id: u16 = 0;
    if let Ok(id) = last_pcb_record[0].parse::<u16>() {
        last_id = id;
    }
    
    create_new_pcb_record(last_id + 1, command, pcb_file_location);
}

pub fn log(pcb_file_location: &String) {
    let mut pcb_records = String::new();

    let mut file = File::open(pcb_file_location);
    if let Err(_error) = file {
        create_new_pcb_file(pcb_file_location);
        file = File::open(pcb_file_location);
    }
    let mut file = file.unwrap();

    file.read_to_string(&mut pcb_records).expect("Error reading from pcb.conf file");

    let pcb_records = pcb_records.split('-').collect::<Vec<&str>>();

    println!("-process_id process_name process_creation_date(UTC)");

    for index in 2..pcb_records.len() {
        println!("{}", pcb_records.get(index).unwrap().trim());
    }
}

fn create_new_pcb_file(pcb_file_location: &String) {
    let pcb_record_template = "#-process_id process_name process_creation_date(UTC)\n";

    File::create(pcb_file_location).expect("Error creating the pcb.conf file")
        .write(&pcb_record_template.as_bytes())
        .expect("Error writing to the file after opening it");
}

fn create_new_pcb_record(id: u16, command: String, pcb_file_location: &String) {
    let mut pcb_record = String::from("-");

    pcb_record.push_str(id.to_string().as_str());
    pcb_record.push(' ');
    pcb_record.push_str(command.as_str().trim());
    pcb_record.push(' ');
    pcb_record.push_str(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs().to_string().as_str());
    pcb_record.push('\n');

    OpenOptions::new().append(true).open(pcb_file_location)
        .expect("Error creating the pcb.conf file")
        .write(&pcb_record.as_bytes())
        .expect("Error writing to the file after opening it");
}