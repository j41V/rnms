// import modules
mod cli;
mod port_recon;
mod scanner;
mod script_api;
// import libaries
use std::{process, u16};
use std::fs;
use std::time;

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
    let (needs_help, scan_all_ports, use_default_script, timeout) = options;
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
    
    // tmp file
    let duration_since_epoch = time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH).unwrap();
    let tmp_file_name = format!("/tmp/rnms{}", duration_since_epoch.as_nanos());
    fs::File::create(tmp_file_name.clone()).expect("Error creating tmp file");
    
    let open_ports = scanner::scan_host(target_ip.clone(), ports, tmp_file_name, timeout).await;
    
    let mut open_ports_hash_map = cli::create_results_keymap(open_ports.clone());
    //dbg!(open_ports_hash_map.keys());
    


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
        for port in open_ports.clone() {
            cli::print_scan_status(port);
            let script_result = script_api::run_script("/usr/share/rnms/scripts/default_scripts.py".to_string(), target_ip.clone(), port);
            
            let original_value = open_ports_hash_map.get(&port).unwrap();
            let new_value = vec![original_value[0].clone(), script_result.clone()];
            open_ports_hash_map.remove(&port);
            open_ports_hash_map.insert(port, new_value);
            //println!("| {port} | {script_result} |             ");
        }
    }
    
    //dbg!(open_ports_hash_map.clone());
    
    cli::print_results(open_ports_hash_map.clone());
    Ok(())
}

