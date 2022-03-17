// use titm::{le::IntoLe, T3,le::Le,be::Be};

// #[test]
// fn it_works() {
//     #[derive(Debug, PartialEq, Eq)]
//     struct Stest0(u8);
//     impl From<[u8; 1]> for Le<Stest0> {
//         fn from(_: [u8; 1]) -> Self { todo!() }
//     }
//     #[derive(Debug, PartialEq, Eq)]
//     struct S {
//         a: u8,
//         b: u16,
//         c: Stest0,
//     }

//     let Le((a,b,c)) = T3::from([3,5,4,5,]).into();
//     let s = S { a,b,c, };
//     assert_eq!(S {a:[3].into_le(),b:4,c:Stest0(23)}, s);
//     let (Le(a),Be(b),Le(c)) = T3::from([3,5,4,5,]).into();
//     let s = S { a,b,c, };
//     assert_eq!(S {a:3,b:4,c:Stest0(11)}, s);
// }
