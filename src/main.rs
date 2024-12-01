// import modules
mod cli;
mod port_recon;
mod scanner;
mod script_api;
// import libaries
use std::{process, u16};

use tokio::time::error::Error;

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
#[tokio::main]
async fn main() -> Result<(), Error> {
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
    
    let open_ports = scanner::scan_host(target_ip.clone(), ports).await;
    
    cli::print_results(open_ports.clone());

    //dbg!(script_api::run_script("src/scripts/test.py".to_string()));
    
    //let script_result = script_api::run_script(String::from("src/scripts/test.py"));
    //println!("{}", script_result);
    //println!("{}", port_recon::get_port_info(80))
    //if use_default_scripts {
    //        let script_result = script_api::run_script("src/scripts/http.py".to_string());
    //        
    //        return format!("{port_name} | {script_result}")
    //    };
    

    
    if use_default_script {
        println!("Running default scripts");
        for port in open_ports {
            cli::print_scan_status(port);
            let script_result = script_api::run_script("src/scripts/default_scripts.py".to_string(), target_ip.clone(), port);
            
            println!("| {port} | {script_result} |             ");
        }
    }
    Ok(())
}

