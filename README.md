This crate provides a library for conversion between **b**ytes/**b**its and **hetero**geneous lists (tuples).

Library features:
- implements compile time type checking
- neither declarative nor procedural macros exports
- mixed endianness from single bytes array

## Examples
Parse complex data structure
```rust
use heterob::{P4, endianness::LeBytesInto, bit_numbering::LsbInto};

// Source is a [u8;6] bytes array
let data = [0x00u8,0x11,0x22,0x33,0x44,0x55,0b1010_1001];

// Target struct
#[derive(Debug, Clone, PartialEq, Eq)]
struct S {
    byte: u8,
    word: Option<u16>,
    bytes: [u8;3],
    is_byte6_bit0: bool,
    byte6_last_4_bits: u8,
}

// Parse bytes array as integers
let P4((byte, word, bytes, byte6)) = data.le_bytes_into();
// Parse last byte as bitfield. Unit type () used as placeholder
let (is_byte6_bit0, (), is_some_word, byte6_last_4_bits) =
    P4::<u8, 1, 2, 1, 4>(byte6).lsb_into();
// `is_some_word` coerce to bool via let statement
let _: bool = is_some_word;

// Final structure
let result = S {
    byte,
    word: is_some_word.then(|| word),
    bytes,
    is_byte6_bit0,
    byte6_last_4_bits,
};

let sample = S {
    byte: 0x00,
    word: Some(0x2211),
    bytes: [0x33,0x44,0x55],
    is_byte6_bit0: true,
    byte6_last_4_bits: 0b0000_1010,
};

assert_eq!(sample, result);
```

Mixed endians
```rust
use heterob::{T3, endianness::{Be, Le}};
#[derive(Debug, Clone, PartialEq, Eq)]
struct S {
    le: u16,
    be: u16,
    bytes: [u8;2]
}

let data = [0x00,0x11,0x22,0x33,0x44,0x55];

let (Le(le),Be(be),bytes) = T3::from(data).into();
let s = S { le, be, bytes };

assert_eq!(S { le: 0x1100, be: 0x2233, bytes: [0x44,0x55] }, s, "{:x}", s.be);
```
