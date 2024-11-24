// import modules
mod cli;
mod port_recon;
mod scanner;
mod script_api;
// import libaries
use std::{process, u16};

// constants

pub const DEFAULT_PORTS: &[u16] = &[
    5601, 9300, 80, 23, 443, 21, 22, 25, 3389, 110, 445, 139, 143, 53, 135, 3306, 8080, 1723, 111,
    995, 993, 5900, 1025, 587, 8888, 199, 1720, 465, 548, 113, 81, 6001, 10000, 514, 5060, 179,
    1026, 2000, 8443, 8000, 32768, 554, 26, 1433, 49152, 2001, 515, 8008, 49154, 1027, 5666, 646,
    5000, 5631, 631, 49153, 8081, 2049, 88, 79, 5800, 106, 2121, 1110, 49155, 6000, 513, 990, 5357,
    427, 49156, 543, 544, 5101, 144, 7, 389, 8009, 3128, 444, 9999, 5009, 7070, 5190, 3000, 5432,
    1900, 3986, 13, 1029, 9, 5051, 6646, 49157, 1028, 873, 1755, 2717, 4899, 9100, 119, 37, 1000,
    3001, 5001, 82, 10010, 1030, 9090, 2107, 1024, 2103, 6004, 1801, 5050, 19, 8031, 1041, 255,
    // ...
];

fn main() {
    let target_ip = cli::get_target_ip();
    let options = cli::get_options();
    let (needs_help, scan_all_ports, use_default_script) = options;
    if needs_help {
        println!("{}", cli::USAGE);
        process::exit(1);
    }
    
    let mut ports: Vec<u16> = Vec::new();
    
    if scan_all_ports {
        ports = (0..u16::MAX).collect();
    } else {
        ports = DEFAULT_PORTS.into();
    }
    
    let open_ports = scanner::scan_host(target_ip, ports);
    
    cli::print_results(open_ports);
    
    //let script_result = script_api::run_script(String::from("src/scripts/test.py"));
    //println!("{}", script_result);
    //println!("{}", port_recon::get_port_info(80))
}

