/*
** src/puzzles/aoc2021/day16.rs
*/

use super::Solution;

#[derive(Debug, PartialEq)]
enum PacketType {
    Sum,
    Product,
    Minimum,
    Maximum,
    Literal,
    Greater,
    Less,
    Equal,
}

impl From<u8> for PacketType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Sum,
            1 => Self::Product,
            2 => Self::Minimum,
            3 => Self::Maximum,
            4 => Self::Literal,
            5 => Self::Greater,
            6 => Self::Less,
            7 => Self::Equal,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
enum PacketData {
    Literal(u64),
    Subpackets(Vec<Packet>),
}

#[derive(Debug, PartialEq)]
struct Packet {
    version: u8,
    type_id: PacketType,
    length_type_id: u8,
    data: PacketData,
}

impl Packet {
    fn literal(&self) -> u64 {
        match self.data {
            PacketData::Literal(n) => n,
            _ => unreachable!(),
        }
    }

    fn subpackets(&self) -> &Vec<Packet> {
        match &self.data {
            PacketData::Subpackets(subpackets) => subpackets,
            _ => unreachable!(),
        }
    }

    fn version_sum(&self) -> u64 {
        let v = self.version as u64;
        match self.type_id {
            PacketType::Literal => v,
            _ => {
                v + self
                    .subpackets()
                    .iter()
                    .map(|p| p.version_sum())
                    .sum::<u64>()
            }
        }
    }

    fn evaluate(&self) -> u64 {
        match self.type_id {
            PacketType::Literal => self.literal(),
            PacketType::Sum => self
                .subpackets()
                .iter()
                .map(|packet| packet.evaluate())
                .sum(),
            PacketType::Product => self
                .subpackets()
                .iter()
                .map(|packet| packet.evaluate())
                .product(),
            PacketType::Minimum => self
                .subpackets()
                .iter()
                .map(|packet| packet.evaluate())
                .min()
                .unwrap(),
            PacketType::Maximum => self
                .subpackets()
                .iter()
                .map(|packet| packet.evaluate())
                .max()
                .unwrap(),
            PacketType::Greater => match self.subpackets().as_slice() {
                [packet_a, packet_b] => {
                    if packet_a.evaluate() > packet_b.evaluate() {
                        1
                    } else {
                        0
                    }
                }
                _ => unreachable!(),
            },
            PacketType::Less => match self.subpackets().as_slice() {
                [packet_a, packet_b] => {
                    if packet_a.evaluate() < packet_b.evaluate() {
                        1
                    } else {
                        0
                    }
                }
                _ => unreachable!(),
            },
            PacketType::Equal => match self.subpackets().as_slice() {
                [packet_a, packet_b] => {
                    if packet_a.evaluate() == packet_b.evaluate() {
                        1
                    } else {
                        0
                    }
                }
                _ => unreachable!(),
            },
        }
    }
}

fn parse_transmission(input: String) -> Vec<u8> {
    let chars = input.chars().collect::<Vec<_>>();
    let n_chars = chars.len();

    let mut data = Vec::with_capacity(n_chars);
    for c in 0..(n_chars / 2) {
        let b0 = chars[c * 2].to_digit(16).unwrap() as u8;
        let b1 = chars[(c * 2) + 1].to_digit(16).unwrap() as u8;
        data.push((b0 << 4) | b1);
    }
    if n_chars % 2 == 1 {
        let b = chars[n_chars - 1].to_digit(16).unwrap() as u8;
        data.push(b << 4);
    }

    data
}

fn grab_bit(data: &[u8], byte_offset: &mut usize, bit_offset: &mut usize) -> u8 {
    let offset = 7 - *bit_offset;
    let mask = 0x1 << offset;
    let bit = (data[*byte_offset] & mask) >> offset;

    *bit_offset += 1;
    if *bit_offset == 8 {
        *byte_offset += 1;
        *bit_offset = 0;
    }

    bit
}

fn grab_bits<const N: usize>(data: &[u8], byte_offset: &mut usize, bit_offset: &mut usize) -> u64 {
    // grab bits
    let mut bits = [0; N];
    for bit in bits.iter_mut().take(N) {
        let offset = 7 - *bit_offset;
        let mask = 0x1 << offset;
        *bit = (data[*byte_offset] & mask) >> offset;

        *bit_offset += 1;
        if *bit_offset == 8 {
            *byte_offset += 1;
            *bit_offset = 0;
        }
    }
    // combine into a single integer
    let mut n = 0u64;
    for (i, &b) in bits.iter().rev().enumerate() {
        n |= (b as u64) << i;
    }
    n
}

fn parse_packet_header(
    data: &[u8],
    byte_offset: &mut usize,
    bit_offset: &mut usize,
) -> (u8, PacketType, u8) {
    let version = grab_bits::<3>(data, byte_offset, bit_offset) as u8;
    let type_id = grab_bits::<3>(data, byte_offset, bit_offset) as u8;
    // note: length type ID is only valid for operators
    let length_type_id = match type_id {
        4 => 0,
        _ => grab_bit(data, byte_offset, bit_offset),
    };

    (version, type_id.into(), length_type_id)
}

fn parse_packet_literal(data: &[u8], byte_offset: &mut usize, bit_offset: &mut usize) -> u64 {
    let flag = 0x10;

    // grab the chunks of the literal
    let mut chunks = vec![];
    while chunks.is_empty() || chunks[chunks.len() - 1] & flag == flag {
        let chunk = grab_bits::<5>(data, byte_offset, bit_offset) as u8;
        chunks.push(chunk);
    }

    let mut n = 0;
    let mask = 0xF;
    for (byte, chunk) in chunks.iter().rev().enumerate() {
        n |= ((chunk & mask) as u64) << (byte * 4);
    }

    n
}

fn parse_packet_operator_length(
    data: &[u8],
    length_type_id: u8,
    byte_offset: &mut usize,
    bit_offset: &mut usize,
) -> u16 {
    match length_type_id {
        // operator length is 15 bits
        0 => grab_bits::<15>(data, byte_offset, bit_offset) as u16,
        // operator length is 11 bits
        1 => grab_bits::<11>(data, byte_offset, bit_offset) as u16,
        _ => unreachable!(),
    }
}

fn parse_subpacket(data: &[u8], byte_offset: &mut usize, bit_offset: &mut usize) -> Packet {
    // parse the packet header
    let (version, type_id, length_type_id) = parse_packet_header(data, byte_offset, bit_offset);

    // parse the remaining portion of the packet based on the type ID
    let packet_data = match type_id {
        // literal
        PacketType::Literal => {
            let literal = parse_packet_literal(data, byte_offset, bit_offset);
            PacketData::Literal(literal)
        }
        // operator
        _ => {
            let mut subpackets = vec![];
            let op_length =
                parse_packet_operator_length(data, length_type_id, byte_offset, bit_offset)
                    as usize;
            match length_type_id {
                0 => {
                    // length is the total length in bits of the subpackets
                    let end = (*byte_offset * 8) + *bit_offset + op_length;
                    while (*byte_offset * 8) + *bit_offset < end {
                        let subpacket = parse_subpacket(data, byte_offset, bit_offset);
                        subpackets.push(subpacket);
                    }
                }
                1 => {
                    // length is the number of subpackets
                    for _ in 0..op_length {
                        let subpacket = parse_subpacket(data, byte_offset, bit_offset);
                        subpackets.push(subpacket);
                    }
                }
                _ => unreachable!(),
            }
            PacketData::Subpackets(subpackets)
        }
    };

    Packet {
        version,
        type_id,
        length_type_id,
        data: packet_data,
    }
}

fn parse_packet(data: &[u8], byte_offset: &mut usize, bit_offset: &mut usize) -> Packet {
    // parse the packet header
    let (version, type_id, length_type_id) = parse_packet_header(data, byte_offset, bit_offset);

    // parse the remaining portion of the packet based on the type ID
    let packet_data = match type_id {
        // literal
        PacketType::Literal => {
            let literal = parse_packet_literal(data, byte_offset, bit_offset);
            PacketData::Literal(literal)
        }
        // operator
        _ => {
            let mut subpackets = vec![];
            let op_length =
                parse_packet_operator_length(data, length_type_id, byte_offset, bit_offset)
                    as usize;
            match length_type_id {
                0 => {
                    // length is the total length in bits of the subpackets
                    let end = (*byte_offset * 8) + *bit_offset + op_length;
                    while (*byte_offset * 8) + *bit_offset < end {
                        let subpacket = parse_subpacket(data, byte_offset, bit_offset);
                        subpackets.push(subpacket);
                    }
                }
                1 => {
                    // length is the number of subpackets
                    for _ in 0..op_length {
                        let subpacket = parse_subpacket(data, byte_offset, bit_offset);
                        subpackets.push(subpacket);
                    }
                }
                _ => unreachable!(),
            }
            PacketData::Subpackets(subpackets)
        }
    };

    // account for trailing bits
    if *bit_offset != 0 {
        *byte_offset += 1;
        *bit_offset = 0;
    }

    Packet {
        version,
        type_id,
        length_type_id,
        data: packet_data,
    }
}

fn parse_packets(transmission: Vec<u8>) -> Vec<Packet> {
    let mut packets = vec![];
    let mut byte_offset = 0;
    let mut bit_offset = 0;
    while byte_offset < transmission.len() {
        let packet = parse_packet(&transmission, &mut byte_offset, &mut bit_offset);
        packets.push(packet);
    }
    packets
}

fn parse_input(input: String) -> Vec<Packet> {
    let transmission = parse_transmission(input);
    parse_packets(transmission)
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    let packets = parse_input(input);

    // Part A: Decode the structure of your hexadecimal-encoded BITS transmission; what do you get
    // if you add up the version numbers in all packets?
    let version_sum = packets
        .iter()
        .map(|packet| packet.version_sum())
        .sum::<u64>();
    solution.set_part_a(version_sum);

    // Part B: What do you get if you evaluate the expression represented by your hexadecimal-
    // encoded BITS transmission?
    solution.set_part_b(packets[0].evaluate());

    solution
}
