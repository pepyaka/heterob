use criterion::{black_box, criterion_group, criterion_main, Criterion};
use heterob::{bit_numbering::LsbInto, P1, P26, P7};

const DATA: u64 = 0x0123456789ABCDEF;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("LsbInto: P1", |b| {
        b.iter(|| {
            let (a,) = P1::<u64, 63>(black_box(DATA)).lsb_into();
            let _: u128 = a;
        })
    });
    c.bench_function("LsbInto: P7", |b| {
        b.iter(|| {
            let (a, b, c, d, e, f, g) = P7::<u64, 1, 2, 3, 4, 5, 6, 7>(black_box(DATA)).lsb_into();
            let _: (bool, u8, u16, u32, u64, u128, usize) = (a, b, c, d, e, f, g);
        })
    });
    c.bench_function("LsbInto: P26", |b| {
        b.iter(|| {
            let (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z) =
                P26::<u64,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,>(black_box(DATA)).lsb_into();
            let _: [bool; 26] = [
                a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z,
            ];
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
