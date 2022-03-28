/*!
Bit conversion according to bit numbering

Bit can be converted in 2 ways:
- Using [FromLsb] / [FromMsb] trait methods `.lsb_into()` / `.msb_into()`
- Using std [From] trait for wrapper types [Lsb] / [Msb]

## Examples
It is obligatory to set sizes of resulting values

```rust
# use heterob::{P3, bit_numbering::*};
let data = 0b1111_0000_1100_1010;

//         Value sizes: ⇓  ⇓  ⇓
let (a,b,c) = P3::<u16, 7, 1, 8>(data).lsb_into();

let sample = (0b100_1010u64, true, 0xF0u8);

assert_eq!(sample, (a,b,c));
```

You could ommit source type annotation
```rust
# use heterob::{P3, bit_numbering::*};
let data: u16 = 0b1111_0000_1100_1010;

let Lsb((a,b,c)) = P3::<_, 7, 1, 8>(data).into();

let sample = (0b100_1010u64, true, 0xF0u8);

assert_eq!(sample, (a,b,c));
```

Ommiting bits using unit type [()](unit)
```rust
# use heterob::{P3, bit_numbering::*};
let data: u16 = 0b1111_0000_0000_1011;

let Lsb(( a, (), b )) = P3::<_, 4, 11, 1>(data).into();

let sample = (0b1011u8, true);

assert_eq!(sample, (a, b));
```

Explicit type coercion
```rust
# use heterob::{P3, bit_numbering::*};
let data: u16 = 0b1011_0000_0000_1011;

let (a,b,c) = P3::<_, 4, 10, 2>(data).msb_into();

let _: (u16,u32,u64) = (a,b,c);

let sample = (11, 2, 3);

assert_eq!(sample, (a, b, c));
```
*/


use core::mem::size_of;

use funty::Integral;

use super::*;



/// Type wrapper for LSB 0 bit numbering data
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lsb<T>(pub T);

/// LSB 0 bit numbering data to value conversion
///
/// It is the reciprocal of [LsbInto].
pub trait FromLsb<T: Sized> {
    const BITS: usize = size_of::<T>() * 8;
    const MAX_BIT_INDEX: usize = Self::BITS - 1;
    const ASSERT_INDEX_IN_BOUNDS: usize = Self::BITS - Self::MAX_BIT_INDEX;
    fn from_lsb(_: T) -> Self;
}

/// LSB 0 bit numbering data to value conversion that consumes the input bytes
///
/// The opposite of [FromLsb].
/// One should avoid implementing [LsbInto] and implement [FromLsb] instead.
pub trait LsbInto<T>: Sized {
    fn lsb_into(self) -> T;
}

/// Implementing [FromLsb] automatically provides one with an implementation of [LsbInto]
/// thanks to this blanket implementation.
impl<T, U: FromLsb<T>> LsbInto<U> for T {
    fn lsb_into(self) -> U {
        U::from_lsb(self)
    }
}

/**
Split integer at some point according to LSB 0 bit numbering

```rust
# use heterob::bit_numbering::lsb_split;
const U32: u32 = 0b1111_1111_0101_1010_1100_001__1_1000_0001;
let result = lsb_split::<_, 9>(U32);
assert_eq!((0b1_1000_0001, 0b1111_1111_0101_1010_1100_001), result);
```
*/
pub fn lsb_split<T: Integral, const N: usize>(data: T) -> (T, T) {
    let mask = !(T::MAX << N);
    (data & mask, data >> N)
}



/// Type wrapper for MSB 0 bit numbering data
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Msb<T>(pub T);

/// MSB 0 bit numbering data to value conversion
///
/// It is the reciprocal of [MsbInto].
pub trait FromMsb<T: Sized> {
    const BITS: usize = size_of::<T>() * 8;
    const MAX_BIT_INDEX: usize = Self::BITS - 1;
    const ASSERT_INDEX_IN_BOUNDS: usize = Self::BITS - Self::MAX_BIT_INDEX;
    fn from_msb(_: T) -> Self;
}

/// MSB 0 bit numbering data to value conversion that consumes the input bytes
///
/// The opposite of [FromMsb].
/// One should avoid implementing [MsbInto] and implement [FromMsb] instead.
pub trait MsbInto<T>: Sized {
    fn msb_into(self) -> T;
}

/// Implementing [FromMsb] automatically provides one with an implementation of [MsbInto]
/// thanks to this blanket implementation.
impl<T, U: FromMsb<T>> MsbInto<U> for T {
    fn msb_into(self) -> U {
        U::from_msb(self)
    }
}

/**
Split integer at some point according to MSB 0 bit numbering

```rust
# use heterob::bit_numbering::msb_split;
const U32: u32 = 0b1111_1111_0__101_1010_1100_0011_1000_0001;
let (a,b) = msb_split::<_, 9>(U32);
assert_eq!((0b1111_1111_0,0b101_1010_1100_0011_1000_0001 << 9), (a,b), "{a:b},{b:b}");
```
*/
pub fn msb_split<T: Integral, const N: usize>(data: T) -> (T, T) {
    let mask = T::MAX >> N;
    ((data & !mask) >> (T::BITS as usize - N), (data & mask) << N)
}



// // Implemented in [bit_numbering_alphabet]
// impl<TY, A,B,C, const AN: usize, const BN: usize, const CN: usize> FromLsb<P3<TY,AN,BN,CN>> for (A,B,C)
// where
//     TY: Integral + AsPrimitive<A> + AsPrimitive<B> + AsPrimitive<C>,
// {
//     const BITS: usize = TY::BITS as usize;
//     const MAX_BIT_INDEX: usize = AN + BN + CN;
//     fn from_lsb(P3(_data): P3<TY,AN,BN,CN>) -> Self {
//         #![allow(path_statements)]
//         <Self as FromLsb<P3<TY, AN,BN,CN>>>::ASSERT_INDEX_IN_BOUNDS;

//         let (a, _data) = lsb_split::<_, AN>(_data);
//         let (b, _data) = lsb_split::<_, BN>(_data);
//         let (c, _data) = lsb_split::<_, CN>(_data);
//         (a.as_primitive(), b.as_primitive(), c.as_primitive())
//     }
// }

// // Implemented in [bit_numbering_alphabet]
// impl<T, U, const AN: usize, const BN: usize, const CN: usize> From<P3<T, AN, BN, CN>> for Lsb<U>
// where
//     U: FromLsb<P3<T,AN,BN,CN>>,
// {
//     fn from(data: P3<T,AN,BN,CN>) -> Self {
//         Self(data.lsb_into())
//     }
// }

bit_numbering_alphabet!(1: A);
bit_numbering_alphabet!(2: A,B);
bit_numbering_alphabet!(3: A,B,C);
bit_numbering_alphabet!(4: A,B,C,D);
bit_numbering_alphabet!(5: A,B,C,D,E);
bit_numbering_alphabet!(6: A,B,C,D,E,F);
bit_numbering_alphabet!(7: A,B,C,D,E,F,G);
bit_numbering_alphabet!(8: A,B,C,D,E,F,G,H);
bit_numbering_alphabet!(9: A,B,C,D,E,F,G,H,I);
bit_numbering_alphabet!(10: A,B,C,D,E,F,G,H,I,J);
bit_numbering_alphabet!(11: A,B,C,D,E,F,G,H,I,J,K);
bit_numbering_alphabet!(12: A,B,C,D,E,F,G,H,I,J,K,L);
bit_numbering_alphabet!(13: A,B,C,D,E,F,G,H,I,J,K,L,M);
bit_numbering_alphabet!(14: A,B,C,D,E,F,G,H,I,J,K,L,M,N);
bit_numbering_alphabet!(15: A,B,C,D,E,F,G,H,I,J,K,L,M,N,O);
bit_numbering_alphabet!(16: A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P);
bit_numbering_alphabet!(17: A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q);
bit_numbering_alphabet!(18: A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R);
bit_numbering_alphabet!(19: A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S);
bit_numbering_alphabet!(20: A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T);
bit_numbering_alphabet!(21: A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U);
bit_numbering_alphabet!(22: A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V);
bit_numbering_alphabet!(23: A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W);
bit_numbering_alphabet!(24: A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X);
bit_numbering_alphabet!(25: A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y);
bit_numbering_alphabet!(26: A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y,Z);



#[cfg(test)]
mod tests {
    use super::*;

    const U32: u32 = 0b1111_1111_0101_1010_1100_0011_1000_0001;

    #[test]
    fn trait_lsb_into_tuple() {
        let (a,b,c,()) = P4::<_, 15, 1, 2, 14>(U32).lsb_into();
        let _: (u16,bool,u8) = (a,b,c);
        assert_eq!((0b100_0011_1000_0001,true,0b10), (a,b,c), "{a:b}, {b}, {c:b}");
    }

    #[test]
    fn trait_msb_into_tuple() {
        let (a,b,c) = P3::<_, 15, 1, 2>(U32).msb_into();
        let _: (u16,bool,u8) = (a,b,c);
        assert_eq!((0b1111_1111_0101_101,false,0b11), (a,b,c), "{a:b}, {b}, {c:b}");
    }

    #[test]
    fn struct_lsb_into_tuple() {
        let Lsb((a,b,c,())) = P4::<_, 15, 1, 2, 14>(U32).into();
        let _: (u16,bool,u8) = (a,b,c);
        assert_eq!((0b100_0011_1000_0001,true,0b10), (a,b,c), "{a:b}, {b}, {c:b}");
    }

    #[test]
    fn struct_msb_into_tuple() {
        let Msb((a,b,c)) = P3::<_, 15, 1, 2>(U32).into();
        let _: (u16,bool,u8) = (a,b,c);
        assert_eq!((0b1111_1111_0101_101,false,0b11), (a,b,c), "{a:b}, {b}, {c:b}");
    }
}
