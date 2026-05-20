extern crate rand;
extern crate pnet;
extern crate synner;
extern crate pnet_base;
extern crate pnet_packet;
extern crate pnet_datalink;
extern crate pnet_transport;

use synner::tcp::packet::{send_tcp_packets};

use std::env;
use std::net::{Ipv4Addr};

fn print_help() {
    println!("Usage: ./synner destination_ip interface_name");
}


fn parse_arguments() -> Result<(Ipv4Addr, String, i32), &'static str>{
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("Too few arguments. See usage:");
        panic!("{:?}",print_help());       
    }
    let dst_ip = args[1].parse::<Ipv4Addr>().unwrap();
    let iface = args[2].to_string();
    let icount = args[3].parse::<i32>().unwrap();    
    Ok((dst_ip, iface,icount))
}


fn main() {
    let parsed_args = parse_arguments().unwrap();
    let count = parsed_args.2;
    send_tcp_packets(parsed_args.0, parsed_args.1, count.try_into().unwrap());  
    println!("Sent {} packet(s)", &count);
}
