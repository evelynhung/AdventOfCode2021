use advent_of_code::read_file_to_string;
use bitvec::prelude::*;

type BitStream = BitVec<Msb0, u8>;

#[derive(Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    value: u64,
    sub_packets: Vec<Packet>,
}

impl Packet {
    fn from_payload(payload: &str) -> Packet {
        let payload = payload.trim().trim_matches('\n');
        let mut bitvec = Packet::hex_string_to_bitvec(payload);
        Packet::from_bit_stream(&mut bitvec)
    }

    fn hex_string_to_bitvec(hex_string: &str) -> BitStream {
        assert_eq!(0, hex_string.len() % 2);
        (0..hex_string.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex_string[i..i + 2], 16).unwrap())
            .collect()
    }

    fn from_bit_stream(bits: &mut BitStream) -> Packet {
        let version = bits.drain(..3).as_bitslice().load_be();
        let type_id = bits.drain(..3).as_bitslice().load_be();
        match type_id {
            4 => {
                let value = Packet::load_literal_value(bits);
                Packet {version, type_id, value, sub_packets: vec![]}
            }
            _ => {
                let sub_packets = Packet::load_sub_packets(bits);
                Packet {version, type_id, value: 0, sub_packets}
            }
        }
    }

    fn load_literal_value(bits: &mut BitStream) -> u64 {
        let mut value_bits = BitStream::new();
        let mut continuing = true;
        while continuing {
            let group: BitStream = bits.drain(..5).collect();
            continuing = group[0];
            value_bits.extend(&group[1..]);
        }

        value_bits.load_be::<u64>()
    }

    fn load_sub_packets(bits: &mut BitStream) -> Vec<Packet> {
        let mut packets = vec![];
        let len_id = bits[0];
        bits.drain(..1);
        match len_id {
            false => {
                let bit_len = bits.drain(..15).as_bitslice().load_be::<usize>();
                let mut sub_packets_bits: BitStream = bits.drain(..bit_len).collect();

                while !sub_packets_bits.is_empty() {
                    packets.push(Packet::from_bit_stream(&mut sub_packets_bits));
                }
            }
            true => {
                let pkg_count = bits.drain(..11).as_bitslice().load_be::<usize>();
                for _ in 0..pkg_count {
                    packets.push(Packet::from_bit_stream(bits));
                }
            }
        }
        packets
    }

    fn version_sum(&self) -> u32 {
        self.sub_packets.iter()
            .map(|sub| sub.version_sum())
            .fold(self.version as u32,  |acc, x| acc + x)
    }

    fn evaluate(&self) -> u64 {
        match self.type_id {
            4 => self.value,
            0 => self.sub_packets.iter().map(|sub| sub.evaluate()).sum(),
            1 => self.sub_packets.iter().map(|sub| sub.evaluate()).product(),
            2 => self.sub_packets.iter().map(|sub| sub.evaluate()).min().unwrap(),
            3 => self.sub_packets.iter().map(|sub| sub.evaluate()).max().unwrap(),
            type_id => {
                assert_eq!(2, self.sub_packets.len());
                let (sub1, sub2) = (&self.sub_packets[0], &self.sub_packets[1]);
                match (type_id, sub1.evaluate().cmp(&sub2.evaluate())) {
                    (5, std::cmp::Ordering::Greater) => 1,
                    (6, std::cmp::Ordering::Less) => 1,
                    (7, std::cmp::Ordering::Equal) => 1,
                    _ => 0,
                }
            }
        }
    }
}

fn main() {
    let input = read_file_to_string(env!("CARGO_BIN_NAME"));
    let packet = Packet::from_payload(&input);
    println!("{}", packet.version_sum());
    println!("{}", packet.evaluate());
}

#[cfg(test)]
mod tests {
    use crate::Packet;

    #[test]
    fn test_packet_from_payload() {
        let input = "D2FE28";
        let packet = Packet::from_payload(&input);
        assert_eq!(6, packet.version);
        assert_eq!(4, packet.type_id);
        assert_eq!(2021, packet.value);

        let input = "38006F45291200";
        let packet = Packet::from_payload(&input);
        assert_eq!(1, packet.version);
        assert_eq!(6, packet.type_id);
        assert_eq!(10, packet.sub_packets[0].value);
        assert_eq!(20, packet.sub_packets[1].value);

        let input = "EE00D40C823060";
        let packet = Packet::from_payload(&input);
        assert_eq!(7, packet.version);
        assert_eq!(3, packet.type_id);
        assert_eq!(1, packet.sub_packets[0].value);
        assert_eq!(2, packet.sub_packets[1].value);
        assert_eq!(3, packet.sub_packets[2].value);
    }

    #[test]
    fn test_packet_version_sum() {
        let input = "8A004A801A8002F478";
        let packet = Packet::from_payload(&input);
        assert_eq!(4, packet.version);
        assert_eq!(1, packet.sub_packets[0].version);
        assert_eq!(5, packet.sub_packets[0].sub_packets[0].version);
        assert_eq!(
            6,
            packet.sub_packets[0].sub_packets[0].sub_packets[0].version
        );
        assert_eq!(16, packet.version_sum());

        let input = "620080001611562C8802118E34";
        assert_eq!(12, Packet::from_payload(&input).version_sum());

        let input = "C0015000016115A2E0802F182340";
        assert_eq!(23, Packet::from_payload(&input).version_sum());

        let input = "A0016C880162017C3686B18A3D4780";
        assert_eq!(31, Packet::from_payload(&input).version_sum());
    }

    #[test]
    fn test_packet_evaluate() {
        let input = "C200B40A82";
        assert_eq!(3, Packet::from_payload(&input).evaluate());

        let input = "04005AC33890";
        assert_eq!(54, Packet::from_payload(&input).evaluate());

        let input = "880086C3E88112";
        assert_eq!(7, Packet::from_payload(&input).evaluate());

        let input = "CE00C43D881120";
        assert_eq!(9, Packet::from_payload(&input).evaluate());

        let input = "D8005AC2A8F0";
        assert_eq!(1, Packet::from_payload(&input).evaluate());

        let input = "F600BC2D8F";
        assert_eq!(0, Packet::from_payload(&input).evaluate());

        let input = "9C005AC2F8F0";
        assert_eq!(0, Packet::from_payload(&input).evaluate());

        let input = "9C0141080250320F1802104A08";
        assert_eq!(1, Packet::from_payload(&input).evaluate());
    }
}
