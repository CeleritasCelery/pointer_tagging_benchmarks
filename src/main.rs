#![allow(dead_code)]
use bumpalo::Bump;

mod types;
use types::*;

use criterion::*;

#[inline(never)]
pub fn sum_byte(x: &[LowByte<Basic>]) -> i32 {
    let mut sum: i32 = 0;
    for i in x {
        if let Basic::T0(x) = i.untag() {
            sum = sum.wrapping_add(unsafe { (*x).data })
        }
    }
    sum
}

#[inline(never)]
pub fn sum_bit(x: &[LowBits<Basic>]) -> i32 {
    let mut sum: i32 = 0;
    for i in x {
        if let Basic::T0(x) = i.untag() {
            sum = sum.wrapping_add(unsafe { (*x).data })
        }
    }
    sum
}

#[inline(never)]
pub fn untag_byte(x: LowByte<Basic>) -> i32 {
    if let Basic::T0(x) = x.untag() {
        unsafe { (*x).data }
    } else {
        13
    }
}

#[inline(never)]
pub fn untag_bit(x: LowBits<Basic>) -> i32 {
    if let Basic::T0(x) = x.untag() {
        unsafe { (*x).data }
    } else {
        13
    }
}

fn sum_t0<T: TaggedPointer<Basic>>(x: &[T]) -> i32 {
    let mut sum: i32 = 0;
    for i in x {
        if let Basic::T0(x) = i.untag() {
            sum = sum.wrapping_add(unsafe { (*x).data })
        }
    }
    sum
}

fn sum_t1<T: TaggedPointer<Basic>>(x: &[T]) -> i32 {
    let mut sum: i32 = 0;
    for i in x {
        if let Basic::T1(x) = i.untag() {
            sum = sum.wrapping_add(unsafe { (*x).data })
        }
    }
    sum
}

fn sum_v2_raw<T: TaggedPointer<Basic>>(x: &[T]) -> i32 {
    let mut sum: i32 = 0;
    for i in x {
        if 1 == i.tag() {
            sum = sum.wrapping_add(unsafe { (*i.data().cast::<X<7>>()).data })
        }
    }
    sum
}

// fn sum_raw<T: TaggedPointer<Basic>>(x: &[T]) -> i32 {
//     let mut sum: i32 = 0;
//     for i in x {
//         match i.tag() {
//             0 => sum = sum.wrapping_add(unsafe { (*i.data().cast::<X<0>>()).data }),
//             // 1 => sum = sum.wrapping_add(unsafe { (*i.data().cast::<X<7>>()).data }),
//             // 2 => sum = sum.wrapping_add(unsafe { (*i.data().cast::<X<4>>()).data }),
//             // 3 => sum = sum.wrapping_add(unsafe { (*i.data().cast::<X<6>>()).data }),
//             _ => {},
//             // 4 => sum = sum.wrapping_add(unsafe { (*i.data().cast::<X<2>>()).data }),
//             // 5 => sum = sum.wrapping_add(unsafe { (*i.data().cast::<X<5>>()).data }),
//             // 6 => sum = sum.wrapping_add(unsafe { (*i.data().cast::<X<3>>()).data }),
//             // 7 => sum = sum.wrapping_add(unsafe { (*i.data().cast::<X<1>>()).data }),
//             // _ => unsafe { unreachable_unchecked() },
//         }
//     }
//     sum
// }

fn count_t0<T: TaggedPointer<Basic>>(x: &[T]) -> i32 {
    let mut sum: i32 = 0;
    for i in x {
        if i.tag() == 0 {
            sum += 1;
        }
        // if matches!(i.untag(), Basic::V1(_)) {
        //     sum += 1;
        // }
    }
    sum
}

fn count_t1<T: TaggedPointer<Basic>>(x: &[T]) -> i32 {
    let mut sum: i32 = 0;
    for i in x {
        if matches!(i.untag(), Basic::T1(_)) {
            sum += 1;
        }
    }
    sum
}

fn sum_all<T: TaggedPointer<Basic>>(x: &[T]) -> i32 {
    let mut sum: i32 = 0;
    for i in x {
        match i.untag() {
            Basic::T0(x) => sum = sum.wrapping_add(unsafe { (*x).data }),
            Basic::T1(x) => sum = sum.wrapping_add(unsafe { (*x).data }),
            Basic::T2(x) => sum = sum.wrapping_add(unsafe { (*x).data }),
            Basic::T3(x) => sum = sum.wrapping_add(unsafe { (*x).data }),
            Basic::T4(x) => sum = sum.wrapping_add(unsafe { (*x).data }),
            Basic::T5(x) => sum = sum.wrapping_add(unsafe { (*x).data }),
            Basic::T6(x) => sum = sum.wrapping_add(unsafe { (*x).data }),
            Basic::T7(x) => sum = sum.wrapping_add(unsafe { (*x).data }),
        }
    }
    sum
}

fn gen_basic_data(bump: &Bump) -> Vec<Basic> {
    // generate a vector of random Basic Values
    let mut x = Vec::new();
    for _ in 0..10000 {
        // generate a random number between 0 and 7 using rand crate
        let rand = rand::random::<u32>();
        let variant = rand % 8;
        let basic = match variant {
            0 => Basic::T0(bump.alloc(X::new(rand as i32))),
            1 => Basic::T1(bump.alloc(X::new(rand as i32))),
            2 => Basic::T2(bump.alloc(X::new(rand as i32))),
            3 => Basic::T3(bump.alloc(X::new(rand as i32))),
            4 => Basic::T4(bump.alloc(X::new(rand as i32))),
            5 => Basic::T5(bump.alloc(X::new(rand as i32))),
            6 => Basic::T6(bump.alloc(X::new(rand as i32))),
            7 => Basic::T7(bump.alloc(X::new(rand as i32))),
            _ => unreachable!(),
        };
        x.push(basic);
    }
    x
}

fn gen_predictable_data(bump: &Bump) -> Vec<Basic> {
    // generate a vector of random Basic Values
    let mut x = Vec::new();
    for _ in 0..10000 {
        // generate a random number between 0 and 7 using rand crate
        let rand = rand::random::<u32>();
        let basic = Basic::T0(bump.alloc(X::new(rand as i32)));
        x.push(basic);
    }
    x
}

fn gen_t0<T: TaggedPointer<Basic> + Clone>(bump: &Bump) -> Vec<T> {
    let basic = Basic::T0(bump.alloc(X::new(37)));
    vec![T::new(basic); 10000]
}

fn gen_t1<T: TaggedPointer<Basic> + Clone>(bump: &Bump) -> Vec<T> {
    let basic = Basic::T1(bump.alloc(X::new(37)));
    vec![T::new(basic); 10000]
}

fn bench_sum_t0(c: &mut Criterion) {
    let mut group = c.benchmark_group("sum_t0");
    let bump = Bump::new();
    let tagged = black_box(gen_t0(&bump));
    group.bench_function("low_bits", |b| b.iter(|| sum_t0::<LowBits<_>>(&tagged)));

    let tagged = black_box(gen_t0(&bump));
    group.bench_function("low_byte", |b| b.iter(|| sum_t0::<LowByte<_>>(&tagged)));

    let tagged = black_box(gen_t0(&bump));
    group.bench_function("high_bits", |b| b.iter(|| sum_t0::<HighBits<_>>(&tagged)));

    let tagged = black_box(gen_t0(&bump));
    group.bench_function("high_byte", |b| b.iter(|| sum_t0::<HighByte<_>>(&tagged)));
}

fn bench_count_t0(c: &mut Criterion) {
    let mut group = c.benchmark_group("count_t0");
    let bump = Bump::new();
    let tagged = black_box(gen_t0(&bump));
    group.bench_function("low_bits", |b| b.iter(|| count_t0::<LowBits<_>>(&tagged)));

    let tagged = black_box(gen_t0(&bump));
    group.bench_function("low_byte", |b| b.iter(|| count_t0::<LowByte<_>>(&tagged)));

    let tagged = black_box(gen_t0(&bump));
    group.bench_function("high_bits", |b| b.iter(|| count_t0::<HighBits<_>>(&tagged)));

    let tagged = black_box(gen_t0(&bump));
    group.bench_function("high_byte", |b| b.iter(|| count_t0::<HighByte<_>>(&tagged)));
}

fn bench_count_t1(c: &mut Criterion) {
    let mut group = c.benchmark_group("count_t1");
    let bump = Bump::new();
    let tagged = black_box(gen_t1(&bump));
    group.bench_function("low_bits", |b| b.iter(|| count_t1::<LowBits<_>>(&tagged)));

    let tagged = black_box(gen_t1(&bump));
    group.bench_function("low_byte", |b| b.iter(|| count_t1::<LowByte<_>>(&tagged)));

    let tagged = black_box(gen_t1(&bump));
    group.bench_function("high_bits", |b| b.iter(|| count_t1::<HighBits<_>>(&tagged)));

    let tagged = black_box(gen_t1(&bump));
    group.bench_function("high_byte", |b| b.iter(|| count_t1::<HighByte<_>>(&tagged)));
}

fn bench_sum_t1(c: &mut Criterion) {
    let mut group = c.benchmark_group("sum_t1");
    let bump = Bump::new();
    let tagged = black_box(gen_t1(&bump));
    group.bench_function("low_bits", |b| b.iter(|| sum_t1::<LowBits<_>>(&tagged)));

    let tagged = black_box(gen_t1(&bump));
    group.bench_function("low_byte", |b| b.iter(|| sum_t1::<LowByte<_>>(&tagged)));

    let tagged = black_box(gen_t1(&bump));
    group.bench_function("high_bits", |b| b.iter(|| sum_t1::<HighBits<_>>(&tagged)));

    let tagged = black_box(gen_t1(&bump));
    group.bench_function("high_byte", |b| b.iter(|| sum_t1::<HighByte<_>>(&tagged)));
}

fn all_benches(c: &mut Criterion) {
    bench_sum_t0(c);
    bench_sum_t1(c);
    bench_count_t0(c);
    bench_count_t1(c);
}

criterion_group!(benches, all_benches);
criterion_main!(benches);

// fn main() {
//     // compare_sums();
//     println!("All tests passed");
//     let x = LowByte::new(Basic::V1(&X::new(13)));
//     let y = untag_bit0(x);
//     println!("y: {:?}", y);
//     let y = untag_bit1(x);
//     println!("y: {:?}", y);

//     let x = LowBits::new(Basic::V1(&X::new(13)));
//     let y = count_low(black_box(&[x]));
//     println!("y: {:?}", y);
//     let x = HighBits::new(Basic::V1(&X::new(13)));
//     let y = count_high(black_box(&[x]));
//     println!("y: {:?}", y);
// }

#[inline(never)]
pub fn untag_bit0(x: LowByte<Basic>) -> i32 {
    if let Basic::T0(x) = x.untag() {
        unsafe { (*x).data }
    } else {
        13
    }
}

#[inline(never)]
pub fn untag_bit1(x: LowByte<Basic>) -> i32 {
    if let Basic::T1(x) = x.untag() {
        unsafe { (*x).data }
    } else {
        13
    }
}

#[inline(never)]
pub fn check_bit0_high(x: HighBits<Basic>) -> bool {
    matches!(x.untag(), Basic::T0(_))
}

#[inline(never)]
pub fn check_bit0_low(x: LowByte<Basic>) -> bool {
    matches!(x.untag(), Basic::T0(_))
}

#[inline(never)]
pub fn count_high(x: &[HighBits<Basic>]) -> i32 {
    count_t0(x)
}

#[inline(never)]
pub fn count_low(x: &[LowBits<Basic>]) -> i32 {
    count_t0(x)
}
