use std::io::{self, Write};

// derive from https://stackoverflow.com/questions/4351371/c-performance-challenge-integer-to-stdstring-conversion
const DIGIT_PAIRS: &[u8; 200] = b"00010203040506070809\
                                  10111213141516171819\
                                  20212223242526272829\
                                  30313233343536373839\
                                  40414243444546474849\
                                  50515253545556575859\
                                  60616263646566676869\
                                  70717273747576777879\
                                  80818283848586878889\
                                  90919293949596979899";

pub fn write_u32<W: Write>(n: u32, writer: &mut W) -> io::Result<()> {
    let mut val = n;
    let mut buffer = [0u8; 11]; // Enough for "4294967295"
    let mut index = buffer.len();

    while val >= 100 {
        index -= 2;
        let pos = (val % 100) as usize;
        val /= 100;
        buffer[index..index + 2].copy_from_slice(&DIGIT_PAIRS[2 * pos..2 * pos + 2]);
    }

    if val < 10 {
        index -= 1;
        buffer[index] = b'0' + val as u8;
    } else {
        index -= 2;
        buffer[index..index + 2]
            .copy_from_slice(&DIGIT_PAIRS[2 * val as usize..2 * val as usize + 2]);
    }

    writer.write_all(&buffer[index..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_u32() {
        let mut buf = Vec::new();
        write_u32(0, &mut buf).unwrap();
        assert_eq!(buf, b"0");

        let mut buf = Vec::new();
        write_u32(1, &mut buf).unwrap();
        assert_eq!(buf, b"1");

        let mut buf = Vec::new();
        write_u32(10, &mut buf).unwrap();
        assert_eq!(buf, b"10");

        let mut buf = Vec::new();
        write_u32(100, &mut buf).unwrap();
        assert_eq!(buf, b"100");

        let mut buf = Vec::new();
        write_u32(1000, &mut buf).unwrap();
        assert_eq!(buf, b"1000");

        let mut buf = Vec::new();
        write_u32(10000, &mut buf).unwrap();
        assert_eq!(buf, b"10000");

        let mut buf = Vec::new();
        write_u32(100000, &mut buf).unwrap();
        assert_eq!(buf, b"100000");

        let mut buf = Vec::new();
        write_u32(1000000, &mut buf).unwrap();
        assert_eq!(buf, b"1000000");

        let mut buf = Vec::new();
        write_u32(10000000, &mut buf).unwrap();
        assert_eq!(buf, b"10000000");

        let mut buf = Vec::new();
        write_u32(100000000, &mut buf).unwrap();
        assert_eq!(buf, b"100000000");

        let mut buf = Vec::new();
        write_u32(1000000000, &mut buf).unwrap();
        assert_eq!(buf, b"1000000000");

        let mut buf = Vec::new();
        write_u32(4294967295, &mut buf).unwrap();
        assert_eq!(buf, b"4294967295");
    }

    #[test]
    fn bench_write_u32() {
        let mut buf = Vec::new();
        for _ in 0..1000000000 {
            buf.clear();
            write_u32(4294967295u32, &mut buf).unwrap();
        }
    }

    #[test]
    fn bench_fmt_u32() {
        let mut buf = Vec::new();
        for _ in 0..1000000000 {
            buf.clear();
            std::write!(buf, "{}", 4294967295u32).unwrap();
        }
    }
}
