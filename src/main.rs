#![allow(dead_code)]
use bumpalo::Bump;

mod types;
use types::*;

use criterion::*;

#[inline(never)]
pub fn sum_byte(x: &[LowByte<Basic>]) -> i32 {
    let mut sum: i32 = 0;
    for i in x {
        if let Basic::V1(x) = i.untag() {
            sum = sum.wrapping_add(unsafe { (*x).data })
        }
    }
    sum
}

#[inline(never)]
pub fn sum_bit(x: &[LowBits<Basic>]) -> i32 {
    let mut sum: i32 = 0;
    for i in x {
        if let Basic::V1(x) = i.untag() {
            sum = sum.wrapping_add(unsafe { (*x).data })
        }
    }
    sum
}

#[inline(never)]
pub fn untag_byte(x: LowByte<Basic>) -> i32 {
    if let Basic::V1(x) = x.untag() {
        unsafe { (*x).data }
    } else {
        13
    }
}

#[inline(never)]
pub fn untag_bit(x: LowBits<Basic>) -> i32 {
    if let Basic::V1(x) = x.untag() {
        unsafe { (*x).data }
    } else {
        13
    }
}

fn sum_v1<T: TaggedPointer<Basic>>(x: &[T]) -> i32 {
    let mut sum: i32 = 0;
    for i in x {
        if let Basic::V1(x) = i.untag() {
            sum = sum.wrapping_add(unsafe { (*x).data })
        }
    }
    sum
}

fn sum_v2<T: TaggedPointer<Basic>>(x: &[T]) -> i32 {
    let mut sum: i32 = 0;
    for i in x {
        if let Basic::V2(x) = i.untag() {
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

fn count_v1<T: TaggedPointer<Basic>>(x: &[T]) -> i32 {
    let mut sum: i32 = 0;
    for i in x {
        if matches!(i.untag(), Basic::V1(_)) {
            sum += 1;
        }
    }
    sum
}

fn count_v2<T: TaggedPointer<Basic>>(x: &[T]) -> i32 {
    let mut sum: i32 = 0;
    for i in x {
        if matches!(i.untag(), Basic::V2(_)) {
            sum += 1;
        }
    }
    sum
}

fn sum_all<T: TaggedPointer<Basic>>(x: &[T]) -> i32 {
    let mut sum: i32 = 0;
    for i in x {
        match i.untag() {
            Basic::V1(x) => sum = sum.wrapping_add(unsafe { (*x).data }),
            Basic::V2(x) => sum = sum.wrapping_add(unsafe { (*x).data }),
            Basic::V3(x) => sum = sum.wrapping_add(unsafe { (*x).data }),
            Basic::V4(x) => sum = sum.wrapping_add(unsafe { (*x).data }),
            Basic::V5(x) => sum = sum.wrapping_add(unsafe { (*x).data }),
            Basic::V6(x) => sum = sum.wrapping_add(unsafe { (*x).data }),
            Basic::V7(x) => sum = sum.wrapping_add(unsafe { (*x).data }),
            Basic::V8(x) => sum = sum.wrapping_add(unsafe { (*x).data }),
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
            0 => Basic::V1(bump.alloc(X::new(rand as i32))),
            1 => Basic::V2(bump.alloc(X::new(rand as i32))),
            2 => Basic::V3(bump.alloc(X::new(rand as i32))),
            3 => Basic::V4(bump.alloc(X::new(rand as i32))),
            4 => Basic::V5(bump.alloc(X::new(rand as i32))),
            5 => Basic::V6(bump.alloc(X::new(rand as i32))),
            6 => Basic::V7(bump.alloc(X::new(rand as i32))),
            7 => Basic::V8(bump.alloc(X::new(rand as i32))),
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
        let basic = Basic::V1(bump.alloc(X::new(rand as i32)));
        x.push(basic);
    }
    x
}

fn gen_v1(bump: &Bump) -> Vec<Basic> {
    // generate a vector of random Basic Values
    let mut x = Vec::new();
    let basic = Basic::V1(bump.alloc(X::new(37)));
    for _ in 0..10000 {
        x.push(basic);
    }
    x
}

fn gen_v2(bump: &Bump) -> Vec<Basic> {
    // generate a vector of random Basic Values
    let mut x = Vec::new();
    let basic = Basic::V2(bump.alloc(X::new(37)));
    for _ in 0..10000 {
        x.push(basic);
    }
    x
}

fn bench_v1_v1(c: &mut Criterion) {
    let mut group = c.benchmark_group("sum_v1_v1");
    let bump = Bump::new();
    let data = black_box(gen_v1(&bump));
    let tagged = data.iter().map(|x| LowBits::new(*x)).collect::<Vec<_>>();
    group.bench_function("low_bits", |b| b.iter(|| sum_v1(&tagged)));

    let tagged = data.iter().map(|x| LowByte::new(*x)).collect::<Vec<_>>();
    group.bench_function("low_byte", |b| b.iter(|| sum_v1(&tagged)));

    let tagged = data.iter().map(|x| HighBits::new(*x)).collect::<Vec<_>>();
    group.bench_function("high_bits", |b| b.iter(|| sum_v1(&tagged)));

    let tagged = data.iter().map(|x| HighByte::new(*x)).collect::<Vec<_>>();
    group.bench_function("high_byte", |b| b.iter(|| sum_v1(&tagged)));
}

fn bench_v2_v2(c: &mut Criterion) {
    let mut group = c.benchmark_group("sum_v2_v2");
    let bump = Bump::new();
    let data = black_box(gen_v2(&bump));
    let tagged = data.iter().map(|x| LowBits::new(*x)).collect::<Vec<_>>();
    group.bench_function("low_bits", |b| b.iter(|| sum_v2(&tagged)));

    let tagged = data.iter().map(|x| LowByte::new(*x)).collect::<Vec<_>>();
    group.bench_function("low_byte", |b| b.iter(|| sum_v2(&tagged)));

    let tagged = data.iter().map(|x| HighBits::new(*x)).collect::<Vec<_>>();
    group.bench_function("high_bits", |b| b.iter(|| sum_v2(&tagged)));

    let tagged = data.iter().map(|x| HighByte::new(*x)).collect::<Vec<_>>();
    group.bench_function("high_byte", |b| b.iter(|| sum_v2(&tagged)));
}

fn all_benches(c: &mut Criterion) {
    bench_v1_v1(c);
    bench_v2_v2(c);
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
// }

// #[inline(never)]
// pub fn untag_bit0(x: LowByte<Basic>) -> i32 {
//     if let Basic::V1(x) = x.untag() {
//         unsafe { (*x).data }
//     } else {
//         13
//     }
// }

// #[inline(never)]
// pub fn untag_bit1(x: LowByte<Basic>) -> i32 {
//     if let Basic::V2(x) = x.untag() {
//         unsafe { (*x).data }
//     } else {
//         13
//     }
// }
