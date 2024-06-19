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
Compile time checks implemented by [inline const expressions](https://doc.rust-lang.org/nightly/reference/expressions/block-expr.html#const-blocks) + [assert!](https://doc.rust-lang.org/core/macro.assert.html). Unfortunately it's hard to find the error source using this type of compile time checks. With stabilized constant generics arithmetics it should be much easy.

There are three things checked at compile time

#### 1. Array spliting on multiple arrays

This check asserts that length of input array equal to sum of lengths of output arrays
```compile_fail
# use heterob::T2;
let data = [0u8; 13];
let _ = T2::<[u8;4], [u8;3]>::from(data);
```
Trying to split 13 bytes length array to 2 arrays with lengths 4 + 3 = 7 will throw an error:
```text
error[E0080]: evaluation of `<heterob::T2<[u8; 4], [u8; 3]> as std::convert::From<[u8; 13]>>::from::{constant#1}` failed
   --> heterob/src/common.rs:402:1
    |
402 | common_impl!(2; A,B);
    | ^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'The sum AN + BN should be equal to N', heterob/src/
common.rs:402:1
    |
    = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `common_impl` (in Nightly builds, r
un with -Z macro-backtrace for more info)

note: erroneous constant encountered
   --> heterob/src/common.rs:402:1
    |
402 | common_impl!(2; A,B);
    | ^^^^^^^^^^^^^^^^^^^^
    |
    = note: this note originates in the macro `common_impl` (in Nightly builds, run with -Z macro-backtrace for more info)

note: the above error was encountered while instantiating `fn <heterob::T2<[u8; 4], [u8; 3]> as std::convert::From<[u8; 13]>>::from`
 --> src/lib.rs:9:13
  |
9 |     let _ = T2::<[u8; 4], [u8; 3]>::from(data);
  |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
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
err(https://doc.rust-lang.org/core/macro.assert.html): evaluation of `<((), (), ()) as heterob::bit_numbering::FromLsb<heterob::P3<u16, 2, 11, 5>>>::from_lsb::{constant#0}` failed
   --> heterob/src/bit_numbering.rs:227:1
    |
227 | bit_numbering_impl!(3: A,B,C);
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'The bits number in TY in P3<TY, ...> is less than sum AN + BN + CN', heterob/src/t(https://doc.rust-lang.org/core/macro.assert.html)ring.rs:227:1
    |
    = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `bit_numbering_impl` (in Nightly builds, run with -Z macro-backtrace for more info)

note: erroneous constant encountered
   --> heterob/src/bit_numbering.rs:227:1
    |
227 | bit_numbering_impl!(3: A,B,C);
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: this note originates in the macro `bit_numbering_impl` (in Nightly builds, run with -Z macro-backtrace for more info)

note: the above error was encountered while instantiating `fn <((), (), ()) as heterob::bit_numbering::FromLsb<heterob::P3<u16, 2, 11, 5>>>::from_lsb`
   --> heterob/src/bit_numbering.rs:136:9
    |
136 |         U::from_lsb(self)
    |         ^^^^^^^^^^^^^^^^^
```

#### 3. Endianness conversions

This checks that number of bytes are equal in source and in result.
```compile_fail
# use heterob::endianness::FromBeBytes;
let _: [u32; 2] = FromBeBytes::from_be_bytes([0u8; 7]);
```
Trying to convert 7 bytes array into 8 bytes type (u32 * 2) will thow an error like:
```text
error[E0080]: evaluation of `<[u32; 2] as heterob::endianness::FromBeBytes<7>>::from_be_bytes::{constant#1}` failed
   --> heterob/src/endianness.rs:352:1
    |
352 | endianness_integers!(u16, u32, u64, u128, usize);
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'The size of [u32; M] and [u8; N] are different', heterob/src/endianness.rs:352:1
    |
    = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `endianness_integers` (in Nightly builds, run with -Z macro-backtrace for more info)

note: erroneous constant encountered
   --> heterob/src/endianness.rs:352:1
    |
352 | endianness_integers!(u16, u32, u64, u128, usize);
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: this note originates in the macro `endianness_integers` (in Nightly builds, run with -Z macro-backtrace for more info)

note: the above error was encountered while instantiating `fn <[u32; 2] as heterob::endianness::FromBeBytes<7>>::from_be_bytes`
 --> src/lib.rs:8:23
  |
8 |     let _: [u32; 2] = FromBeBytes::from_be_bytes([0u8; 7]);
```
