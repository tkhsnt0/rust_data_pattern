// endian モジュール
use std::convert::From;

#[derive(Debug, Clone, Copy)]
pub struct LittleEndianU16 {
    byte_array: [u8; 2],
}

impl LittleEndianU16 {
    pub fn new(data: [u8; 2]) -> Self {
        LittleEndianU16 {
            byte_array: [data[0], data[1]],
        }
    }

    pub fn get_value(&self) -> u16 {
        let mut data_u16: u16;
        data_u16 = (self.byte_array[0] as u16) << 8;
        data_u16 |= self.byte_array[1] as u16;
        data_u16
    }
}
impl From<BigEndianU16> for LittleEndianU16 {
    fn from(big: BigEndianU16) -> Self {
        Self {
            byte_array: [big.byte_array[1], big.byte_array[0]],
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BigEndianU16 {
    byte_array: [u8; 2],
}

impl BigEndianU16 {
    pub fn new(data: [u8; 2]) -> Self {
        BigEndianU16 {
            byte_array: [data[1], data[0]],
        }
    }
    pub fn get_value(&self) -> u16 {
        let mut data_u16: u16;
        data_u16 = (self.byte_array[0] as u16) << 8;
        data_u16 |= self.byte_array[1] as u16;
        data_u16
    }
}
impl From<LittleEndianU16> for BigEndianU16 {
    fn from(little: LittleEndianU16) -> Self {
        Self {
            byte_array: [little.byte_array[1], little.byte_array[0]],
        }
    }
}

#[cfg(test)]
mod tests {
    // 親モジュール(=このファイルのmodが無い部分)のインポート
    use super::*;
    #[test]
    fn cmp_big_and_little() {
        // byte_array
        let byte_array_u8: [u8; 2] = [0x01, 0x02];
        // endian
        let big_endian = BigEndianU16::new(byte_array_u8);
        let little_endian = LittleEndianU16::new(byte_array_u8);
        assert_ne!(big_endian.get_value(), little_endian.get_value());
    }
    #[test]
    fn cmp_little_endian() {
        // byte_array
        let byte_array_u8: [u8; 2] = [0x01, 0x02];
        // endian
        let big_endian = BigEndianU16::new(byte_array_u8);
        let little_endian = LittleEndianU16::new(byte_array_u8);
        // little endian
        let a = little_endian.get_value();
        let b = LittleEndianU16::from(big_endian).get_value(); //from
        let c = Into::<LittleEndianU16>::into(big_endian).get_value(); //into
        let d: LittleEndianU16 = big_endian.into(); //into2
        assert_eq!(a, b);
        assert_eq!(a, c);
        assert_eq!(a, d.get_value());
    }
    #[test]
    fn cmp_big_endian() {
        // byte_array
        let byte_array_u8: [u8; 2] = [0x01, 0x02];
        // endian
        let big_endian = BigEndianU16::new(byte_array_u8);
        let little_endian = LittleEndianU16::new(byte_array_u8);
        // big endian
        let a = big_endian.get_value();
        let b = BigEndianU16::from(little_endian).get_value(); //from
        let c = Into::<BigEndianU16>::into(little_endian).get_value(); //into
        let d: BigEndianU16 = little_endian.into(); //into2
        assert_eq!(a, b);
        assert_eq!(a, c);
        assert_eq!(a, d.get_value());
    }
}
