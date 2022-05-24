/*!
# Type wrappers

This module defines type wrappers for 1..26 generic parameters. For example, tuple wrapper for arity equal to six named [T6].

## Tuple wrapper

Wrappers used to implement transformations using std trait [From].

```rust
pub struct T3<A, B, C>(pub A, pub B, pub C);

```
This implementation splits one array to multiple arrays: `[T;N]` -> `[T;AN],[T;BN],[T;CN]` where `AN + BN + CN = N`
```rust
# pub struct T3<A, B, C>(pub A, pub B, pub C);
# use heterob::ParamAndAssociatedConst;
# impl<T, const N: usize, const AN: usize, const BN: usize, const CN: usize> ParamAndAssociatedConst<N> for T3<[T;AN],[T;BN],[T;CN]> {
#     const VALUE: usize = AN + BN + CN;
# }
impl<T, const N: usize, const AN: usize, const BN: usize, const CN: usize> From<[T; N]> for T3<[T;AN],[T;BN],[T;CN]>
where
    T: Default + Copy,
{
    fn from(data: [T; N]) -> Self {
        #![allow(path_statements)]
        <Self as ParamAndAssociatedConst::<N>>::ASSERT_EQ;

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

        T3(a,b,c)
    }
}
```
This two helps transforms from/into native tuple
```rust
# pub struct T3<A, B, C>(pub A, pub B, pub C);
impl<A0,A1,B0,B1,C0,C1> From<T3<A0,B0,C0>> for (A1,B1,C1)
where
    A1: From<A0>,
    B1: From<B0>,
    C1: From<C0>,
{
    fn from(T3(a,b,c): T3<A0,B0,C0>) -> Self {
        (a.into(), b.into(), c.into())
    }
}

impl<A0,A1,B0,B1,C0,C1> From<(A0,B0,C0)> for T3<A1,B1,C1>
where
    A1: From<A0>,
    B1: From<B0>,
    C1: From<C0>,
{
    fn from((a,b,c): (A0,B0,C0)) -> Self {
        T3(a.into(), b.into(), c.into())
    }
}
```

## Ready to partition value wrappers

Wrappers used to show how value could be partitioned
```rust
# use heterob::ParamAndAssociatedConst;
# pub struct T3<A, B, C>(pub A, pub B, pub C);
# impl<T, const N: usize, const AN: usize, const BN: usize, const CN: usize> From<[T; N]> for T3<[T;AN],[T;BN],[T;CN]>
# where
#     T: Default + Copy,
# {
#     fn from(data: [T; N]) -> Self {
#         #![allow(path_statements)]
#         <Self as ParamAndAssociatedConst::<N>>::ASSERT_EQ;
#
#         let end = 0;
#
#         let mut a = [Default::default(); AN];
#         let (start, end) = (end, end + AN);
#         a.copy_from_slice(&data[start..end]);
#
#         let mut b = [Default::default(); BN];
#         let (start, end) = (end, end + BN);
#         b.copy_from_slice(&data[start..end]);
#
#         let mut c = [Default::default(); CN];
#         let (start, end) = (end, end + CN);
#         c.copy_from_slice(&data[start..end]);
#
#         T3(a,b,c)
#     }
# }
# impl<T, const N: usize, const AN: usize, const BN: usize, const CN: usize> ParamAndAssociatedConst<N> for T3<[T;AN],[T;BN],[T;CN]> {
#     const VALUE: usize = AN + BN + CN;
# }
# impl<A0,A1,B0,B1,C0,C1> From<T3<A0,B0,C0>> for (A1,B1,C1)
# where
#     A1: From<A0>,
#     B1: From<B0>,
#     C1: From<C0>,
# {
#     fn from(T3(a,b,c): T3<A0,B0,C0>) -> Self {
#         (a.into(), b.into(), c.into())
#     }
# }

pub struct P3<TY, const A: usize, const B: usize, const C: usize>(pub TY);

# impl<T, const N: usize, const AN: usize, const BN: usize, const CN: usize> ParamAndAssociatedConst<N> for P3<T, AN, BN, CN> {
#     const VALUE: usize = AN + BN + CN;
# }

impl<TY,A,B,C, const N: usize, const AN: usize, const BN: usize, const CN: usize> From<P3<[TY;N],AN,BN,CN>> for (A,B,C,)
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
````
It also used to reduce record length of same type tuple wrappers `T3<[u16; 1], [u16; 2], [u16; 3]>` equals to `P3<u16, 1, 2, 3>`
*/

use paste::paste;

use funty::Fundamental;

/// Compile time const generic validation
pub trait ParamAndAssociatedConst<const N: usize> {
    const VALUE: usize;
    const LESS: usize = Self::VALUE - N;
    const GREATER: usize = N - Self::VALUE;
    const ASSERT_EQ: (usize, usize) = (Self::LESS, Self::GREATER);
}

macro_rules! main_alphabet {
    ($len:literal: $($cl:ident),+ $(,)?) => { paste!{
        #[doc=concat!($len, "-ary tuple wrapper")]
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct [<T $len>]<$($cl,)+>($(pub $cl,)+);

        // impl<T, const N: usize, const AN: usize, .. > CmpConstParamUsize<N> for T#<[T;AN], .. > {
        impl<T, const N: usize, $(const [<$cl N>]:usize,)+> ParamAndAssociatedConst<N> for [<T $len>]<$([T;[<$cl N>]],)+> {
            const VALUE: usize = 0 $(+ [<$cl N>])+;
        }

        // impl<A0,A1, .. > From<T#<A0, .. >> for (A1, .. )
        impl<$([<$cl 0>],[<$cl 1>],)+> From<[<T $len>]<$([<$cl 0>],)+>> for ($([<$cl 1>],)+)
        where
            $([<$cl 1>]: From<[<$cl 0>]>,)+
        {
            fn from([<T $len>]($([<$cl:lower>],)+): [<T $len>]<$([<$cl 0>],)+>) -> Self {
                ( $([<$cl:lower>].into(),)+ )
            }
        }

        // impl<A0,A1, .. > From<(A0, .. )> for T#<A1, .. >
        impl<$([<$cl 0>],[<$cl 1>],)+> From<($([<$cl 0>],)+)> for [<T $len>]<$([<$cl 1>],)+>
        where
            $([<$cl 1>]: From<[<$cl 0>]>,)+
        {
            fn from(($([<$cl:lower>],)+): ($([<$cl 0>],)+)) -> Self {
                Self( $([<$cl:lower>].into(),)+ )
            }
        }

        // impl<T, const N: usize, const AN: usize, .. > From<[T; N]> for T#<[T;AN], .. >
        impl<T, const N: usize, $(const [<$cl N>]:usize,)+> From<[T; N]> for [<T $len>]<$([T;[<$cl N>]],)+>
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

        #[doc=concat!("Type wrapper with ", $len, " const generic parameters")]
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct [<P $len>]<TY, $(const $cl: usize,)+>(pub TY);

        // impl<TY, A, .. , const N: usize, const AN: usize, .. > From<P#<[TY;N], AN, .. >> for (A, .. )
        impl<TY, $($cl,)+ const NU: usize, $(const [<$cl N>]: usize,)+> From<[<P $len>]<[TY;NU],$([<$cl N>],)+>> for ($($cl,)+)
        where
            TY: Copy + Default,
            $($cl: From<[TY;[<$cl N>]]>,)+
        {
            fn from([<P $len>](data): [<P $len>]<[TY;NU],$([<$cl N>],)+>) -> Self {
                [<T $len>]::from(data).into()
            }
        }
    }};
}

// Monuple
main_alphabet!(1: A);
// Couple
main_alphabet!(2: A,B);
// Triple
main_alphabet!(3: A,B,C);
// Quadruple
main_alphabet!(4: A,B,C,D);
// Quintuple
main_alphabet!(5: A,B,C,D,E);
// Sextuple
main_alphabet!(6: A,B,C,D,E,F);
// Septuple
main_alphabet!(7: A,B,C,D,E,F,G);
// Octuple
main_alphabet!(8: A,B,C,D,E,F,G,H);
// Nonuple
main_alphabet!(9: A,B,C,D,E,F,G,H,I);
// Decuple
main_alphabet!(10: A,B,C,D,E,F,G,H,I,J);
// Undecuple
main_alphabet!(11: A,B,C,D,E,F,G,H,I,J,K);
// Duodecuple
main_alphabet!(12: A,B,C,D,E,F,G,H,I,J,K,L);
// Tredecuple
main_alphabet!(13: A,B,C,D,E,F,G,H,I,J,K,L,M);
// Quattuordecuple
main_alphabet!(14: A,B,C,D,E,F,G,H,I,J,K,L,M,N);
// Quindecuple
main_alphabet!(15: A,B,C,D,E,F,G,H,I,J,K,L,M,N,O);
// Sexdecuple
main_alphabet!(16: A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P);
// Septendecuple
main_alphabet!(17: A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q);
// Octodecuple
main_alphabet!(18: A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R);
// Novemdecuple
main_alphabet!(19: A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S);
// Vigintuple
main_alphabet!(20: A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T);
// Unvigintuple
main_alphabet!(21: A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U);
// Duovigintuple
main_alphabet!(22: A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V);
// Trevigintuple
main_alphabet!(23: A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W);
// Quattuorvigintuple
main_alphabet!(24: A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X);
// Quinvigintuple
main_alphabet!(25: A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y);
// Sexvigintuple
main_alphabet!(26: A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y,Z);

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
