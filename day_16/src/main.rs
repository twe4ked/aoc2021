fn main() {
    let input = to_binary_string(include_str!("../input").trim());
    let (_len, packet) = Packet::parse(&input);

    let part_1 = packet.version_sum();
    println!("Part 1: {}", part_1);
    assert_eq!(873, part_1);

    let part_2 = packet.calculate();
    println!("Part 2: {}", part_2);
    assert_eq!(402817863665, part_2);
}

#[derive(Debug, PartialEq)]
enum Op {
    Sum,
    Product,
    Minimum,
    Maximum,
    Greater,
    Less,
    Equal,
}

impl From<u8> for Op {
    fn from(type_id: u8) -> Self {
        use Op::*;
        match type_id {
            0 => Sum,
            1 => Product,
            2 => Minimum,
            3 => Maximum,
            4 => unreachable!(),
            5 => Greater,
            6 => Less,
            7 => Equal,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Packet {
    Literal {
        version: u8,
        value: u64,
    },
    Operator {
        version: u8,
        op: Op,
        packets: Vec<Packet>,
    },
}

impl Packet {
    fn parse(input: &str) -> (usize, Self) {
        // Every packet begins with a standard header: the first three bits encode the packet version,
        // and the next three bits encode the packet type ID.
        let version = u8::from_str_radix(&input[0..3], 2).unwrap();
        let type_id = u8::from_str_radix(&input[3..6], 2).unwrap();

        let mut i = 6;

        match type_id {
            // Literal packet
            4 => {
                let mut binary_integer = String::new();
                loop {
                    binary_integer.push_str(&input[i + 1..i + 5]);
                    match &input[i..i + 1] {
                        "1" => i += 5, // Keep reading
                        "0" => {
                            // Last part of the number
                            i += 5;
                            break;
                        }
                        _ => unreachable!(),
                    }
                }

                let value = u64::from_str_radix(&binary_integer, 2).unwrap();

                (i, Packet::Literal { version, value })
            }
            // Operator packet
            _ => {
                let op = Op::from(type_id);

                // Length type ID
                match &input[i..i + 1] {
                    "0" => {
                        // If the length type ID is 0, then the next 15 bits are a number that
                        // represents the total length in bits of the sub-packets contained by this
                        // packet.
                        let sub_packets_len =
                            usize::from_str_radix(&input[i + 1..i + 16], 2).unwrap();
                        i += 16;

                        let mut packets = Vec::new();
                        let read_until = i + sub_packets_len;
                        while i < read_until {
                            let (len, packet) = Packet::parse(&input[i..]);
                            i += len;
                            packets.push(packet)
                        }

                        (
                            i,
                            Packet::Operator {
                                version,
                                op,
                                packets,
                            },
                        )
                    }
                    "1" => {
                        // If the length type ID is 1, then the next 11 bits are a number that
                        // represents the number of sub-packets immediately contained by this
                        // packet.
                        let sub_packets_count =
                            usize::from_str_radix(&input[i + 1..i + 12], 2).unwrap();
                        i += 12;

                        let mut packets = Vec::new();
                        while packets.len() < sub_packets_count {
                            let (len, packet) = Packet::parse(&input[i..]);
                            i += len;
                            packets.push(packet)
                        }

                        (
                            i,
                            Packet::Operator {
                                version,
                                op,
                                packets,
                            },
                        )
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    fn version_sum(&self) -> u64 {
        match self {
            Packet::Literal { version, .. } => *version as u64,
            Packet::Operator {
                version, packets, ..
            } => *version as u64 + packets.iter().map(|p| p.version_sum()).sum::<u64>(),
        }
    }

    fn calculate(&self) -> u64 {
        match self {
            Packet::Literal { value, .. } => *value as u64,
            Packet::Operator { op, packets, .. } => match op {
                Op::Sum => packets.iter().map(|p| p.calculate()).sum::<u64>(),
                Op::Product => packets.iter().map(|p| p.calculate()).product::<u64>(),
                Op::Minimum => packets.iter().map(|p| p.calculate()).min().unwrap() as u64,
                Op::Maximum => packets.iter().map(|p| p.calculate()).max().unwrap() as u64,
                Op::Greater => (packets[0].calculate() > packets[1].calculate()) as u64,
                Op::Less => (packets[0].calculate() < packets[1].calculate()) as u64,
                Op::Equal => (packets[0].calculate() == packets[1].calculate()) as u64,
            },
        }
    }
}

fn to_binary_string(input: &str) -> String {
    input
        .chars()
        .map(|c| format!("{:04b}", c.to_digit(16).unwrap()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use Packet::*;

    #[test]
    fn part_1() {
        let example = to_binary_string("D2FE28");
        assert_eq!("110100101111111000101000", &example);
        let (len, packet) = Packet::parse(&example);
        assert_eq!(
            Literal {
                version: 6,
                value: 2021
            },
            packet,
        );
        assert_eq!(example.len() - 3, len);

        let example = to_binary_string("38006F45291200");
        assert_eq!(
            "00111000000000000110111101000101001010010001001000000000",
            &example
        );
        let (len, packet) = Packet::parse(&example);
        assert_eq!(
            Operator {
                version: 1,
                op: Op::Less,
                packets: vec![
                    Literal {
                        version: 6,
                        value: 10
                    },
                    Literal {
                        version: 2,
                        value: 20
                    }
                ]
            },
            packet
        );
        assert_eq!(example.len() - 7, len);

        let example = to_binary_string("EE00D40C823060");
        assert_eq!(
            "11101110000000001101010000001100100000100011000001100000",
            &example
        );
        let (len, packet) = Packet::parse(&example);
        assert_eq!(
            Operator {
                version: 7,
                op: Op::Maximum,
                packets: vec![
                    Literal {
                        version: 2,
                        value: 1
                    },
                    Literal {
                        version: 4,
                        value: 2
                    },
                    Literal {
                        version: 1,
                        value: 3
                    }
                ]
            },
            packet
        );
        assert_eq!(example.len() - 5, len);

        let example = to_binary_string("8A004A801A8002F478");
        let (len, packet) = Packet::parse(&example);
        assert_eq!(16, packet.version_sum());
        assert_eq!(example.len() - 3, len);

        let example = to_binary_string("620080001611562C8802118E34");
        let (_len, packet) = Packet::parse(&example);
        assert_eq!(12, packet.version_sum());

        let example = to_binary_string("C0015000016115A2E0802F182340");
        let (_len, packet) = Packet::parse(&example);
        assert_eq!(23, packet.version_sum());

        let example = to_binary_string("A0016C880162017C3686B18A3D4780");
        let (_len, packet) = Packet::parse(&example);
        assert_eq!(31, packet.version_sum());
    }

    #[test]
    fn part_2() {
        // C200B40A82 finds the sum of 1 and 2, resulting in the value 3.
        let example = to_binary_string("C200B40A82");
        let (_len, packet) = Packet::parse(&example);
        assert_eq!(3, packet.calculate());

        // 04005AC33890 finds the product of 6 and 9, resulting in the value 54.
        let example = to_binary_string("04005AC33890");
        let (_len, packet) = Packet::parse(&example);
        assert_eq!(54, packet.calculate());

        // 880086C3E88112 finds the minimum of 7, 8, and 9, resulting in the value 7.
        let example = to_binary_string("880086C3E88112");
        let (_len, packet) = Packet::parse(&example);
        assert_eq!(7, packet.calculate());

        // CE00C43D881120 finds the maximum of 7, 8, and 9, resulting in the value 9.
        let example = to_binary_string("CE00C43D881120");
        let (_len, packet) = Packet::parse(&example);
        assert_eq!(9, packet.calculate());

        // D8005AC2A8F0 produces 1, because 5 is less than 15.
        let example = to_binary_string("D8005AC2A8F0");
        let (_len, packet) = Packet::parse(&example);
        assert_eq!(1, packet.calculate());

        // F600BC2D8F produces 0, because 5 is not greater than 15.
        let example = to_binary_string("F600BC2D8F");
        let (_len, packet) = Packet::parse(&example);
        assert_eq!(0, packet.calculate());

        // 9C005AC2F8F0 produces 0, because 5 is not equal to 15.
        let example = to_binary_string("9C005AC2F8F0");
        let (_len, packet) = Packet::parse(&example);
        assert_eq!(0, packet.calculate());

        // 9C0141080250320F1802104A08 produces 1, because 1 + 3 = 2 * 2.
        let example = to_binary_string("9C0141080250320F1802104A08");
        let (_len, packet) = Packet::parse(&example);
        assert_eq!(1, packet.calculate());
    }
}
