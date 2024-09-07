use std::{collections::HashMap, io::Cursor, net::SocketAddrV4, time::Instant};

use bytes::{Buf, BytesMut};
use pnet::packet::{ethernet::EthernetPacket, ipv4::Ipv4Packet, tcp::TcpPacket, Packet};
use prost::{DecodeError, Message, Name};

use dofus_protocol::{connection, game};

#[derive(Debug, Default)]
pub struct Connection {
    data: BytesMut,
}

impl Connection {
    fn recv_data(&mut self, data: &[u8]) {
        self.data.extend_from_slice(data)
    }

    fn try_parse(&mut self) -> Result<game::Message, DecodeError> {
        let mut cursor = Cursor::new(&self.data);
        let message = game::Message::decode_length_delimited(&mut cursor)?;
        self.data.advance(cursor.position() as usize);
        Ok(message)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Source {
    Server,
    Client,
}

fn source(src: SocketAddrV4, dst: SocketAddrV4) -> Option<Source> {
    match (src.port() == 5555, dst.port() == 5555) {
        (true, false) => Some(Source::Server),
        (false, true) => Some(Source::Client),
        _ => None,
    }
}

fn vec_to_hex_string(vec: &Vec<u8>) -> String {
    vec.iter().map(|byte| format!("{:02x}", byte)).collect()
}

fn main() {
    let main_device = pcap::Device::lookup().unwrap().unwrap();
    let mut capture = pcap::Capture::from_device(main_device)
        .unwrap()
        .promisc(true)
        .timeout(1000)
        .open()
        .unwrap();
    capture.filter("tcp", true).unwrap();

    let mut connections: HashMap<(SocketAddrV4, SocketAddrV4), Connection> = HashMap::new();

    while let Ok(packet) = capture.next_packet() {
        let eth_packet = EthernetPacket::new(packet.data).unwrap();
        if eth_packet.get_ethertype() != pnet::packet::ethernet::EtherTypes::Ipv4 {
            continue;
        }

        let ipv4_packet = Ipv4Packet::new(eth_packet.payload()).unwrap();

        if ipv4_packet.get_next_level_protocol() != pnet::packet::ip::IpNextHeaderProtocols::Tcp {
            continue;
        }

        let tcp_packet = TcpPacket::new(ipv4_packet.payload()).unwrap();
        let src_addr = SocketAddrV4::new(ipv4_packet.get_source(), tcp_packet.get_source());
        let dest_addr =
            SocketAddrV4::new(ipv4_packet.get_destination(), tcp_packet.get_destination());

        let src = source(src_addr, dest_addr);
        if src.is_none() {
            continue;
        }

        let payload = tcp_packet.payload();
        if payload.is_empty() {
            continue;
        }

        let connection = connections.entry((src_addr, dest_addr)).or_default();
        connection.recv_data(payload);

        while let Ok(game::Message { content }) = connection.try_parse() {
            process_content(content, src);
        }
    }
}

fn process_content(content: Option<game::message::Content>, src: Option<Source>) {
    todo!()
}
