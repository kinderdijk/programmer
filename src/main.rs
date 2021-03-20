use serialport::{SerialPortInfo, SerialPortType};
use std::env;
use std::time::Duration;
use std::path::Path;
use std::ffi::OsStr;
use std::fs::read;
use std::io::Result;
use clap::{Arg, App, crate_version};
use env_logger::Builder;
use log::{LevelFilter, trace, debug, info};


////////////////////////////////////////////////////////////////////
/// 
/// Might need to add some logic to only send 64 bytes at a time
/// as that is the most the arduino can buffer at a time.
/// 
/// Look into using something like Clap for the command 
/// line arguments. Can be configured in yml config file.
/// 
/////////////////////////////////////////////////////////////////////


fn main() {
    let arg_matches = App::new("Custom 8-bit Computer Assembler")
                            .version(crate_version!())
                            .author("Jon Pendlebury")
                            .about("Assembles a custom script to be run on a custom 8-bit computer")
                            .arg(Arg::with_name("OBJECT_FILE")
                                    .help("The name of the object file. Must be 'myobj' extension")
                                    .required(true))
                            .arg(Arg::with_name("v")
                                    .short("v")
                                    .multiple(true)
                                    .help("Sets the verbosity of the output."))
                            .get_matches();

    let binary_filename = arg_matches.value_of("OBJECT_FILE").unwrap();
    let verbosity = match arg_matches.occurrences_of("v") {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        2 | _ => LevelFilter::Trace,
    };
    Builder::new().filter_level(verbosity).init();

    debug!("Binary filename: {:?}", binary_filename);

    let file_contents: Vec<u8> = read_file_contents(binary_filename).expect("Unable to read file contents.");
    trace!("Contents: {:?}", file_contents);
    let file_contents: &[u8] = &file_contents;

    let mut usb_port_name: String = String::new();
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        usb_port_name = match get_port_name(p) {
            Some(name) => name,
            None => String::from("None")
        }
    }
    if usb_port_name == "None" {
        panic!("No usb ports found!");
    }
    info!("Found port. Attempting to connect. port={:?}", usb_port_name);

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