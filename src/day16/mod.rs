use itertools::Itertools;
use std::io;

enum PacketBody {
    LiteralValue(usize),
    Operator(u8, Vec<Packet>),
}
struct Packet {
    version: u8,
    body: PacketBody,
}

struct Decoder<T: Iterator<Item = u8>> {
    it: T,
    buffer: u8,
    capacity: u32,
}

struct SubDecoder<'a> {
    decoder: &'a mut dyn DecoderTrait,
    bits: u32,
}

impl<'a> DecoderTrait for SubDecoder<'a> {
    fn read_bit(&mut self) -> Result<u8> {
        if self.bits > 0 {
            self.bits -= 1;
            self.decoder.read_bit()
        } else {
            Ok(0)
        }
    }

    fn read_bits(&mut self, n: u32) -> Result<usize> {
        if self.bits >= n {
            self.bits -= n;
            self.decoder.read_bits(n)
        } else if self.bits > 0 {
            let ret = match self.decoder.read_bits(self.bits) {
                Ok(v) => Ok(v << (n - self.bits)),
                Err(e) => Err(e),
            };
            self.bits = 0;
            ret
        } else {
            Ok(0)
        }
    }
}

impl<'a> SubDecoder<'a> {
    fn new(decoder: &'a mut dyn DecoderTrait, bits: u32) -> Self {
        SubDecoder { decoder, bits }
    }
}

type Result<T> = std::result::Result<T, std::io::Error>;

trait DecoderTrait {
    fn read_bit(&mut self) -> Result<u8>;
    fn read_bits(&mut self, n: u32) -> Result<usize>;
    fn decode_unsigned_integer(&mut self) -> Result<usize> {
        // 1XXXX ... 1XXXX 0XXXX
        let mut result = self.read_bits(5)?.into();
        // < 16: just one chunk, optimal case
        if result < 16 {
            return Ok(result);
        }

        // ELSE: multiple bytes...
        result &= 0xf;
        let mut shift: u32 = 4;
        loop {
            // 1. Read the next chunk
            let b: usize = self.read_bits(5)?.into();
            // 2. Multiply the value of the unsigned number represented by
            // the 4 least significant
            // bits of the octet by the current multiplier and add the
            // result to the current value.
            result += match (b & 0xf).checked_shl(shift) {
                Some(v) => v,
                None => {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        "UnsignedInteger size too large to be stored in usize",
                    ))
                }
            };
            // 3. Multiply the multiplier by 128
            shift += 4;
            // 4. If the most significant bit of the octet was 1, go back to
            // step 1
            if b < 16 {
                break;
            }
        }
        Ok(result)
    }
}

impl<T: Iterator<Item = u8>> Decoder<T> {
    const BUFFER_CAPACITY: u32 = u8::BITS;
    const EOS_MESSAGE: &'static str = "Premature EOS found while reading data.";

    fn from(it: T) -> Self {
        Decoder {
            it,
            buffer: 0,
            capacity: 0,
        }
    }

    // read direct byte
    fn read_direct_byte(&mut self) -> Result<u8> {
        self.it
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, Self::EOS_MESSAGE))
    }

    /**
     * If buffer is empty, read byte from underlying stream.
     */
    fn read_buffer(&mut self) -> Result<()> {
        assert_eq!(self.capacity, 0);
        self.buffer = self.read_direct_byte()?;
        self.capacity = Self::BUFFER_CAPACITY;
        Ok(())
    }
}

impl<T: Iterator<Item = u8>> DecoderTrait for Decoder<T> {
    fn read_bit(&mut self) -> Result<u8> {
        if self.capacity == 0 {
            self.read_buffer()?;
        }
        self.capacity -= 1;
        Ok((self.buffer >> self.capacity) & 0x1)
    }

    fn read_bits(&mut self, n: u32) -> Result<usize> {
        assert!(n > 0);

        if n <= self.capacity {
            // buffer already holds all necessary bits
            self.capacity -= n;
            return Ok(
                ((self.buffer >> self.capacity) & (0xff_u8 >> (Self::BUFFER_CAPACITY - n))).into(),
            );
        }
        if self.capacity == 0 && n == Self::BUFFER_CAPACITY {
            // possible to read direct byte, nothing else to do
            return Ok(self.read_direct_byte()?.into());
        }

        // get as many bits from buffer as possible
        let mut result: usize = (self.buffer
            & 0xff_u8
                .checked_shr(Self::BUFFER_CAPACITY - self.capacity)
                .unwrap_or(0))
        .into();
        let mut n = n - self.capacity;
        self.capacity = 0;
        // possibly read whole bytes
        while n > 7 {
            if self.capacity == 0 {
                self.read_buffer()?;
            }
            result = result << Self::BUFFER_CAPACITY;
            let a: usize = self.buffer.into();
            result |= a;
            n -= Self::BUFFER_CAPACITY;
            self.capacity = 0;
        }
        // read the rest of the bits
        if n > 0 {
            if self.capacity == 0 {
                self.read_buffer()?;
            }
            self.capacity = Self::BUFFER_CAPACITY - n;
            result = result << n;
            let a: usize = (self.buffer >> self.capacity).into();
            result |= a;
        }
        Ok(result)
    }
}

fn parse_packet<T: DecoderTrait>(decoder: &mut T) -> Packet {
    let version = decoder.read_bits(3).unwrap() as u8;
    let t = decoder.read_bits(3).unwrap() as u8;
    if t == 4 {
        Packet {
            version,
            body: PacketBody::LiteralValue(decoder.decode_unsigned_integer().unwrap()),
        }
    } else {
        let i = decoder.read_bit().unwrap();
        if i != 0 {
            let packets_number = decoder.read_bits(11).unwrap();
            assert_ne!(packets_number, 1718);
            Packet {
                version,
                body: PacketBody::Operator(
                    t,
                    (0..packets_number).map(|_| parse_packet(decoder)).collect(),
                ),
            }
        } else {
            let bits_number = decoder.read_bits(15).unwrap() as u32;
            let mut subdecoder = SubDecoder::new(decoder, bits_number);
            let mut v = Vec::new();
            while subdecoder.bits > 0 {
                v.push(parse_packet(&mut subdecoder));
            }
            Packet {
                version,
                body: PacketBody::Operator(t, v),
            }
        }
    }
}

fn parse_str(s: &str) -> Packet {
    let mut decoder = Decoder::from(
        s.chars()
            .tuples()
            .map(|(u, l)| ((u.to_digit(16).unwrap() << 4) | l.to_digit(16).unwrap()) as u8),
    );
    parse_packet(&mut decoder)
}

fn add_versions(p: &Packet) -> usize {
    p.version as usize
        + match &p.body {
            PacketBody::Operator(_, v) => v.iter().map(add_versions).sum(),
            _ => 0,
        }
}

fn parse_file(path: &str) -> Packet {
    parse_str(std::fs::read_to_string(path).unwrap().as_str())
}

#[test]
fn task1_example() {
    let result = add_versions(&parse_str("8A004A801A8002F478"));
    println!("D16T1E1 {}", result);
    assert_eq!(result, 16);
    let result = add_versions(&parse_str("620080001611562C8802118E34"));
    println!("D16T1E2 {}", result);
    assert_eq!(result, 12);
    let result = add_versions(&parse_str("C0015000016115A2E0802F182340"));
    println!("D16T1E3 {}", result);
    assert_eq!(result, 23);
    let result = add_versions(&parse_str("A0016C880162017C3686B18A3D4780"));
    println!("D16T1E4 {}", result);
    assert_eq!(result, 31);
}

#[test]
fn task1_puzzle() {
    let result = add_versions(&parse_file("src/day16/input.txt"));
    println!("D16T1P {}", result);
    assert_eq!(result, 821);
}

#[bench]
fn task1_puzzle_bench(b: &mut test::Bencher) {
    b.iter(|| {
        task1_puzzle();
    });
}
