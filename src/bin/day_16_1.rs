use std::fs;

use itertools::Itertools;

#[derive(Debug)]
enum Packets {
    Literal(LiteralPacket),
    Operator(OperatorPacket),
}

#[derive(Debug)]
struct LiteralPacket {
    version: u64,
    // type_id: u64,
    // value: u64,
}

impl LiteralPacket {
    fn parse(reader: &mut Vec<char>, version: u64, _type_id: u64) -> (Self, u64) {
        let mut bits = Vec::new();
        let mut bits_read = 0;
        loop {
            let should_continue = reader.drain(0..1).next().unwrap();
            bits_read += 1;

            let val = reader.drain(0..4);
            bits_read += 4;

            bits.append(&mut val.collect_vec());

            if should_continue == '0' {
                break;
            }
        }

        let _value = u64::from_str_radix(&bits.iter().join(""), 2).unwrap();

        (
            Self {
                version,
                // type_id,
                // value,
            },
            bits_read,
        )
    }
}

#[derive(Debug)]
struct OperatorPacket {
    version: u64,
    // type_id: u64,
    // length_type_id: u64,
    // length: u64,
    sub_packets: Vec<Packets>,
}

impl OperatorPacket {
    fn parse(reader: &mut Vec<char>, version: u64, _type_id: u64) -> (Self, u64) {
        let mut bits_read = 0;
        let length_type_id: u64 = reader
            .drain(0..1)
            .next()
            .unwrap()
            .to_digit(10)
            .unwrap()
            .into();
        bits_read += 1;

        let length = if length_type_id == 0 {
            let length = u64::from_str_radix(&reader.drain(0..15).join(""), 2).unwrap();
            bits_read += 15;
            length
        } else {
            let length = u64::from_str_radix(&reader.drain(0..11).join(""), 2).unwrap();
            bits_read += 11;
            length
        };

        let mut sub_packets = Vec::new();
        let mut sub_bits_read = 0;
        let mut sub_packets_read = 0;
        loop {
            let (sub_packet, sbr) = Packets::parse(reader);
            sub_packets.push(sub_packet);
            sub_bits_read += sbr;
            bits_read += sbr;
            sub_packets_read += 1;

            if length_type_id == 0 && sub_bits_read == length {
                break;
            }

            if length_type_id == 1 && sub_packets_read == length {
                break;
            }
        }

        (
            Self {
                version,
                // type_id,
                // length_type_id,
                // length,
                sub_packets,
            },
            bits_read,
        )
    }
}

impl Packets {
    fn parse(reader: &mut Vec<char>) -> (Self, u64) {
        let mut bits_read = 0;
        let version = u64::from_str_radix(&reader.drain(0..3).join(""), 2).unwrap();
        bits_read += 3;
        let type_id = u64::from_str_radix(&reader.drain(0..3).join(""), 2).unwrap();
        bits_read += 3;

        if type_id == 4 {
            let (data, br) = LiteralPacket::parse(reader, version, type_id);
            (Self::Literal(data), bits_read + br)
        } else {
            let (data, br) = OperatorPacket::parse(reader, version, type_id);
            (Self::Operator(data), bits_read + br)
        }
    }
}

fn sum_versions(packet: Packets) -> u64 {
    let mut sum = 0;
    match packet {
        Packets::Literal(literal) => {
            sum += literal.version;
        }

        Packets::Operator(operator) => {
            sum += operator.version;
            for p in operator.sub_packets {
                sum += sum_versions(p)
            }
        }
    }
    sum
}

fn main() {
    let mut input: Vec<char> = fs::read_to_string("inputs/day_16.txt")
        .unwrap()
        .trim()
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| {
            let n = u64::from_str_radix(&format!("{}", c), 16).unwrap();

            format!("{:04b}", n).chars().collect::<Vec<char>>()
        })
        .flatten()
        .collect();

    let (packet, _) = Packets::parse(&mut input);
    let version_sum = sum_versions(packet);
    println!("{:#?}", version_sum);
}
