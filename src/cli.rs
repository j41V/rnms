use std::collections::HashMap;
use std::{env, process};
use std::io::{stdout, Write};

use crate::port_recon;
/*

rnms [OPTIONS] target_ip

OPTIONS:
-h/--help            | print usage screen
-ap/ --allports      | scan all ports
-ds/--defaultscripts | run default scripts after scan
*/

pub const USAGE: &str = "rnms [OPTIONS] target_ip\n\nOPTIONS:\n-h/--help            | print usage screen\n-ap/ --allports      | scan all ports\n-ds/--defaultscripts | run default scripts after scan\n";

pub fn print_usage() {
    println!("{}", USAGE);
}

pub fn get_cli_arguments() -> Vec<String> {
    let argsuments: Vec<String> = env::args().collect();
    return argsuments
    
}

pub fn get_options() -> (bool, bool, bool) {
    let arguments = get_cli_arguments();
    // strings of options
    let help_short = "-h".to_string();
    let help = "--help".to_string();
    
    let scan_all_ports_short = "-ap".to_string();
    let scan_all_ports = "--allports".to_string();
    
    let default_script_short = "-ds".to_string();
    let default_script = "--defaultscript".to_string();
    
    let mut is_help = false;
    let mut is_scan_all_ports = false;
    let mut use_default_script = false;
    
    for argument in arguments{
        if argument == help_short || argument == help {
            is_help = true; 
        }
        if argument == scan_all_ports || argument == scan_all_ports_short {
            is_scan_all_ports = true;
        }
        if argument == default_script || argument == default_script_short {
            use_default_script = true;
        }
    };
    
    return (is_help, is_scan_all_ports, use_default_script)
}

pub fn get_target_ip() -> String{
    let arguments = get_cli_arguments();
    if arguments.len() == 1 {
        print_usage();
        process::exit(1);
    }
    let target_ip = arguments.last().expect("Not enough arguments").to_owned();
    return target_ip
}

pub fn print_scan_status(port: u16) {
    print!("scanning port {}         \r", port);
    stdout().flush();
}

pub  fn print_results(open_ports_hash_map: HashMap<u16, Vec<String>>) {
    /*
    println!("found {} open ports              ", open_ports.len());
    for open_port in open_ports {
        let open_port_string = format!("{}", open_port);
        println!("| {} | {} |",open_port, port_recon::get_port_description(open_port));
    }
    */
    
    let empty_string = "".to_string();
    
    let ports = open_ports_hash_map.keys();
    for port in ports.clone() {
        let port_vec = open_ports_hash_map.get(port).unwrap();
        let port_name = port_vec[0].clone();
        let script_result = port_vec[1].clone();
        
        let port_string: String = port.to_string();
        let spaces_port = " ".repeat(5 - port_string.len());
        let spaces_port_name = " ".repeat(15 - port_name.len());
        let spaces_script_result = " ".repeat(45 - script_result.len());
        if ports.clone().len() == 0 {
            println!("Found no open ports");
            process::exit(0);
        }
        
        if port_name == empty_string && script_result == empty_string {
            println!("| {}{spaces_port}|", port);  
        } else if script_result == empty_string {
            println!("| {}{spaces_port}| {}{spaces_port_name}|", port, port_name);
        } else {
            println!("| {port}{spaces_port}| {port_name}{spaces_port_name}| {script_result}{spaces_script_result}|");
        }
        
    }
}

pub fn create_results_keymap(open_ports: Vec<u16>)-> HashMap<u16, Vec<String>> {
    let mut results = std::collections::HashMap::new();
    for port in open_ports {
        let port_info = vec![port_recon::get_port_description(port.clone()), String::new()];
        results.insert(port, port_info);
    }
    results
}