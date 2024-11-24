use std::env;
use std::io::{stdout, Write};

use crate::port_recon;
/*

rnms [OPTIONS] target_ip

OPTIONS:
-h/--help            | print usage screen
-ap/ --allports      | scan all ports
-ds/--defaultscripts | run default scripts after scan
*/

pub const USAGE: &str = "rnms [OPTIONS] target_ip";

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
    let target_ip = arguments.last().expect("Not enough arguments").to_owned();
    return target_ip
}

pub fn print_scan_status(port: u16) {
    print!("scanning port {}         \r", port);
    stdout().flush();
}

pub  fn print_results(open_ports: Vec<u16>) {
    println!("found {} open ports              ", open_ports.len());
    for open_port in open_ports {
        let open_port_string = format!("{}", open_port);
        println!("| {} | {} |",open_port, port_recon::get_port_info(open_port));
    }
}