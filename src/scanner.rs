use std::future::Future;
use std::net;
use std::net::ToSocketAddrs;
use std::thread;
use std::time;
use futures;
use futures::StreamExt;

use crate::cli;

async fn scan_port(port: u16, target_ip: String) -> bool{
    //let address = format!("{}:{}", target_ip, port);
    let address: Vec<_> = format!("{target_ip}:{port}").to_socket_addrs().expect("").collect();
    let timeout_duration = time::Duration::from_secs(1);
    match net::TcpStream::connect_timeout(&address[0], timeout_duration) {
        Err(_) => false,
        Ok(_) => true
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

pub async  fn scan_host(target_ip: String, ports: Vec<u16>) -> Vec<u16> {
    let port_box = futures::stream::iter(ports);
    let mut open_ports: Vec<u16> = Vec::new();
    
    let result = port_box.for_each_concurrent(1002, |port | {
        cli::print_scan_status(port.clone());
        scan_port_simple(port, target_ip.clone())
        
        }).await;
    dbg!(result);
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
    open_ports.push(2);
    open_ports
}

