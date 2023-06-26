use core::fmt;
use std::{mem, error::Error};

pub struct Deserializer {
    input: Vec<u8>,
    offset: usize,
}

#[derive(Debug)]
pub struct DeserializeError {
    cause: &'static str,
}

impl<'a> DeserializeError<'a> {
    fn new(cause: &'a str) -> Self {
        Self {
            cause: cause,
        }
    }
}

impl<'a> fmt::Display for DeserializeError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "deserialization error: {}", self.cause)
    }
}

impl<'a> Error for DeserializeError<'a> {}

struct TestStruct {
    a: u32,
    b: f64,
}

mod read {
    use crate::deserialize::DeserializeError;

    macro_rules! fn_read_primitive {
        ($name: ident, $num_type: ty, $size: literal) => {
            pub fn $name<'a>(input: &[u8], mut offset: usize) -> Result<($num_type, usize), DeserializeError> {
                let mut offset = offset;
                Ok((<$num_type>::from_le_bytes(*get_byte_slice::<$size>(input, &mut offset)?), offset + $size))
            }
        };
    }

    fn get_byte_slice<'a, const SIZE: usize>(input: &[u8], offset: &mut usize) -> Result<&'a [u8; SIZE], DeserializeError> {
        match input[*offset..*offset + SIZE].try_into() {
            Ok(bytes) => {
                *offset += SIZE;
                Ok(bytes)
            }, 
            Err(_) => Err(DeserializeError::new("malformed data"))
        }
    }

    fn_read_primitive!(read_u8, u8, 1);
    fn_read_primitive!(read_u16, u16, 2);
    fn_read_primitive!(read_u32, u32, 4);
    fn_read_primitive!(read_u64, u64, 8);

    fn_read_primitive!(read_i8, i8, 1);
    fn_read_primitive!(read_i16, i16, 2);
    fn_read_primitive!(read_i32, i32, 4);
    fn_read_primitive!(read_i64, i64, 8);

    fn_read_primitive!(read_f32, f32, 4);
    fn_read_primitive!(read_f64, f64, 8);

    macro_rules! read {
        ($func: path, $offset_var: ident,) => {
            
        };
    }
}

impl Deserialize for TestStruct {
    fn deserialize<'a>(input: &[u8], offset: usize) -> Result<Self, DeserializeError> {
        let mut o = 0;
        Ok(Self { 
            a: {
                let value = read::read_u32(input, offset)?;                
                o = value.1;
                value.0
            },
            b: read::read_f64(input, offset)?.0,
        })
    }
}

type RawData<'a, const N: usize> = &'a [u8; N];

pub trait Deserialize
where
    Self: Sized
{
    fn deserialize<'a>(input: &[u8], offset: usize) -> Result<Self, DeserializeError>;
}

macro_rules! deserializer_fn_read_primitive {
    ($name: ident, $num_type: ty, $size: literal) => {
        pub fn $name(&mut self) -> Result<$num_type, DeserializeError> {
            Ok(read::$name(&self.input, &mut self.offset)?)
        }
    };
}

impl Deserializer {
    pub fn from_file(path: &str) -> std::io::Result<Self> {
        Ok(
            Self {
                input: std::fs::read(path)?,
                offset: 0,
            }
        )
    }

    fn get_byte_slice<const SIZE: usize>(&mut self) -> Result<&[u8; SIZE], DeserializeError> {
        match self.input[self.offset..self.offset + SIZE].try_into() {
            Ok(bytes) => {
                self.offset += SIZE;
                Ok(bytes)
            }, 
            Err(_) => Err(DeserializeError::new("malformed data"))
        }
    }

    deserializer_fn_read_primitive!(read_u8, u8, 1);
    deserializer_fn_read_primitive!(read_u16, u16, 2);
    deserializer_fn_read_primitive!(read_u32, u32, 4);
    deserializer_fn_read_primitive!(read_u64, u64, 8);

    deserializer_fn_read_primitive!(read_i8, i8, 1);
    deserializer_fn_read_primitive!(read_i16, i16, 2);
    deserializer_fn_read_primitive!(read_i32, i32, 4);
    deserializer_fn_read_primitive!(read_i64, i64, 8);

    deserializer_fn_read_primitive!(read_f32, f32, 4);
    deserializer_fn_read_primitive!(read_f64, f64, 8);

    pub fn read_struct<T: Deserialize>(&mut self) -> Result<T, DeserializeError> {
        T::deserialize(&self.input, &mut self.offset)
    }
        
}