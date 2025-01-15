use std::net::Ipv4Addr;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use std::collections::HashMap;

use clap::Parser;
use pnet::datalink::Channel;
use pnet::ipnetwork::IpNetwork;
use pnet::packet::arp::{ArpHardwareTypes, ArpOperations, ArpPacket, MutableArpPacket};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket, MutableEthernetPacket};
use pnet::util::MacAddr;

mod cli;
use crate::cli::Cli;

const PKT_ETH_SIZE: usize = EthernetPacket::minimum_packet_size();
const PKT_ARP_SIZE: usize = ArpPacket::minimum_packet_size();
const PKT_ARP_OFFSET: usize = PKT_ETH_SIZE;
const PKT_MIN_ARP_RESP_SIZE: usize = PKT_ETH_SIZE + PKT_ARP_SIZE;

fn main() {
    let args = Cli::parse();
    if args.list {
        list_all_interfaces();
        return;
    }
    let timeout = Duration::from_millis(args.timeout);
    let index = args.iface.unwrap();
    let interface = pnet::datalink::interfaces()[index].clone();
    let local_mac = interface.mac.expect("mac is none");
    let local_network = interface
        .ips
        .iter()
        .find_map(|ip| match ip {
            IpNetwork::V4(addr) => Some(addr.clone()),
            IpNetwork::V6(_) => None,
        })
        .expect("ip is none");
    let local_ip = local_network.ip();

    let cfg = pnet::datalink::Config::default();
    let (mut sender, mut receiver) = match pnet::datalink::channel(&interface, cfg) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unknown channel type"),
        Err(e) => panic!("Channel error: {e}"),
    };
    let probe_ip = Ipv4Addr::new(0, 0, 0, 0);
    let (tx, rx) = mpsc::channel();
    // 线程内接收arp回应 通过tx channel将有效(ip, mac)发送出来
    thread::spawn(move || loop {
        let buf = receiver.next().unwrap();

        if buf.len() < PKT_MIN_ARP_RESP_SIZE {
            continue;
        }

        let pkt_arp = ArpPacket::new(&buf[PKT_ARP_OFFSET..]).unwrap();
        if pkt_arp.get_operation() != ArpOperations::Reply {
            continue;
        }
        let sender_ip = pkt_arp.get_sender_proto_addr();
        let sender_mac = pkt_arp.get_sender_hw_addr();
        let target_ip = pkt_arp.get_target_proto_addr();
        let target_mac = pkt_arp.get_target_hw_addr();

        if local_network.contains(sender_ip)
            && (target_ip == local_ip || target_ip == probe_ip)
            && target_mac == local_mac
        {
            tx.send((sender_ip, sender_mac)).unwrap();
        }
    });

    let mut pkt_buf = [0u8; PKT_ETH_SIZE + PKT_ARP_SIZE];
    for ip in &local_network {
        // Use scope blocks so we can reborrow our buffer
        {
            // Build our base ethernet frame
            let mut pkt_eth = MutableEthernetPacket::new(&mut pkt_buf).unwrap();

            pkt_eth.set_destination(MacAddr::broadcast());
            pkt_eth.set_source(local_mac);
            pkt_eth.set_ethertype(EtherTypes::Arp);
        }

        {
            // Build the ARP frame on top of the ethernet frame
            let mut pkt_arp = MutableArpPacket::new(&mut pkt_buf[PKT_ARP_OFFSET..]).unwrap();

            pkt_arp.set_hardware_type(ArpHardwareTypes::Ethernet);
            pkt_arp.set_protocol_type(EtherTypes::Ipv4);
            pkt_arp.set_hw_addr_len(6);
            pkt_arp.set_proto_addr_len(4);
            pkt_arp.set_operation(ArpOperations::Request);
            pkt_arp.set_sender_hw_addr(local_mac);
            if ip == local_ip {
                // ARP Probe
                pkt_arp.set_sender_proto_addr(probe_ip);
            } else {
                pkt_arp.set_sender_proto_addr(local_ip);
            }
            pkt_arp.set_target_hw_addr(MacAddr::zero());
            pkt_arp.set_target_proto_addr(ip);
        }

        sender.send_to(&pkt_buf, None).unwrap().unwrap();
    }
    let mut devices = Vec::new();
    loop {
        let device = match rx.recv_timeout(timeout) {
            Ok(x) => x,
            Err(_) => {
                break;
            }
        };
        devices.push(device);
    }
    devices.push((local_ip, local_mac));
    devices.sort_by_key(|(ip, _)| *ip);
    for dev in &devices {
        if dev.0 == local_ip && dev.1 == local_mac {
            println!("{}\t{} (local)", dev.0, dev.1);
        } else {
            println!("{}\t{}", dev.0, dev.1);
        }
    }
    println!("devices count: {}", devices.len());

    let mut ip_map = HashMap::new();
    let mut mac_map = HashMap::new();
    let mut ip_conflicts = Vec::new();
    let mut mac_conflicts = Vec::new();

    for (ip, mac) in devices {
        let ip_entries = ip_map.entry(ip).or_insert_with(Vec::new);
        ip_entries.push((ip, mac));
        if ip_entries.len() == 2 {
            ip_conflicts.extend(ip_entries.iter().cloned());
        } else if ip_entries.len() > 2 {
            ip_conflicts.push((ip, mac));
        }

        let mac_entries = mac_map.entry(mac).or_insert_with(Vec::new);
        mac_entries.push((ip, mac));
        if mac_entries.len() == 2 {
            mac_conflicts.extend(mac_entries.iter().cloned());
        } else if mac_entries.len() > 2 {
            mac_conflicts.push((ip, mac));
        }
    }

    if ip_conflicts.len() == 0 && mac_conflicts.len() == 0 {
        println!("no conflict");
    }

    if ip_conflicts.len() != 0 {
        println!("ip conflict");
        for dev in ip_conflicts {
            println!("{dev:?}");
        }
    }
    if mac_conflicts.len() != 0 {
        println!("mac conflict");
        for dev in mac_conflicts {
            println!("{dev:?}");
        }
    }
}

fn list_all_interfaces() {
    let mut i = 0;
    for interface in pnet::datalink::interfaces() {
        let network = interface.ips.iter().find_map(|ip| match ip {
            IpNetwork::V4(addr) => Some(addr.clone()),
            IpNetwork::V6(_) => None,
        });
        let network = match network {
            Some(x) => format!("{}", x),
            None => format!("none"),
        };
        if cfg!(target_os = "windows") {
            println!("{i}. {:<18} |  {}", network, interface.description);
        } else {
            println!("{i}. {:<18} |  {}", network, interface.name);
        }

        i += 1;
    }
}
