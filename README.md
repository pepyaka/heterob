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
    byte6_last_4_bits: 0b1010,
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
Fallible bytes slice parsing
```rust
use heterob::{Seq, P3, endianness::Be};

// Source is a bytes slice
let data = [0x00u8,0x11,0x22,0x33,0x44,0x55,0b1010_1001].as_slice();

// Target struct
#[derive(Debug, Clone, PartialEq, Eq)]
struct S {
    byte: u8,
    word: u16,
    bytes: [u8;3],
}

// Parse bytes array as integers
let Seq { head: Be((byte, word, bytes)), .. } = P3(data).try_into().unwrap();

// Final structure
let result = S {
    byte,
    word,
    bytes,
};

let sample = S {
    byte: 0x00,
    word: 0x1122,
    bytes: [0x33,0x44,0x55],
};

assert_eq!(sample, result);
```

## Compile time type checking
The idea of compile time checks taken from
[issue comment](https://github.com/nvzqz/static-assertions-rs/issues/40#issuecomment-846228355)
of [static_assertions](https://docs.rs/static_assertions/) crate. Unfortunately it's hard to find the error source using this type of compile time checks. With stabilized constant generics arithmetics it should be much easy.

There are two things checking at compile time

#### 1. Array spliting on multiple arrays

This check asserts that length of input array equal to sum of lengths of output arrays
```compile_fail
# use heterob::T2;
let data = [0u8; 13];
let _ = T2::<[u8;4], [u8;3]>::from(data);
```
Trying to split 13 bytes length array to 2 arrays with lengths 4 + 3 = 7 will throw an error:
```text
error[E0080]: evaluation of `<heterob::T2<[u8; 4], [u8; 3]> as heterob::ParamAndAssociatedConst<13_usize>>::LESS` failed
  --> src/lib.rs:21:25
   |
21 |     const LESS: usize = Self::VALUE - N;
   |                         ^^^^^^^^^^^^^^^ attempt to compute `7_usize - 13_usize`, which would overflow

note: the above error was encountered while instantiating `fn <heterob::T2<[u8; 4], [u8; 3]> as std::convert::From<[u8; 13]>>::from`
 --> src/lib.rs:86:9
  |
6 | let _ = T2::<[u8;4], [u8;3]>::from(data);
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

#### 2. Bit index in arbitrary value

This check asserts that sum of bit indexes is less than value bits count
```compile_fail
# use heterob::{P3, bit_numbering::LsbInto};
let data = 0u16;
let ((),(),()) = P3::<_, 2, 11, 5>(data).lsb_into();
```
Trying to extract bits 12-17 from 16 bits value will throw an error:
```text
error[E0080]: evaluation of `<((), (), ()) as heterob::bit_numbering::FromLsb<heterob::P3<u16, 2_usize, 11_usize, 5_usize>>>::ASSERT_INDEX_IN_BOUNDS` failed
  --> src/bit_numbering.rs:81:43
   |
81 |     const ASSERT_INDEX_IN_BOUNDS: usize = Self::BITS - Self::MAX_BIT_INDEX;
   |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ attempt to compute `16_usize - 18_usize`, which would overflow

note: the above error was encountered while instantiating `fn <((), (), ()) as heterob::bit_numbering::FromLsb<heterob::P3<u16, 2_usize, 11_usize, 5_usize>>>::from_lsb`
  --> src/bit_numbering.rs:97:9
   |
97 |         U::from_lsb(self)
   |         ^^^^^^^^^^^^^^^^^
```
