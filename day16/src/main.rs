use anyhow::{bail, ensure, Context, Result};
use bitvec::{prelude::*, view::AsBits};
use std::io::{self, Read};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let bits = read_raw(&input)?;
    let packet = Packet::from(&bits)?;

    part1(&packet)?;
    part2(&packet)?;

    Ok(())
}

fn part1(packet: &Packet) -> Result<()> {
    let sum = packet
        .flatten()
        .iter()
        .fold(0, |acc, p| acc + p.version as u64);

    println!("Part 1 answer: {}", sum);

    Ok(())
}

fn part2(packet: &Packet) -> Result<()> {
    println!("Part 2 answer: {}", packet.payload.value()?);

    Ok(())
}

fn read_raw(s: &str) -> Result<PacketBitVec> {
    let s = s.trim();
    let bytes = (0..s.len())
        .step_by(2)
        .map(|i| {
            let j = (i + 1).min(s.len() - 1);
            u8::from_str_radix(&s[i..=j], 16).context("bad packet data")
        })
        .collect::<Result<Vec<u8>>>()?;

    Ok(bytes.as_bits().to_bitvec())
}

type PacketVersion = u8;
type PacketValue = u64;
type PacketBitVec = BitVec<Msb0, u8>;
type PacketBitSlice = BitSlice<Msb0, u8>;

struct Packet {
    version: PacketVersion,
    payload: PacketPayload,
}

impl Packet {
    fn from(slice: &PacketBitSlice) -> Result<Self> {
        Self::build(slice).map(|p| p.0)
    }

    fn build(slice: &PacketBitSlice) -> Result<(Packet, &PacketBitSlice)> {
        let kind: u8 = slice[3..6].load_be();
        if kind == 4 {
            Self::build_literal(slice)
        } else {
            Self::build_operator(slice)
        }
    }

    fn build_literal(slice: &PacketBitSlice) -> Result<(Packet, &PacketBitSlice)> {
        let version: u8 = slice[..3].load_be();
        let mut value = PacketBitVec::new();
        let mut i = 6;

        loop {
            let is_last = !slice[i];
            let mut group = slice[i + 1..i + 5].to_bitvec();
            value.append(&mut group);

            i += 5;

            if is_last {
                break;
            }
        }

        let packet = Packet {
            version,
            payload: PacketPayload::Literal {
                value: value.load_be(),
            },
        };
        let next_slice = &slice[i..];

        Ok((packet, next_slice))
    }

    fn build_operator(slice: &PacketBitSlice) -> Result<(Packet, &PacketBitSlice)> {
        let version: u8 = slice[..3].load_be();
        let kind = PacketOperatorKind::from(slice[3..6].load_be())?;
        let is_len_bits = !slice[6];

        let mut packets = vec![];
        let mut next_slice;

        if is_len_bits {
            let mut len_bits = slice[7..22].load_be::<u16>() as usize;
            next_slice = &slice[22..];
            let mut rem_bits = next_slice.len();

            while len_bits > 0 {
                let (packet, rem_slice) = Self::build(next_slice)?;
                packets.push(packet);

                let packet_size = rem_bits - rem_slice.len();
                len_bits -= packet_size;
                rem_bits = rem_slice.len();
                next_slice = rem_slice;
            }
        } else {
            let total_packets = slice[7..18].load_be::<u16>();
            next_slice = &slice[18..];

            for _ in 0..total_packets {
                let (packet, rem_slice) = Self::build(next_slice)?;
                packets.push(packet);

                next_slice = rem_slice;
            }
        };

        let packet = Packet {
            version,
            payload: PacketPayload::Operator { kind, packets },
        };

        Ok((packet, next_slice))
    }

    fn flatten(&self) -> Vec<&Packet> {
        let mut packets = vec![];
        Self::accumulate(self, &mut packets);
        packets
    }

    fn accumulate<'a, 'b>(packet: &'a Packet, acc: &'b mut Vec<&'a Packet>) {
        acc.push(packet);
        if let PacketPayload::Operator { kind: _, packets } = &packet.payload {
            for packet in packets {
                Self::accumulate(packet, acc);
            }
        }
    }
}

enum PacketPayload {
    Literal {
        value: PacketValue,
    },
    Operator {
        kind: PacketOperatorKind,
        packets: Vec<Packet>,
    },
}

impl PacketPayload {
    fn value(&self) -> Result<PacketValue> {
        match self {
            Self::Literal { value } => Ok(*value),
            Self::Operator { kind, packets } => {
                let values = packets
                    .iter()
                    .map(|p| p.payload.value())
                    .collect::<Result<Vec<_>>>()?;
                kind.evaluate(&values)
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PacketOperatorKind {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl PacketOperatorKind {
    fn from(value: u8) -> Result<Self> {
        match value {
            0 => Ok(Self::Sum),
            1 => Ok(Self::Product),
            2 => Ok(Self::Minimum),
            3 => Ok(Self::Maximum),
            5 => Ok(Self::GreaterThan),
            6 => Ok(Self::LessThan),
            7 => Ok(Self::EqualTo),
            _ => bail!("invalid operator kind: {}", value),
        }
    }

    fn evaluate(&self, values: &[PacketValue]) -> Result<PacketValue> {
        match self {
            Self::Sum => Ok(values.iter().sum()),
            Self::Product => Ok(values.iter().product()),
            Self::Minimum => {
                ensure!(!values.is_empty(), "minimum operation needs args");
                Ok(*values.iter().min().unwrap())
            }
            Self::Maximum => {
                ensure!(!values.is_empty(), "maximum operation needs args");
                Ok(*values.iter().max().unwrap())
            }
            Self::GreaterThan => {
                ensure!(values.len() == 2, "greater-than operation needs two args");
                Ok((values[0] > values[1]) as PacketValue)
            }
            Self::LessThan => {
                ensure!(values.len() == 2, "less-than operation needs two args");
                Ok((values[0] < values[1]) as PacketValue)
            }
            Self::EqualTo => {
                ensure!(values.len() == 2, "equal-to operation needs two args");
                Ok((values[0] == values[1]) as PacketValue)
            }
        }
    }
}
