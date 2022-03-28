#![doc = include_str!("../README.md")]

#![no_std]

#[cfg(test)]
#[macro_use]
extern crate std;

#[macro_use]
mod macros;

use paste::paste;
use funty::Fundamental;


/// Compile time const generic validation
pub trait ParamAndAssociatedConst<const N: usize> {
    const VALUE: usize;
    const LESS: usize = Self::VALUE - N;
    const GREATER: usize = N - Self::VALUE;
    const ASSERT_EQ: (usize, usize) = (Self::LESS, Self::GREATER);
}


// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct T3<A,B,C>(pub A, pub B, pub C);

// impl<T, const N: usize, const AN: usize, const BN: usize, const CN: usize> ParamAndAssociatedConst<N> for T3<[T;AN],[T;BN],[T;CN]> {
//     const VALUE: usize = AN + BN + CN;
// }

// impl<T, const N: usize, const AN: usize, const BN: usize, const CN: usize> From<[T; N]> for T3<[T;AN],[T;BN],[T;CN]>
// where
//     T: Default + Copy,
// {
//     fn from(data: [T; N]) -> Self {
//         #![allow(path_statements)]
//         <Self as ParamAndAssociatedConst::<N>>::ASSERT_EQ;

//         let end = 0;

//         let mut a = [Default::default(); AN];
//         let (start, end) = (end, end + AN);
//         a.copy_from_slice(&data[start..end]);

//         let mut b = [Default::default(); BN];
//         let (start, end) = (end, end + BN);
//         b.copy_from_slice(&data[start..end]);

//         let mut c = [Default::default(); CN];
//         let (start, end) = (end, end + CN);
//         c.copy_from_slice(&data[start..end]);

//         T3(a,b,c)
//     }
// }

// impl<A0,A1,B0,B1,C0,C1> From<T3<A0,B0,C0>> for (A1,B1,C1)
// where
//     A1: From<A0>,
//     B1: From<B0>,
//     C1: From<C0>,
// {
//     fn from(T3(a,b,c): T3<A0,B0,C0>) -> Self {
//         (a.into(), b.into(), c.into())
//     }
// }

// impl<A0,A1,B0,B1,C0,C1> From<(A0,B0,C0)> for T3<A1,B1,C1>
// where
//     A1: From<A0>,
//     B1: From<B0>,
//     C1: From<C0>,
// {
//     fn from((a,b,c): (A0,B0,C0)) -> Self {
//         T3(a.into(), b.into(), c.into())
//     }
// }

// pub struct P3<TY, const A: usize, const B: usize, const C: usize>(pub TY);
// impl<T, const N: usize, const AN: usize, const BN: usize, const CN: usize> ParamAndAssociatedConst<N> for P3<T, AN, BN, CN> {
//     const VALUE: usize = AN + BN + CN;
// }

// impl<TY,A,B,C, const N: usize, const AN: usize, const BN: usize, const CN: usize> From<P3<[TY;N],AN,BN,CN>> for (A,B,C,)
// where
//     TY: Copy + Default,
//     A: From<[TY;AN]>,
//     B: From<[TY;BN]>,
//     C: From<[TY;CN]>,
// {
//     fn from(P3(data): P3<[TY;N],AN,BN,CN>) -> Self {
//         T3::from(data).into()
//     }
// }



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
    fn as_primitive(self) -> Option<char> { self.as_char() }
}

/// Reserved space unit type implementation
impl<T> AsPrimitive<()> for T {
    fn as_primitive(self) {}
}

main_impl_for!(AsPrimitive => bool,u8,u16,u32,u64,u128);


pub mod endianness;
pub mod bit_numbering;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_array() {
        let chars = ['a','b','b',];
        assert_eq!(T2(['a'],['b','b']), T2::from(chars));

        let bytes = [1u8,2,2,3,3,3,3];
        assert_eq!(T3([1],[2,2],[3,3,3,3]), T3::from(bytes));
    }

    #[test]
    fn tuple_elements_from() {
        assert_eq!((97u128,98usize), T2(97u8,98u16).into());
        assert_eq!(('a','b','c'), T3(97,98,99).into());
    }

    #[test]
    fn tuple_elements_into() {
        assert_eq!(T2(97u32,98i32), (97u8,98u16).into());
        assert_eq!(T3('a','b','c'), (97,98,99).into());
    }
}
