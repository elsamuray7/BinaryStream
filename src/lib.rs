pub mod binary_stream;

pub use binary_stream::{BinaryStream, ByteOrder};

#[cfg(test)]
mod tests {
    use crate::binary_stream::{BinaryStream, ByteOrder};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_binary_stream() {
        let mut bs = BinaryStream::from_byte_order(ByteOrder::LittleEndian);

        let base = 2;

        bs.put_u8(128);
        bs.put_u32(128_000);
        bs.put_i16(-32_000);
        bs.put_i64((base as i64).pow(24));

        bs.flip();

        assert_eq!(bs.get_u8(), 128);
        assert_eq!(bs.get_u32(), 128_000);
        assert_eq!(bs.get_i16(), -32_000);
        assert_eq!(bs.get_i64(), (base as i64).pow(24));

        bs.close();
    }
}