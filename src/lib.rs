#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(test)]
#[macro_use]
extern crate std;

pub mod common;
pub use common::*;

pub mod bit_numbering;
pub mod endianness;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_array() {
        let chars = ['a', 'b', 'b'];
        assert_eq!(T2(['a'], ['b', 'b']), T2::from(chars));

        let bytes = [1u8, 2, 2, 3, 3, 3, 3];
        assert_eq!(T3([1], [2, 2], [3, 3, 3, 3]), T3::from(bytes));
    }

    #[test]
    fn tuple_elements_from() {
        assert_eq!((97u128, 98usize), T2(97u8, 98u16).into());
        assert_eq!(('a', 'b', 'c'), T3(97, 98, 99).into());
    }

    #[test]
    fn tuple_elements_into() {
        assert_eq!(T2(97u32, 98i32), (97u8, 98u16).into());
        assert_eq!(T3('a', 'b', 'c'), (97, 98, 99).into());
    }
}
