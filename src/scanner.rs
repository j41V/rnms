use std::future::Future;
use std::io::Write;
use std::net;
use std::net::ToSocketAddrs;
use std::thread;
use std::time;
use std::time::Duration;
use futures;
use futures::StreamExt;
use std::fs;

use crate::cli;

fn get_open_ports(tmp_file_name: String) -> String{
    fs::read_to_string(tmp_file_name).expect("Error reading tmp file")
}

fn parse_open_ports(to_parse: String) -> Vec<u16> {
    let mut open_ports: Vec<u16> = Vec::new();
    let ports: Vec<&str> = to_parse.split("\n").collect();
    //dbg!(ports.clone());
    for port in ports {
        if port == "" {
            continue;
        }
        open_ports.push(port.parse().expect(""));
    }
    
    open_ports
}

async fn scan_port(port: u16, target_ip: String, tmp_file_name: String, timeout: Duration) {
    //let address = format!("{}:{}", target_ip, port);
    let mut tmp_file = fs::OpenOptions::new().append(true).open(tmp_file_name).expect("Error opening tmp file");
    let output = format!("{}\n", port.clone());
    let address: Vec<_> = format!("{target_ip}:{port}").to_socket_addrs().expect("").collect();
    match net::TcpStream::connect_timeout(&address[0], timeout) {
        Err(_) => {},
        Ok(_) => {tmp_file.write_all(output.as_bytes()).expect("Error dumping results");}
    }
}

async fn scan_port_simple(port: u16, target_ip: String) {
    let address: Vec<_> = format!("{target_ip}:{port}").to_socket_addrs().expect("").collect();
    let timeout_duration = time::Duration::from_secs_f32(0.5);
    match net::TcpStream::connect_timeout(&address[0], timeout_duration) {
        Err(_) => {},
        Ok(_) => {println!("{}                              ", port)}
    };    
}

pub async  fn scan_host(target_ip: String, ports: Vec<u16>, tmp_file_name: String, timeout: Duration) -> Vec<u16> {
    let port_box = futures::stream::iter(ports);
    //let mut open_ports: Vec<u16> = Vec::new();
    
    port_box.for_each_concurrent(10000, |port | {
        cli::print_scan_status(port.clone());
        scan_port(port, target_ip.clone(), tmp_file_name.clone(), timeout)
        }).await;
    //for port in ports {
    /*
        cli::print_scan_status(port);
        let thread_ip = target_ip.clone();
        let thread_port = port.clone();
        let handle = thread::spawn(move || {
            //let thread_port = port.clone();
            scan_port(thread_port, thread_ip)
        //    cli::print_scan_status(port);
        //    if scan_port(port.clone(), target_ip.clone()) {
        //        open_ports.push(port);
        //    }
        });
        let result = handle.join().unwrap();
        
        if result{
            open_ports.push(port);
        }
    */
    //cli::print_scan_status(port);
    //if scan_port(port, target_ip.clone()).await {
    //    open_ports.push(port);
    //}
    
    //}
    let open_ports = get_open_ports(tmp_file_name);
    //dbg!(open_ports.clone());
    parse_open_ports(open_ports)
}

