/*!
# Type wrappers

This module defines type wrappers for 1..26 generic parameters. For example, tuple wrapper for arity equal to six named [T6].

## Tuple wrappers

These wrappers used to implement transformations using std trait [From].

Example: [T3]

Implementations:
- [splits one array to multiple arrays](struct.T3.html#impl-From%3C%5BT%3B%20N%5D%3E)

  `[T;N]` -> `[T;AN],[T;BN],[T;CN]` where `AN + BN + CN = N`

- [helps transforms from native tuple](struct.T3.html#impl-From%3C(A0%2C%20B0%2C%20C0)%3E)

  `(A0, B0, C0)` -> `T3(A1, B1, C1)` where A1, B1, C1 have [From] A0, B0, C0 traits respectively

- [helps transforms into native tuple](struct.T3.html#impl-From%3CT3%3CA0%2C%20B0%2C%20C0%3E%3E)

  `T3(A0, B0, C0)` -> `(A1, B1, C1)` where A1, B1, C1 have [From] A0, B0, C0 traits respectively

## Ready to partition value wrappers

Example: [P3]

These wrappers used to show how value could be partitioned

Implementations:
- [transforms array to tuple of types](struct.P3.html#impl-From%3CP3%3C%5BTY%3B%20NU%5D%2C%20AN%2C%20BN%2C%20CN%3E%3E)

  `[T; N]` -> `(A, B, C)` where A, B, C have [From] `[T; AN]`, `[T; BN]`, `[T; CN]` traits respectively

- [try to transform slice to tuple of types](struct.P3.html#impl-TryFrom%3CP3%3C%26%27a%20%5BT%5D%2C%20AN%2C%20BN%2C%20CN%3E%3E)

  `&[N]` -> `(A, B, C)` where A, B, C have [From] `[T; AN]`, `[T; BN]`, `[T; CN]` traits respectively

It also used to reduce records length of same type tuple wrappers:
`T3<[u16; 1], [u16; 2], [u16; 3]>` equals to `P3<u16, 1, 2, 3>`

## Slice Transformation

*/

use core::array::TryFromSliceError;

use paste::paste;
use funty::Fundamental;


/// Trait derives primitive types.
///
/// 'Opposite' to [funty::Fundamental](https://docs.rs/funty/latest/funty/trait.Fundamental.html)
/// trait
pub trait AsPrimitive<T> {
    fn as_primitive(self) -> T;
}
impl<T: Fundamental> AsPrimitive<Option<char>> for T {
    fn as_primitive(self) -> Option<char> {
        self.as_char()
    }
}

/// Reserved space unit type implementation
impl<T> AsPrimitive<()> for T {
    fn as_primitive(self) {}
}

macro_rules! main_impl_for {
    ( AsPrimitive => $($cl:ty),+ $(,)?) => {paste!{ $(
        impl<T: Fundamental> AsPrimitive<$cl> for T {
            fn as_primitive(self) -> $cl { self.[<as_ $cl>]() }
        }
    )+ }};
}

main_impl_for!(AsPrimitive => bool,u8,u16,u32,u64,u128);


/// Compile time const generic validation
pub trait ParamAndAssociatedConst<const N: usize> {
    const VALUE: usize;
    const LESS: usize = Self::VALUE - N;
    const GREATER: usize = N - Self::VALUE;
    const ASSERT_EQ: (usize, usize) = (Self::LESS, Self::GREATER);
}

/// Sequence of elements with head and tail
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Seq<H, T> {
    pub head: H,
    pub tail: T,
}

/**
Fallible conversion from slice to array

In contrast with std
[`TryFrom<&'_ [T]> for [T; N]`](https://doc.rust-lang.org/std/primitive.array.html#impl-TryFrom%3C%26%27_%20%5BT%5D%3E)
this implementation return array and slice tail on any slice that longer than array
```rust
# use heterob::Seq;
let bytes = [1u8, 2, 2, 3, 3, 3, 3];
let seq: Seq<[_; 3], _> = bytes[..].try_into().unwrap();
assert_eq!(Seq { head: [1, 2, 2], tail: [3, 3, 3, 3].as_slice() }, seq);
```
*/

impl<'a, T, const N: usize> TryFrom<&'a [T]> for Seq<[T; N], &'a [T]>
where
    T: Copy,
{
    type Error = TryFromSliceError;

    fn try_from(slice: &'a [T]) -> Result<Self, Self::Error> {
        let (head, tail) = slice.split_at(slice.len().min(N));
        Ok(Self {
            head: head.try_into()?,
            tail,
        })
    }
}

macro_rules! main_alphabet {
    ($len:expr; $($cl:ident),+ $(,)?) => { paste!{
        // #[derive(Debug, Clone, PartialEq, Eq)]
        // pub struct T3<A, B, C>(pub A, pub B, pub C);
        #[doc=concat!($len, "-ary tuple wrapper")]
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct [<T $len>]<$($cl,)+>($(pub $cl,)+);

        /*
        impl<T, const N: usize, const AN: usize, const BN: usize, const CN: usize>
            ParamAndAssociatedConst<N> for P3<T, AN, BN, CN>
        {
            const VALUE: usize = AN + BN + CN;
        }
        */
        impl<T, const N: usize, $(const [<$cl N>]:usize,)+> ParamAndAssociatedConst<N>
            for [<T $len>]<$([T;[<$cl N>]],)+>
        {
            const VALUE: usize = 0 $(+ [<$cl N>])+;
        }

        /*
        impl<A0, A1, B0, B1, C0, C1> From<T3<A0, B0, C0>> for (A1, B1, C1)
        where
            A1: From<A0>,
            B1: From<B0>,
            C1: From<C0>,
        {
            fn from(T3(a, b, c): T3<A0, B0, C0>) -> Self {
                (a.into(), b.into(), c.into())
            }
        }
        */
        impl<$([<$cl 0>],[<$cl 1>],)+> From<[<T $len>]<$([<$cl 0>],)+>> for ($([<$cl 1>],)+)
        where
            $([<$cl 1>]: From<[<$cl 0>]>,)+
        {
            fn from([<T $len>]($([<$cl:lower>],)+): [<T $len>]<$([<$cl 0>],)+>) -> Self {
                ( $([<$cl:lower>].into(),)+ )
            }
        }

        /*
        impl<A0, A1, B0, B1, C0, C1> From<(A0, B0, C0)> for T3<A1, B1, C1>
        where
            A1: From<A0>,
            B1: From<B0>,
            C1: From<C0>,
        {
            fn from((a, b, c): (A0, B0, C0)) -> Self {
                T3(a.into(), b.into(), c.into())
            }
        }
        */
        impl<$([<$cl 0>],[<$cl 1>],)+> From<($([<$cl 0>],)+)> for [<T $len>]<$([<$cl 1>],)+>
        where
            $([<$cl 1>]: From<[<$cl 0>]>,)+
        {
            fn from(($([<$cl:lower>],)+): ($([<$cl 0>],)+)) -> Self {
                Self( $([<$cl:lower>].into(),)+ )
            }
        }

        /*
        impl<T, const N: usize, const AN: usize, const BN: usize, const CN: usize>
            From<[T; N]> for T3<[T; AN], [T; BN], [T; CN]>
        where
            T: Default + Copy,
        {
            fn from(data: [T; N]) -> Self {
                #![allow(path_statements)]
                <Self as ParamAndAssociatedConst<N>>::ASSERT_EQ;

                let end = 0;

                let mut a = [Default::default(); AN];
                let (start, end) = (end, end + AN);
                a.copy_from_slice(&data[start..end]);

                let mut b = [Default::default(); BN];
                let (start, end) = (end, end + BN);
                b.copy_from_slice(&data[start..end]);

                let mut c = [Default::default(); CN];
                let (start, end) = (end, end + CN);
                c.copy_from_slice(&data[start..end]);

                T3(a, b, c)
            }
        }
        */
        impl<T, const N: usize, $(const [<$cl N>]:usize,)+>
            From<[T; N]> for [<T $len>]<$([T;[<$cl N>]],)+>
        where
            T: Default + Copy,
        {
            fn from(data: [T; N]) -> Self {
                #![allow(path_statements)]
                <Self as ParamAndAssociatedConst::<N>>::ASSERT_EQ;

                let end = 0;
                $(
                    let mut [<$cl:lower>] = [Default::default();[<$cl N>]];
                    let (start, end) = (end, end + [<$cl N>]);
                    [<$cl:lower>].copy_from_slice(&data[start..end]);
                )+
                Self($([<$cl:lower>],)+)
            }
        }

        // #[derive(Debug, Clone, PartialEq, Eq)]
        // pub struct P3<TY, const A: usize, const B: usize, const C: usize>(pub TY);
        #[doc=concat!("Type wrapper with ", $len, " const generic parameters")]
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct [<P $len>]<TY, $(const $cl: usize,)+>(pub TY);

        /*
        impl<TY,A,B,C, const N: usize, const AN: usize, const BN: usize, const CN: usize>
            From<P3<[TY;N],AN,BN,CN>> for (A,B,C,)
        where
            TY: Copy + Default,
            A: From<[TY;AN]>,
            B: From<[TY;BN]>,
            C: From<[TY;CN]>,
        {
            fn from(P3(data): P3<[TY;N],AN,BN,CN>) -> Self {
                T3::from(data).into()
            }
        }
        */
        impl<TY, $($cl,)+ const NU: usize, $(const [<$cl N>]: usize,)+>
            From<[<P $len>]<[TY;NU],$([<$cl N>],)+>> for ($($cl,)+)
        where
            TY: Copy + Default,
            $($cl: From<[TY;[<$cl N>]]>,)+
        {
            fn from([<P $len>](data): [<P $len>]<[TY;NU],$([<$cl N>],)+>) -> Self {
                [<T $len>]::from(data).into()
            }
        }

        /*
        impl<'a, T, const AN: usize, const BN: usize, const CN: usize> TryFrom<&'a [T]>
            for Seq<T3<[T; AN], [T; BN], [T; CN]>, &'a [T]>
        where
            T: Copy,
        {
            type Error = TryFromSliceError;

            fn try_from(slice: &'a [T]) -> Result<Self, Self::Error> {
                let Seq { head: a, tail: slice }: Seq<_, &[T]> = slice.try_into()?;
                let Seq { head: b, tail: slice }: Seq<_, &[T]> = slice.try_into()?;
                let Seq { head: c, tail: slice }: Seq<_, &[T]> = slice.try_into()?;
                Ok(Self {
                    head: T3(a, b, c),
                    tail: slice,
                })
            }
        }
        */
        impl<'a, T, $(const [<$cl N>]:usize,)+> TryFrom<&'a [T]>
            for Seq<[<T $len>]<$([T;[<$cl N>]],)+>, &'a [T]>
        where
            T: Copy,
        {
            type Error = TryFromSliceError;

            fn try_from(slice: &'a [T]) -> Result<Self, Self::Error> {
                $(
                    let Seq { head: [<$cl:lower>], tail: slice }: Seq<_, &[T]> =
                        slice.try_into()?;
                )+
                Ok(Self {
                    head: [<T $len>]($([<$cl:lower>],)+),
                    tail: slice,
                })
            }
        }

        /*
        impl<'a, T, U, const AN: usize, const BN: usize, const CN: usize>
            TryFrom<P3<&'a [T], AN, BN, CN>> for Seq<U, &'a [T]>
        where
            T: Copy,
            U: From<T3<[T; AN], [T; BN], [T; CN]>>,
        {
            type Error = TryFromSliceError;

            fn try_from(P3(data): P3<&'a [T], AN, BN, CN>) -> Result<Self, Self::Error> {
                data.try_into().map(|Seq { head, tail }| Seq {
                    head: From::<T3<[T; AN], [T; BN], [T; CN]>>::from(head),
                    tail,
                })
            }
        }
        */
        impl<'a, T, U, $(const [<$cl N>]: usize,)+>
            TryFrom<[<P $len>]<&'a [T], $([<$cl N>],)+>> for Seq<U, &'a [T]>
        where
            T: Copy,
            U: From<[<T $len>]<$([T;[<$cl N>]],)+>>
        {
            type Error = TryFromSliceError;

            fn try_from([<P $len>](data): [<P $len>]<&'a [T], $([<$cl N>],)+>) ->
                 Result<Self, Self::Error>
            {
                data.try_into().map(|Seq { head,  tail }| Seq {
                    head: From::<[<T $len>]<$([T; [<$cl N>]],)+>>::from(head),
                    tail,
                })
            }
        }
    }};
}

// Monuple
main_alphabet!(1; A);
// Couple
main_alphabet!(2; A,B);
// Triple
main_alphabet!(3; A,B,C);
// Quadruple
main_alphabet!(4; A,B,C,D);
// Quintuple
main_alphabet!(5; A,B,C,D,E);
// Sextuple
main_alphabet!(6; A,B,C,D,E,F);
// Septuple
main_alphabet!(7; A,B,C,D,E,F,G);
// Octuple
main_alphabet!(8; A,B,C,D,E,F,G,H);
// Nonuple
main_alphabet!(9; A,B,C,D,E,F,G,H,I);
// Decuple
main_alphabet!(10; A,B,C,D,E,F,G,H,I,J);
// Undecuple
main_alphabet!(11; A,B,C,D,E,F,G,H,I,J,K);
// Duodecuple
main_alphabet!(12; A,B,C,D,E,F,G,H,I,J,K,L);
// Tredecuple
main_alphabet!(13; A,B,C,D,E,F,G,H,I,J,K,L,M);
// Quattuordecuple
main_alphabet!(14; A,B,C,D,E,F,G,H,I,J,K,L,M,N);
// Quindecuple
main_alphabet!(15; A,B,C,D,E,F,G,H,I,J,K,L,M,N,O);
// Sexdecuple
main_alphabet!(16; A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P);
// Septendecuple
main_alphabet!(17; A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q);
// Octodecuple
main_alphabet!(18; A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R);
// Novemdecuple
main_alphabet!(19; A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S);
// Vigintuple
main_alphabet!(20; A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T);
// Unvigintuple
main_alphabet!(21; A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U);
// Duovigintuple
main_alphabet!(22; A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V);
// Trevigintuple
main_alphabet!(23; A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W);
// Quattuorvigintuple
main_alphabet!(24; A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X);
// Quinvigintuple
main_alphabet!(25; A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y);
// Sexvigintuple
main_alphabet!(26; A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y,Z);


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
    fn tuple_of_elements_from() {
        assert_eq!((97u128, 98usize), T2(97u8, 98u16).into());
        assert_eq!(('a', 'b', 'c'), T3(97, 98, 99).into());
    }

    #[test]
    fn tuple_of_elements_into() {
        assert_eq!(T2(97u32, 98i32), (97u8, 98u16).into());
        assert_eq!(T3('a', 'b', 'c'), (97, 98, 99).into());
    }

    #[test]
    fn slice_try_into_tuple_of_arrays() {
        let bytes = [1u8, 2, 2, 3, 3, 3, 3];
        let Seq {
            head: T3(a, b, c), ..
        } = bytes[..].try_into().unwrap();
        assert_eq!(([1], [2, 2], [3, 3, 3]), (a, b, c));
    }

    #[test]
    fn partition_ready_longer_slice_try_into() {
        let bytes = [1u8, 2, 2, 3, 3, 3, 3, 42];

        let result = bytes.as_slice().try_into().ok();
        let sample = Some(Seq {
            head: T3([1], [2, 2], [3, 3, 3, 3]),
            tail: &bytes[7..],
        });
        assert_eq!(sample, result, "tuple warpper");

        let result = P3(bytes.as_slice()).try_into().ok();
        let sample = Some(Seq {
            head: ([1], [2, 2], [3, 3, 3, 3]),
            tail: &bytes[7..],
        });
        assert_eq!(sample, result, "tuple of arrays");
    }

    #[test]
    fn partition_ready_exact_slice_try_into() {
        let bytes = [1u8, 2, 2, 3, 3, 3, 3];

        let result = bytes.as_slice().try_into().ok();
        let sample = Some(Seq {
            head: T3([1], [2, 2], [3, 3, 3, 3]),
            tail: [].as_slice(),
        });
        assert_eq!(sample, result, "tuple warpper");

        let result = P3(bytes.as_slice()).try_into().ok();
        let sample = Some(Seq {
            head: ([1], [2, 2], [3, 3, 3, 3]),
            tail: [].as_slice(),
        });
        assert_eq!(sample, result, "tuple of arrays");
    }

    #[test]
    fn partition_ready_shorter_slice_try_into() {
        let bytes = [1u8, 2, 2, 3, 3];

        let result: Option<Seq<T3<[_; 1], [_; 2], [_; 3]>, &[u8]>> =
            bytes.as_slice().try_into().ok();
        let sample = None;
        assert_eq!(sample, result, "slice is shorter");

        let result: Option<Seq<([_; 1], [_; 2], [_; 3]), &[u8]>> =
            P3(bytes.as_slice()).try_into().ok();
        let sample = None;
        assert_eq!(sample, result, "slice is shorter");
    }
}
