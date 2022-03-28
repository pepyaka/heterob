macro_rules! main_alphabet {
    ($len:literal: $($cl:ident),+ $(,)?) => { paste!{
        // pub struct T#<A, .. >(A, .. );
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

        // pub struct P#<TY, const A: usize, .. >(TY);
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

macro_rules! main_impl_for {
    ( AsPrimitive => $($cl:ty),+ $(,)?) => {paste!{ $(
        impl<T: Fundamental> AsPrimitive<$cl> for T {
            fn as_primitive(self) -> $cl { self.[<as_ $cl>]() }
        }
    )+ }};
}



macro_rules! endianness_alphabet {
    (Common: $e:ident => $len:literal: $($cl:ident),+ $(,)?) => { paste!{
        // impl<A, .. > From<(Le<A>, .. )> for Le<(A, .. )> {
        impl<$($cl,)+> From<($($e<$cl>,)+)> for $e<($($cl,)+)> {
            fn from(($($e([<$cl:lower>]),)+): ($($e<$cl>,)+)) -> Self {
                $e(($([<$cl:lower>],)+))
            }
        }

        // impl<TY, A, .. , const AN: usize, .. > From<T#<[TY;AN], .. >> for Le<(A, .. )>
        impl<TY, $($cl,)+ $(const [<$cl N>]: usize,)+> From<[<T $len>]<$([TY;[<$cl N>]],)+>> for $e<($($cl,)+)>
        where
            $($e<$cl>: From<[TY;[<$cl N>]]>,)+
        {
            fn from(data: [<T $len>]<$([TY;[<$cl N>]],)+>) -> Self {
                <($($e<$cl>,)+)>::from(data).into()
            }
        }

        // impl<TY, A, .. , const N: usize, const AN: usize, .. > From<P#<[TY;N],AN, .. >> for Le<(A, .. )>
        impl<TY, $($cl,)+ const NU: usize, $(const [<$cl N>]: usize,)+> From<[<P $len>]<[TY;NU],$([<$cl N>],)+>> for $e<($($cl,)+)>
        where
            TY: Copy + Default,
            $($e<$cl>: From<[TY;[<$cl N>]]>,)+
        {
            fn from(data: [<P $len>]<[TY;NU],$([<$cl N>],)+>) -> Self {
                <($($e<$cl>,)+)>::from(data).into()
            }
        }

        // impl<A, .. , const N: usize, const AN: usize, .. > FromLeBytes<N> for P#<(A, .. ), AN, .. >
        impl<$($cl,)+ const NU: usize, $(const [<$cl N>]: usize,)+> [<From $e Bytes>]<NU> for [<P $len>]<($($cl,)+), $([<$cl N>],)+>
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

macro_rules! endianness_integers {
    (Common: $e:ident => $($t:ty),+ $(,)?) => { paste!{ $(
        // impl FromLeBytes<2> for u16 {
        impl [<From $e Bytes>]<{ size_of::<Self>() }> for $t {
            fn [<from_ $e:lower _bytes>](bytes: [u8; size_of::<$t>()]) -> Self {
                $t::[<from_ $e:lower _bytes>](bytes)
            }
        }

        // impl<const N: usize, const M: usize> FromLeBytes<N> for [u16;M] {
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



macro_rules! bit_numbering_alphabet {
    ($sb:ident => $len:literal: $($cl:ident),+ $(,)?) => { paste!{
        // impl<TY, A, .. , const AN: usize, .. > FromLsb<P#<TY, .. >> for (A, .. )
        impl<TY, $($cl,)+ $(const [<$cl N>]: usize,)+> [<From $sb>]<[<P $len>]<TY, $([<$cl N>],)+>> for ($($cl,)+)
        where
            TY: Integral $(+ AsPrimitive<$cl>)+,
        {
            const BITS: usize = TY::BITS as usize;
            const MAX_BIT_INDEX: usize = 0 $(+ [<$cl N>])+;
            fn [<from_ $sb:lower>]([<P $len>](_data): [<P $len>]<TY, $([<$cl N>],)+>) -> Self {
                #![allow(path_statements)]
                <Self as [<From $sb>]<[<P $len>]<TY, $([<$cl N>],)+>>>::ASSERT_INDEX_IN_BOUNDS;

                $(let ([<$cl:lower>], _data) = [<$sb:lower _split>]::<_, [<$cl N>]>(_data);)+
                ($([<$cl:lower>].as_primitive(),)+)
            }
        }

        // impl<T, U, const AN: usize, .. > From<P3<T, AN, .. >> for Lsb<U>
        impl<T, U, $(const [<$cl N>]: usize,)+> From<[<P $len>]<T, $([<$cl N>],)+>> for $sb<U>
        where
            U: [<From $sb>]<[<P $len>]<T,$([<$cl N>],)+>>,
        {
            fn from(data: [<P $len>]<T,$([<$cl N>],)+>) -> Self {
                Self(data.[<$sb:lower _into>]())
            }
        }
    }};
    ($len:literal: $($cl:ident),+ $(,)?) => {
        bit_numbering_alphabet!(Lsb => $len: $($cl),+);
        bit_numbering_alphabet!(Msb => $len: $($cl),+);
    };
}
