use std::net;
use std::thread;

use crate::cli;

fn scan_port(port: &u16, target_ip: &String) -> bool{
    let address = format!("{}:{}", target_ip, port);
    match net::TcpStream::connect(address) {
        Err(e) => false,
        Ok(s) => true
    }
}

pub fn scan_host(target_ip: String, ports: Vec<u16>) -> Vec<u16> { 
    let mut open_ports: Vec<u16> = Vec::new();   
    for port in ports {
        cli::print_scan_status(port);
        if scan_port(&port, &target_ip) {
            open_ports.push(port);
        }
    }
    open_ports
}

