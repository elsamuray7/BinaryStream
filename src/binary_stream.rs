use std::mem;
use crate::binary_stream::ByteOrder::BigEndian;
use std::ops::Range;

const INIT_CAPACITY_HEUR: usize = 1024;

/// Enumeration of byte orders
pub enum ByteOrder {
    BigEndian,
    LittleEndian
}

impl ByteOrder {
    /// Get the mask shift multipliers for a given Type as a Range depending on the ByteOrder
    fn get_mask_shift_multipliers<T: ?Sized>(&self, val: &T) -> Range<usize> {
        match &self {
            BigEndian => mem::size_of_val(val)..0,
            _ => 0..mem::size_of_val(val)
        }
    }
}

/// A stream to hold binary data
pub struct BinaryStream {
    stream: Vec<u8>,
    pub bo: ByteOrder
}

impl BinaryStream {
    /// Creates a new BinaryStream with initial capacity init_capacity and ByteOrder bo
    pub fn with_init_capacity(init_capacity: usize, bo: ByteOrder) -> BinaryStream {
        BinaryStream {
            stream: Vec::with_capacity(init_capacity),
            bo
        }
    }

    /// Creates a new BinaryStream with default initial capacity and ByteOrder bo
    pub fn from_byte_order(bo: ByteOrder) -> BinaryStream {
        BinaryStream::with_init_capacity(INIT_CAPACITY_HEUR, bo)
    }

    /// Put an u8 value at the end of the BinaryStream
    pub fn put_u8(&mut self, val: u8) {
        self.stream.push(val);
    }

    /// Put an u16 value at the end of the BinaryStream
    pub fn put_u16(&mut self, val: u16) {
        let mask: u16 = 0xFF;
        for i in self.bo.get_mask_shift_multipliers(&val) {
            let to_shift = i*mem::size_of::<u8>()*8;
            let to_and = mask << to_shift;
            let to_put = ((val & to_and) >> to_shift) as u8;
            self.put_u8(to_put);
        }
    }

    /// Put an u32 value at the end of the BinaryStream
    pub fn put_u32(&mut self, val: u32) {
        let mask: u32 = 0xFF;
        for i in self.bo.get_mask_shift_multipliers(&val) {
            let to_shift = i*mem::size_of::<u8>()*8;
            let to_and = mask << to_shift;
            let to_put = ((val & to_and) >> to_shift) as u8;
            self.put_u8(to_put);
        }
    }

    /// Put an u64 value at the end of the BinaryStream
    pub fn put_u64(&mut self, val: u64) {
        let mask: u64 = 0xFF;
        for i in self.bo.get_mask_shift_multipliers(&val) {
            let to_shift = i*mem::size_of::<u8>()*8;
            let to_and = mask << to_shift;
            let to_put = ((val & to_and) >> to_shift) as u8;
            self.put_u8(to_put);
        }
    }

    /// Put an i8 value at the end of the BinaryStream
    pub fn put_i8(&mut self, val: i8) {
        self.put_u8(val as u8);
    }

    /// Put an i16 value at the end of the BinaryStream
    pub fn put_i16(&mut self, val: i16) {
        self.put_u16(val as u16);
    }

    /// Put an i32 value at the end of the BinaryStream
    pub fn put_i32(&mut self, val: i32) {
        self.put_u32(val as u32);
    }

    /// Put an i64 value at the end of the BinaryStream
    pub fn put_i64(&mut self, val: i64) {
        self.put_u64(val as u64);
    }

    /// After being done writing to the BinaryStream, prepare it for subsequent read operations
    pub fn flip(&mut self) {
        self.stream.reverse();
    }

    /// Get an u8 from the beginning of the BinaryStream
    pub fn get_u8(&mut self) -> u8 {
        self.stream.pop().expect("BinaryStream is empty.")
    }

    /// Get an u16 from the beginning of the BinaryStream
    pub fn get_u16(&mut self) -> u16 {
        let mut val: u16 = 0;
        for i in self.bo.get_mask_shift_multipliers(&val) {
            let to_or = (self.get_u8() as u16) << i*mem::size_of::<u8>()*8;
            val |= to_or;
        }
        val
    }

    /// Get an u32 from the beginning of the BinaryStream
    pub fn get_u32(&mut self) -> u32 {
        let mut val: u32 = 0;
        for i in self.bo.get_mask_shift_multipliers(&val) {
            let to_or = (self.get_u8() as u32) << i*mem::size_of::<u8>()*8;
            val |= to_or;
        }
        val
    }

    /// Get an u64 from the beginning of the BinaryStream
    pub fn get_u64(&mut self) -> u64 {
        let mut val: u64 = 0;
        for i in self.bo.get_mask_shift_multipliers(&val) {
            let to_or = (self.get_u8() as u64) << i*mem::size_of::<u8>()*8;
            val |= to_or;
        }
        val
    }

    /// Get an i8 from the beginning of the BinaryStream
    pub fn get_i8(&mut self) -> i8 {
        self.get_u8() as i8
    }

    /// Get an i16 from the beginning of the BinaryStream
    pub fn get_i16(&mut self) -> i16 {
        self.get_u16() as i16
    }

    /// Get an i32 from the beginning of the BinaryStream
    pub fn get_i32(&mut self) -> i32 {
        self.get_u32() as i32
    }

    /// Get an i64 from the beginning of the BinaryStream
    pub fn get_i64(&mut self) -> i64 {
        self.get_u64() as i64
    }

    /// Close the BinaryStream
    pub fn close(self) {}
}