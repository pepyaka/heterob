use heterob::{
    endianness::{Be, BeBytesTryInto, Le, LeBytesTryInto, TryFromLeBytes},
    Seq, P1, P2,
};

#[test]
fn array_from_slice() {
    let data = ['a'; 5].as_slice();
    let Seq { head: result, .. }: Seq<[char; 3], _> = data.try_into().unwrap();
    let sample: &[char] = data[..3].try_into().unwrap();
    assert_eq!(sample, result);
}

#[test]
fn array_from_slice_with_offset() {
    let data = [1, 2, 3, 4, 5].as_slice();

    // Explicit offset array
    let Seq {
        head: (_, result), ..
    }: Seq<([u8; 3], [u8; 2]), _> = P2(data).try_into().unwrap();
    assert_eq!([4, 5], result);

    // Using slice .get() method
    let Seq { head, .. }: Seq<[u8; 2], _> = data.get(3..).unwrap_or_default().try_into().unwrap();
    assert_eq!([4, 5], head);
}

#[test]
fn integer_from_slice() {
    let data = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55].as_slice();
    // We can't use TryInto trait directly like this
    // let Seq {
    //     head: Le(result), ..
    // }: Seq<Le<u32>, _> = data.try_into().unwrap();
    // But we can use LeBytesTryInto trait:
    let Seq { head: result, tail }: Seq<u32, _> = data.le_bytes_try_into().unwrap();
    assert_eq!(0x33221100, result);

    let Seq { head: result, .. } = tail.be_bytes_try_into().unwrap();
    assert_eq!(0x4455u16, result);
}

#[test]
fn single_integer_as_partable_from_slice() {
    let data = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55].as_slice();
    let Seq {
        head: Le((result,)),
        tail,
    } = P1(data).try_into().unwrap();
    assert_eq!(0x33221100u32, result);

    let Seq {
        head: Be((result,)),
        ..
    }: Seq<Be<(u16,)>, _> = P1(tail).try_into().unwrap();
    assert_eq!(0x4455, result);
}

#[test]
fn integers_array_from_slice() {
    let data = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55].as_slice();
    let Seq { head: result, .. }: Seq<[u16; 2], _> =
        // For array to array conversion we should use explicit array length
        TryFromLeBytes::<4>::try_from_le_bytes(data).unwrap();
    assert_eq!([0x1100, 0x3322], result);

    // Or we can use 2 step conversion
    let Seq { head, .. } = data.le_bytes_try_into().unwrap();
    let Le(result): Le<[u16; 2]> = From::<[u8; 4]>::from(head);
    assert_eq!([0x1100, 0x3322], result);
}
