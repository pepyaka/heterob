/*!
Bytes conversions according to endianness

## Conversions implemented within library

- Integer conversion
```rust
# use heterob::endianness::*;
// Using LeBytesInto trait
let word: u16 = [0x11,0x22].le_bytes_into();
assert_eq!(0x2211, word);

// Using Le wrapper
let Le(word) = [0x11,0x22].into();
assert_eq!(0x2211u16, word);
```

- Tupled integers conversion
```rust
# use heterob::{P3, endianness::*};
// Using LeBytesInto trait
let P3((byte, word, dword)) = [0x00,0x11,0x22,0x33,0x44,0x55,0x66].le_bytes_into();
assert_eq!((0x00u8,0x2211u16,0x66554433u32), (byte, word, dword));

// Using Le wrapper for each value
let (Le(byte), Le(word), Le(dword)) = P3([0x00,0x11,0x22,0x33,0x44,0x55,0x66]).into();
assert_eq!((0x00u8,0x2211u16,0x66554433u32), (byte, word, dword));

// Using Le wrapper for all values at once
let Le((byte, word, dword)) = P3([0x00,0x11,0x22,0x33,0x44,0x55,0x66]).into();
assert_eq!((0x00u8,0x2211u16,0x66554433u32), (byte, word, dword));
```

- Array of integers
```rust
# use heterob::endianness::*;
// Using LeBytesInto trait
let array: [u16;3] = [0x00,0x11,0x22,0x33,0x44,0x55].le_bytes_into();
assert_eq!([0x1100,0x3322,0x5544], array);

// Using Le wrapper
let Le(array) = [0x00,0x11,0x22,0x33,0x44,0x55].into();
let _: [u16;3] = array; // let statements type coercioon
assert_eq!([0x1100,0x3322,0x5544], array);
```

## More than 26 entries conversion

Library limited max to 26 types list conversion. There are several workarounds

- Pre split large bytes array
```rust
# use heterob::{T2, P2, P3, endianness::*};
let data = [
    0x00, 0x11,0x11,0x11,0x11,
    0x22, 0x33,0x33, 0x44,0x44,
];

// Split data into 2 arrays
let T2(head, tail): T2<[u8;5], [u8;5]> = data.into();
// Convert first half
let P2((v0, v1)) = head.le_bytes_into();
// Convert second half
let Le((v2, v3, v4)) = P3(tail).into();

let sample = (0x00u8, 0x11111111u32, 0x22u8, 0x3333u16, 0x4444u16);

assert_eq!(sample, (v0, v1, v2, v3, v4));
```

- Complicated type annotation
```rust
# use heterob::{P3, endianness::*};
let data = [
    0x00, 0x11,0x11,0x11,0x11,
    0x22, 0x33,0x33, 0x44,0x44,
];

let P3((v0, v1, P3((v2, v3, v4)))):
    P3<(u8,  _, P3<(u8,  _,  _), 1, 2, 2>), 1, 4, 5>
    = data.le_bytes_into();

let sample = (0x00, 0x11111111u32, 0x22, 0x3333u16, 0x4444u16);

assert_eq!(sample, (v0, v1, v2, v3, v4));
```

- Using endianness independent bytes to bytes conversion feature
```rust
# use heterob::{T3, P3, endianness::*};
let data = [
    0x00, 0x11,0x11,0x11,0x11,
    0x22, 0x33,0x33, 0x44,0x44,
];

// Convert data to 2 values and tail bytes
let Le((v0, v1, tail)) = T3::from(data).into();
// Convert tail bytes to values
let Le((v2, v3, v4)) = P3::<[u8; 5], 1, 2, 2>(tail).into();

let sample = (0x00u8, 0x11111111u32, 0x22, 0x3333, 0x4444);

assert_eq!(sample, (v0, v1, v2, v3, v4));
```

*/
use core::{mem::size_of, usize};
use paste::paste;
use core::array::TryFromSliceError;

use super::*;

/// Little endian bytes to value conversion
///
/// It is the reciprocal of [LeBytesInto].
pub trait FromLeBytes<const N: usize>: Sized {
    const SIZE: usize = size_of::<Self>();
    const ASSERT_SELF_SIZE: (usize, usize) = (Self::SIZE - N, N - Self::SIZE);
    fn from_le_bytes(bytes: [u8;N]) -> Self;
}

/// Little endian to value conversion that consumes the input bytes
///
/// The opposite of [FromLeBytes].
/// One should avoid implementing [LeBytesInto] and implement [FromLeBytes] instead.
pub trait LeBytesInto<T> {
    fn le_bytes_into(self) -> T;
}

/// Implementing [FromLeBytes] automatically provides one with an implementation of [LeBytesInto]
/// thanks to this blanket implementation.
impl<T, const N: usize> LeBytesInto<T> for [u8;N]
where
    T: FromLeBytes<N>,
{
    fn le_bytes_into(self) -> T {
        T::from_le_bytes(self)
    }
}

/// One byte array conversion
impl FromLeBytes<1> for u8 {
    fn from_le_bytes(bytes: [u8;1]) -> Self {
        bytes[0]
    }
}

/// Bytes to bytes (no)conversion
impl<const N: usize> FromLeBytes<N> for [u8;N] {
    fn from_le_bytes(bytes: [u8;N]) -> Self {
        bytes
    }
}

/// Type wrapper for little endian bytes value
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Le<T>(pub T);

/// Any type that implemented [FromLeBytes] can be converted to [Le] wrapper
impl<T: FromLeBytes<N>, const N: usize> From<[u8;N]> for Le<T> {
    fn from(data: [u8;N]) -> Self {
        Le(data.le_bytes_into())
    }
}

/// Little endian bytes slice to value conversion that may fail
///
/// It is the reciprocal of [LeBytesTryInto].
pub trait TryFromLeBytes<const N: usize>: Sized + FromLeBytes<N> {
    fn try_from_le_bytes(slice: &[u8]) -> Result<Seq<Self, &[u8]>, TryFromSliceError>;
}

impl<T: FromLeBytes<N>, const N: usize> TryFromLeBytes<N> for T {
    fn try_from_le_bytes(slice: &[u8]) -> Result<Seq<Self, &[u8]>, TryFromSliceError> {
        let (head, tail) = slice.split_at(slice.len().min(size_of::<Self>()));
        let head = head.try_into().map(FromLeBytes::from_le_bytes)?;
        Ok(Seq { head, tail })
    }
}

/// Big endian bytes slice to value conversion that may fail
///
/// It is the reciprocal of [BeBytesTryInto].
pub trait TryFromBeBytes<const N: usize>: Sized + FromBeBytes<N> {
    fn try_from_be_bytes(slice: &[u8]) -> Result<Seq<Self, &[u8]>, TryFromSliceError>;
}

impl<T: FromBeBytes<N>, const N: usize> TryFromBeBytes<N> for T {
    fn try_from_be_bytes(slice: &[u8]) -> Result<Seq<Self, &[u8]>, TryFromSliceError> {
        let (head, tail) = slice.split_at(slice.len().min(size_of::<Self>()));
        let head = head.try_into().map(FromBeBytes::from_be_bytes)?;
        Ok(Seq { head, tail })
    }
}

/// Little endian bytes slice to value attempted conversion
///
/// The opposite of [TryFromLeBytes].
/// One should avoid implementing [LeBytesTryInto] and implement [TryFromLeBytes] instead.
pub trait LeBytesTryInto<'a, T, const N: usize> {
    /// Performs the conversion.
    fn le_bytes_try_into(self) -> Result<Seq<T, &'a [u8]>, TryFromSliceError>;
}

impl<'a, T, const N: usize> LeBytesTryInto<'a, T, N> for &'a [u8]
where
    T: TryFromLeBytes<N>,
{
    fn le_bytes_try_into(self) -> Result<Seq<T, &'a [u8]>, TryFromSliceError> {
        T::try_from_le_bytes(self)
    }
}

/// Big endian bytes slice to value attempted conversion
///
/// The opposite of [TryFromBeBytes].
/// One should avoid implementing [BeBytesTryInto] and implement [TryFromBeBytes] instead.
pub trait BeBytesTryInto<'a, T, const N: usize> {
    /// Performs the conversion.
    fn be_bytes_try_into(self) -> Result<Seq<T, &'a [u8]>, TryFromSliceError>;
}

impl<'a, T, const N: usize> BeBytesTryInto<'a, T, N> for &'a [u8]
where
    T: TryFromBeBytes<N>,
{
    fn be_bytes_try_into(self) -> Result<Seq<T, &'a [u8]>, TryFromSliceError> {
        T::try_from_be_bytes(self)
    }
}

/// Big endian bytes to value conversion
///
/// It is the reciprocal of [BeBytesInto].
pub trait FromBeBytes<const N: usize>: Sized {
    const SIZE: usize = size_of::<Self>();
    const ASSERT_SELF_SIZE: (usize, usize) = (Self::SIZE - N, N - Self::SIZE);
    fn from_be_bytes(bytes: [u8;N]) -> Self;
}

/// Big endian to value conversion that consumes the input bytes
///
/// The opposite of [FromBeBytes].
/// One should avoid implementing [BeBytesInto] and implement [FromBeBytes] instead.
pub trait BeBytesInto<T> {
    fn be_bytes_into(self) -> T;
}

/// Implementing [FromBeBytes] automatically provides one with an implementation of [BeBytesInto]
/// thanks to this blanket implementation.
impl<T, const N: usize> BeBytesInto<T> for [u8;N]
where
    T: FromBeBytes<N>,
{
    fn be_bytes_into(self) -> T {
        T::from_be_bytes(self)
    }
}

/// One byte array conversion
impl FromBeBytes<1> for u8 {
    fn from_be_bytes(bytes: [u8;1]) -> Self {
        bytes[0]
    }
}

/// Bytes to bytes (no)conversion
impl<const N: usize> FromBeBytes<N> for [u8;N] {
    fn from_be_bytes(bytes: [u8;N]) -> Self {
        bytes
    }
}

/// Type wrapper for big endian bytes value
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Be<T>(pub T);

/// Any type that implemented [FromBeBytes] can be converted to [Be] wrapper
impl<T: FromBeBytes<N>, const N: usize> From<[u8;N]> for Be<T> {
    fn from(data: [u8;N]) -> Self {
        Be(data.be_bytes_into())
    }
}

macro_rules! endianness_integers {
    (Common: $e:ident => $($t:ty),+ $(,)?) => { paste!{ $(
        /*
        impl FromLeBytes<2> for u16 {
            fn from_le_bytes(bytes: [u8;2]) -> Self {
                u16::from_le_bytes(bytes)
            }
        }
        */
        impl [<From $e Bytes>]<{ size_of::<Self>() }> for $t {
            fn [<from_ $e:lower _bytes>](bytes: [u8; size_of::<$t>()]) -> Self {
                $t::[<from_ $e:lower _bytes>](bytes)
            }
        }

        /*
        impl<const M: usize, const N: usize> FromLeBytes<M> for [u16; N] {
            fn from_le_bytes(bytes: [u8; M]) -> Self {
                #![allow(path_statements)]
                <Self as [<From $e Bytes>]<N>>::ASSERT_SELF_SIZE;

                let mut result = [0; N];
                for (n, data) in bytes.chunks_exact(size_of::<u16>()).enumerate() {
                    match <[u8;size_of::<u16>()]>::try_from(data) {
                        Ok(data) => result[n] = data.le_bytes_into(),
                        Err(_) => break,
                    }
                }
                result
            }
        }
        */
        impl<const N: usize, const M: usize> [<From $e Bytes>]<N> for [$t;M] {
            fn [<from_ $e:lower _bytes>](bytes: [u8;N]) -> Self {
                #![allow(path_statements)]
                <Self as [<From $e Bytes>]<N>>::ASSERT_SELF_SIZE;

                const SIZE: usize = size_of::<$t>();
                let mut result = [0;M];
                for (n, data) in bytes.chunks_exact(SIZE).enumerate() {
                    match <[u8;SIZE]>::try_from(data) {
                        Ok(data) => result[n] = data.[<$e:lower _bytes_into>](),
                        Err(_) => break,
                    }
                }
                result
            }
        }
    )+ }};
    (Le => $($t:ty),+ $(,)?) => { $(
    )+ };
    (Be => $($t:ty),+ $(,)?) => { $(
    )+ };
    ($($ty:ty),+ $(,)?) => {
        endianness_integers!(Common: Le => $($ty,)+);
        // endianness_integers!(Le => $($ty,)+);
        endianness_integers!(Common: Be => $($ty,)+);
        // endianness_integers!(Be => $($ty,)+);
    };
}

endianness_integers!(u16,u32,u64,u128,usize);


macro_rules! endianness_alphabet {
    (Common: $e:ident => $len:literal: $($cl:ident),+ $(,)?) => { paste!{
        /*
        impl<A,B,C> From<(Le<A>,Le<B>,Le<C>)> for Le<(A,B,C)> {
            fn from((Le(a),Le(b),Le(c)): (Le<A>,Le<B>,Le<C>)) -> Self {
                Le((a,b,c))
            }
        }
        */
        impl<$($cl,)+> From<($($e<$cl>,)+)> for $e<($($cl,)+)> {
            fn from(($($e([<$cl:lower>]),)+): ($($e<$cl>,)+)) -> Self {
                $e(($([<$cl:lower>],)+))
            }
        }

        /*
        impl<TY,A,B,C, const AN: usize, const BN: usize, const CN: usize>
            From<T3<[TY;AN],[TY;BN],[TY;CN]>> for Le<(A,B,C)>
        where
            Le<A>: From<[TY;AN]>,
            Le<B>: From<[TY;BN]>,
            Le<C>: From<[TY;CN]>,
        {
            fn from(data: T3<[TY;AN],[TY;BN],[TY;CN]>) -> Self {
                <(Le<A>,Le<B>,Le<C>)>::from(data).into()
            }
        }
        */
        impl<TY, $($cl,)+ $(const [<$cl N>]: usize,)+>
            From<[<T $len>]<$([TY;[<$cl N>]],)+>> for $e<($($cl,)+)>
        where
            $($e<$cl>: From<[TY;[<$cl N>]]>,)+
        {
            fn from(data: [<T $len>]<$([TY;[<$cl N>]],)+>) -> Self {
                <($($e<$cl>,)+)>::from(data).into()
            }
        }

        /*
        impl<TY,A,B,C, const N: usize, const AN: usize, const BN: usize, const CN: usize>
            From<P3<[TY;N],AN,BN,CN>> for Le<(A,B,C)>
        where
            TY: Copy + Default,
            Le<A>: From<[TY;AN]>,
            Le<B>: From<[TY;BN]>,
            Le<C>: From<[TY;CN]>,
        {
            fn from(data: P3<[TY;N],AN,BN,CN>) -> Self {
                <(Le<A>,Le<B>,Le<C>)>::from(data).into()
            }
        }
        */
        impl<TY, $($cl,)+ const NU: usize, $(const [<$cl N>]: usize,)+>
            From<[<P $len>]<[TY;NU],$([<$cl N>],)+>> for $e<($($cl,)+)>
        where
            TY: Copy + Default,
            $($e<$cl>: From<[TY;[<$cl N>]]>,)+
        {
            fn from(data: [<P $len>]<[TY;NU],$([<$cl N>],)+>) -> Self {
                <($($e<$cl>,)+)>::from(data).into()
            }
        }

        /*
        impl<A,B,C, const N: usize, const AN: usize, const BN: usize, const CN: usize>
            FromLeBytes<N> for P3<(A,B,C), AN, BN, CN>
        where
            A: FromLeBytes<AN>,
            B: FromLeBytes<BN>,
            C: FromLeBytes<CN>,
        {
            fn from_le_bytes(bytes: [u8;N]) -> Self {
                let T3(a,b,c) = bytes.into();
                P3((a.le_bytes_into(),b.le_bytes_into(),c.le_bytes_into()))
            }
        }
        */
        impl<$($cl,)+ const NU: usize, $(const [<$cl N>]: usize,)+>
            [<From $e Bytes>]<NU> for [<P $len>]<($($cl,)+), $([<$cl N>],)+>
        where
            $( $cl: [<From $e Bytes>]<[<$cl N>]>, )+
        {
            fn [<from_ $e:lower _bytes>](bytes: [u8;NU]) -> Self {
                let [<T $len>]($([<$cl:lower>],)+) = bytes.into();
                [<P $len>](($([<$cl:lower>].[<$e:lower _bytes_into>](),)+))
            }
        }
    }};
    (Le => $len:literal: $($cl:ident),+ $(,)?) => { paste!{
    }};
    (Be => $len:literal: $($cl:ident),+ $(,)?) => { paste!{
    }};
    ($len:literal: $($cl:ident),+ $(,)?) => {
        endianness_alphabet!(Common: Le => $len: $($cl),+);
        // endianness_alphabet!(Le => $len: $($cl),+);
        endianness_alphabet!(Common: Be => $len: $($cl),+);
        // endianness_alphabet!(Be => $len: $($cl),+);
    };
}

endianness_alphabet!(1: A);
endianness_alphabet!(2: A,B);
endianness_alphabet!(3: A,B,C);
endianness_alphabet!(4: A,B,C,D);
endianness_alphabet!(5: A,B,C,D,E);
endianness_alphabet!(6: A,B,C,D,E,F);
endianness_alphabet!(7: A,B,C,D,E,F,G);
endianness_alphabet!(8: A,B,C,D,E,F,G,H);
endianness_alphabet!(9: A,B,C,D,E,F,G,H,I);
endianness_alphabet!(10: A,B,C,D,E,F,G,H,I,J);
endianness_alphabet!(11: A,B,C,D,E,F,G,H,I,J,K);
endianness_alphabet!(12: A,B,C,D,E,F,G,H,I,J,K,L);
endianness_alphabet!(13: A,B,C,D,E,F,G,H,I,J,K,L,M);
endianness_alphabet!(14: A,B,C,D,E,F,G,H,I,J,K,L,M,N);
endianness_alphabet!(15: A,B,C,D,E,F,G,H,I,J,K,L,M,N,O);
endianness_alphabet!(16: A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P);
endianness_alphabet!(17: A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q);
endianness_alphabet!(18: A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R);
endianness_alphabet!(19: A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S);
endianness_alphabet!(20: A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T);
endianness_alphabet!(21: A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U);
endianness_alphabet!(22: A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V);
endianness_alphabet!(23: A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W);
endianness_alphabet!(24: A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X);
endianness_alphabet!(25: A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y);
endianness_alphabet!(26: A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y,Z);


#[cfg(test)]
mod tests {
    use super::*;
    const DATA: [u8; 31] = [
        0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE,
        0xFF, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD,
        0xEE ];
    const RESULT_LE_U8: u8 = 0x00;
    const RESULT_LE_U16: u16 = 0x1100;
    const RESULT_LE_U32: u32 = 0x33221100;
    const RESULT_LE_U64: u64 = 0x7766554433221100;
    const RESULT_LE_U128: u128 = 0xFFEEDDCCBBAA99887766554433221100;
    const RESULT_BE_U8: u8 = 0x00;
    const RESULT_BE_U16: u16 = 0x0011;
    const RESULT_BE_U32: u32 = 0x00112233;
    const RESULT_BE_U64: u64 = 0x0011223344556677;
    const RESULT_BE_U128: u128 = 0x00112233445566778899AABBCCDDEEFF;

    #[test]
    fn le_bytes_into_integer_array() {
        let data: [u8; 16] = DATA[..16].try_into().unwrap();

        let result: [u8; 16] = data.le_bytes_into();
        assert_eq!(data, result, "[u8;16]");

        let result: [u16; 8] = data.le_bytes_into();
        let sample = [
            0x1100, 0x3322, 0x5544, 0x7766, 0x9988, 0xBBAA, 0xDDCC, 0xFFEE,
        ];
        assert_eq!(sample, result, "[u16;8]");

        let result: [u32; 4] = data.le_bytes_into();
        let sample = [0x33221100, 0x77665544, 0xBBAA9988, 0xFFEEDDCC];
        assert_eq!(sample, result, "[u32;4]");

        let result: [u64; 2] = data.le_bytes_into();
        let sample = [0x7766554433221100, 0xFFEEDDCCBBAA9988];
        assert_eq!(sample, result, "[u64;2]");

        let result: [u128; 1] = data.le_bytes_into();
        let sample = [0xFFEEDDCCBBAA99887766554433221100];
        assert_eq!(sample, result, "[u128;1]");
    }

    #[test]
    fn be_bytes_into_integer_array() {
        let data: [u8; 16] = DATA[..16].try_into().unwrap();

        let result: [u8; 16] = data.be_bytes_into();
        assert_eq!(data, result, "[u8;16]");

        let result: [u16; 8] = data.be_bytes_into();
        let sample = [
            0x0011, 0x2233, 0x4455, 0x6677, 0x8899, 0xAABB, 0xCCDD, 0xEEFF,
        ];
        assert_eq!(sample, result, "[u16;8]");

        let result: [u32; 4] = data.be_bytes_into();
        let sample = [0x00112233, 0x44556677, 0x8899AABB, 0xCCDDEEFF];
        assert_eq!(sample, result, "[u32;4]");

        let result: [u64; 2] = data.be_bytes_into();
        let sample = [0x0011223344556677, 0x8899AABBCCDDEEFF];
        assert_eq!(sample, result, "[u64;2]");

        let result: [u128; 1] = data.be_bytes_into();
        let sample = [0x00112233445566778899AABBCCDDEEFF];
        assert_eq!(sample, result, "[u128;1]");
    }

    #[test]
    fn into_le_integer_array_wrapper() {
        let data: [u8; 16] = DATA[..16].try_into().unwrap();

        let result: Le<[u8; 16]> = data.into();
        assert_eq!(data, result.0, "Le<[u8;16]>");

        let result: Le<[u16; 8]> = data.into();
        let sample = [
            0x1100u16, 0x3322, 0x5544, 0x7766, 0x9988, 0xBBAA, 0xDDCC, 0xFFEE,
        ];
        assert_eq!(sample, result.0, "Le<[u16;8]>");

        let result: Le<[u32; 4]> = data.into();
        let sample = [0x33221100, 0x77665544, 0xBBAA9988, 0xFFEEDDCC];
        assert_eq!(sample, result.0, "Le<[u32;4]>");

        let result: Le<[u64; 2]> = data.into();
        let sample = [0x7766554433221100, 0xFFEEDDCCBBAA9988];
        assert_eq!(sample, result.0, "Le<[u64;2]>");

        let result: Le<[u128; 1]> = data.into();
        let sample = [0xFFEEDDCCBBAA99887766554433221100];
        assert_eq!(sample, result.0, "Le<[u128;1]>");
    }

    #[test]
    fn into_mixed_integers() {
        let data: [u8; 8] = DATA[..8].try_into().unwrap();
        let (Be(a), Le(b), c) = T3::from(data).into();
        let _: (u16, u32, [u8; 2]) = (a, b, c);

        let sample = (0x0011, 0x55443322, [0x66, 0x77]);
        assert_eq!(sample, (a, b, c), "mixed integers");
    }

    #[test]
    fn into_mixed_arrays() {
        let data: [u8; 8] = DATA[..8].try_into().unwrap();
        let (Be(a), Le(b)) = P2::<_, 4, 4>(data).into();
        let _: ([u16; 2], [u32; 1]) = (a, b);

        let sample = ([0x0011, 0x2233], [0x77665544]);
        assert_eq!(sample, (a, b), "mixed arrays");
    }

    #[test]
    fn into_tupled_integers() {
        let result: Le<(u8, u16, u32, u64, u128)> = T5::from(DATA).into();
        let sample = (
            0x00,
            0x2211,
            0x66554433,
            0xEEDDCCBBAA998877,
            0xEEDDCCBBAA99887766554433221100FF,
        );
        assert_eq!(sample, result.0, "u8 .. u128");

        let result: Le<(usize,)> = T1::from(0x1234usize.to_le_bytes()).into();
        assert_eq!(0x1234usize, result.0 .0, "usize");
    }

    #[test]
    fn destructuring() {
        if let Le((0x1100u16, data)) = T2::from(DATA).into() {
            let _: [u8; 29] = data;
        } else {
            panic!();
        }
    }

    #[test]
    fn le_bytes_into_mixed_integers() {
        let data: [u8; 8] = DATA[..8].try_into().unwrap();
        let P3((a, b, c)) = data.le_bytes_into();
        let _: (u16, u32, [u8; 2]) = (a, b, c);

        let sample = (0x1100, 0x55443322, [0x66, 0x77]);
        assert_eq!(sample, (a, b, c), "mixed integers");
    }

    macro_rules! integers {
        ($e:ident => $($ty:ty),+ $(,)?) => { paste!{ $(

            #[test]
            fn [<try_from_ $e:lower _bytes_ $ty>]() {
                // let Seq { head: result, .. } =
                //     TryFromLeBytes::try_from_le_bytes(&DATA[..2]).unwrap();
                let Seq { head: result, .. } =
                    [<TryFrom $e Bytes>]::[<try_from_ $e:lower _bytes>](&DATA[..size_of::<$ty>()])
                        .unwrap();
                assert_eq!([<RESULT_ $e:upper _ $ty:upper>], result);
            }

            #[test]
            fn [<$e:lower _bytes_into_ $ty>]() {
                // let data: [u8; 1] = DATA[..1].try_into().unwrap();
                // let result: u8 = data.le_bytes_into();
                let data: [u8; size_of::<$ty>()] =
                    DATA[..size_of::<$ty>()].try_into().unwrap();
                let result: $ty = data.[<$e:lower _bytes_into>]();
                assert_eq!([<RESULT_ $e:upper _ $ty:upper>], result);
            }

            #[test]
            fn [<into_ $e:lower _wrapper_ $ty>]() {
                // let data: [u8; 1] = DATA[..1].try_into().unwrap();
                // let result: Le<u8> = data.into();
                let data: [u8; size_of::<$ty>()] =
                    DATA[..size_of::<$ty>()].try_into().unwrap();
                let result: [<$e>]<$ty> = data.into();
                assert_eq!([<RESULT_ $e:upper _ $ty:upper>], result.0);
            }

            #[test]
            fn [<$e:lower _bytes_try_into_ $ty>]() {
                // let Seq { head: result, .. } = DATA[..2].le_bytes_try_into().unwrap();
                let Seq { head: result, .. } =
                    DATA[..size_of::<$ty>()].[<$e:lower _bytes_try_into>]().unwrap();
                assert_eq!([<RESULT_ $e:upper _ $ty:upper>], result);
            }

        )+ } }
    }
    integers!(Le => u8,u16,u32,u64,u128);
    integers!(Be => u8,u16,u32,u64,u128);
}

