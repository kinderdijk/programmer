use serialport::{SerialPortInfo, SerialPortType};
use std::env;
use std::time::Duration;
use std::path::Path;
use std::ffi::OsStr;
use std::fs::read;
use std::io::Result;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    println!("{:?}", filename);

    let file_contents: Vec<u8> = read_file_contents(filename).expect("Unable to read file contents.");
    println!("Contents: {:?}", file_contents);
    let file_contents: &[u8] = &file_contents;

    // let ports = serialport::available_ports().expect("No ports were found.");

    let mut usb_port_name: String = String::new();
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        usb_port_name = match get_port_name(p) {
            Some(name) => name,
            None => panic!("No usb ports found!  Aborting!")
        }
    }
    
    // for p in ports {
    //     println!("Port: {:?}", p.port_name);
    // }

    let mut active_port = serialport::new(usb_port_name, 115_200)
            .timeout(Duration::from_millis(10))
            .open().expect("Failed to open port.");

    active_port.write(file_contents).expect("Problem writing to port.");
}

fn get_port_name(port_info: SerialPortInfo) -> Option<String> {
    match port_info.port_type {
        SerialPortType::UsbPort(_type_info) => Some(port_info.port_name),
        _ => None
    }
}

fn valid_file(filename: &str) -> bool {
    let mut valid: bool = true;
    let extension = Path::new(filename).extension()
            .and_then(OsStr::to_str)
            .expect("No extension was found.");

    if extension != "myobj" {
        valid = false;
        return valid;
    }

    return valid;
}

fn read_file_contents(filename: &str) -> Result<Vec<u8>> {
    if !valid_file(filename) {
        panic!("File is not valid. filename={}", filename);
    }

    let file_contents = read(filename).expect("Problem reading the file.");
    Ok(file_contents)
}