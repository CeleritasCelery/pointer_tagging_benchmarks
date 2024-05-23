#![allow(dead_code)]
use bumpalo::Bump;

mod types;
use types::*;

use criterion::*;

const ILP: usize = 8;

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

fn sum_chunk_t0<T: TaggedPointer<Basic>>(x: &[[T; ILP]]) -> i32 {
    let mut sum: i32 = 0;
    for i in x {
        if let Basic::T0(x) = i[0].untag() {
            sum = sum.wrapping_add(unsafe { (*x).data })
        }
        if let Basic::T0(x) = i[1].untag() {
            sum = sum.wrapping_sub(unsafe { (*x).data })
        }
        if let Basic::T0(x) = i[2].untag() {
            sum = sum.wrapping_add(unsafe { (*x).data })
        }
        if let Basic::T0(x) = i[3].untag() {
            sum = sum.wrapping_sub(unsafe { (*x).data })
        }
        if let Basic::T0(x) = i[4].untag() {
            sum = sum.wrapping_add(unsafe { (*x).data })
        }
        if let Basic::T0(x) = i[5].untag() {
            sum = sum.wrapping_sub(unsafe { (*x).data })
        }
        if let Basic::T0(x) = i[6].untag() {
            sum = sum.wrapping_sub(unsafe { (*x).data })
        }
        if let Basic::T0(x) = i[7].untag() {
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

fn sum_t2<T: TaggedPointer<Basic>>(x: &[T]) -> i32 {
    let mut sum: i32 = 0;
    for i in x {
        if let Basic::T2(x) = i.untag() {
            sum = sum.wrapping_add(unsafe { (*x).data })
        }
    }
    sum
}

pub fn sum_t1_t2<T: TaggedPointer<Basic>>(x: &[T]) -> i32 {
    let mut sum: i32 = 0;
    for i in x {
        match i.untag() {
            Basic::T1(x) => sum = sum.wrapping_add(unsafe { (*x).data }),
            Basic::T2(x) => sum = sum.wrapping_add(unsafe { (*x).data }),
            _ => {}
        }
    }
    sum
}

pub fn sum_t1_t2_raw<T: TaggedPointer<Basic>>(x: &[T]) -> i32 {
    let mut sum: i32 = 0;
    for i in x {
        match i.tag() {
            1 => sum = sum.wrapping_add(unsafe { (*i.data().cast::<X<7>>()).data }),
            2 => sum = sum.wrapping_sub(unsafe { (*i.data().cast::<X<4>>()).data }),
            _ => {}
        }
    }
    sum
}

fn count_t0<T: TaggedPointer<Basic>>(x: &[T]) -> i32 {
    let mut sum: i32 = 0;
    for i in x {
        if matches!(i.untag(), Basic::T0(_)) {
            sum += 1;
        }
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

fn count_t0_t1<T: TaggedPointer<Basic>>(x: &[T]) -> i32 {
    let mut sum: i32 = 0;
    for i in x {
        if matches!(i.untag(), Basic::T0(_) | Basic::T1(_)) {
            sum += 1;
        }
    }
    sum
}

fn count_t1_t2<T: TaggedPointer<Basic>>(x: &[T]) -> i32 {
    let mut sum: i32 = 0;
    for i in x {
        if matches!(i.untag(), Basic::T1(_) | Basic::T2(_)) {
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

fn gen_tagged<T: TaggedPointer<Basic> + Clone>(basic: Basic) -> Vec<T> {
    vec![T::new(basic); 10000]
}

fn gen_t0<T: TaggedPointer<Basic> + Clone>(bump: &Bump) -> Vec<T> {
    let basic = Basic::T0(bump.alloc(X::new(37)));
    vec![T::new(basic); 10000]
}

fn gen_t1<T: TaggedPointer<Basic> + Clone>(bump: &Bump) -> Vec<T> {
    let basic = Basic::T1(bump.alloc(X::new(37)));
    vec![T::new(basic); 10000]
}

fn gen_t1_t2<T: TaggedPointer<Basic> + Clone>(bump: &Bump) -> Vec<T> {
    let t1 = Basic::T1(bump.alloc(X::new(37)));
    let t2 = Basic::T2(bump.alloc(X::new(33)));
    let mut vec = Vec::new();
    for _ in 0..5000 {
        vec.push(T::new(t1));
        vec.push(T::new(t2));
    }
    vec
}

fn gen_t0_set<T: TaggedPointer<Basic> + Copy>(bump: &Bump) -> Vec<[T; 8]> {
    let basic = Basic::T0(bump.alloc(X::new(37)));
    let array = [T::new(basic); 8];
    vec![array; 10000]
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

fn bench_count_t1_t2(c: &mut Criterion) {
    let mut group = c.benchmark_group("count_t1_t2");
    let bump = Bump::new();
    let tagged = black_box(gen_t1_t2(&bump));
    group.bench_function("low_bits", |b| b.iter(|| count_t1_t2::<LowBits<_>>(&tagged)));

    let tagged = black_box(gen_t1(&bump));
    group.bench_function("low_byte", |b| b.iter(|| count_t1_t2::<LowByte<_>>(&tagged)));

    let tagged = black_box(gen_t1(&bump));
    group.bench_function("high_bits", |b| b.iter(|| count_t1_t2::<HighBits<_>>(&tagged)));

    let tagged = black_box(gen_t1(&bump));
    group.bench_function("high_byte", |b| b.iter(|| count_t1_t2::<HighByte<_>>(&tagged)));
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

fn bench_sum_t2(c: &mut Criterion) {
    let mut group = c.benchmark_group("sum_t2");
    let x = X::new(37);
    let basic = Basic::T2(&x);
    let tagged = black_box(gen_tagged(basic));
    group.bench_function("low_bits", |b| b.iter(|| sum_t2::<LowBits<_>>(&tagged)));

    let tagged = black_box(gen_tagged(basic));
    group.bench_function("low_byte", |b| b.iter(|| sum_t2::<LowByte<_>>(&tagged)));

    let tagged = black_box(gen_tagged(basic));
    group.bench_function("high_bits", |b| b.iter(|| sum_t2::<HighBits<_>>(&tagged)));

    let tagged = black_box(gen_tagged(basic));
    group.bench_function("high_byte", |b| b.iter(|| sum_t2::<HighByte<_>>(&tagged)));
}

fn bench_sum_t1_t2(c: &mut Criterion) {
    let mut group = c.benchmark_group("sum_t1_t2");
    let bump = Bump::new();
    let tagged = black_box(gen_t1_t2(&bump));
    group.bench_function("low_bits", |b| b.iter(|| sum_t1_t2::<LowBits<_>>(&tagged)));
    group.bench_function("low_bits_raw", |b| {
        b.iter(|| sum_t1_t2_raw::<LowBits<_>>(&tagged))
    });

    let tagged = black_box(gen_t1_t2(&bump));
    group.bench_function("low_byte_raw", |b| {
        b.iter(|| sum_t1_t2_raw::<LowByte<_>>(&tagged))
    });
    group.bench_function("low_byte", |b| b.iter(|| sum_t1_t2::<LowByte<_>>(&tagged)));

    let tagged = black_box(gen_t1_t2(&bump));
    group.bench_function("high_bits", |b| {
        b.iter(|| sum_t1_t2::<HighBits<_>>(&tagged))
    });

    let tagged = black_box(gen_t1_t2(&bump));
    group.bench_function("high_byte", |b| {
        b.iter(|| sum_t1_t2::<HighByte<_>>(&tagged))
    });
}

fn bench_ilp_t0(c: &mut Criterion) {
    let mut group = c.benchmark_group("ilp_t0");
    let bump = Bump::new();
    let tagged = black_box(gen_t0_set(&bump));
    group.bench_function("low_bits", |b| {
        b.iter(|| sum_chunk_t0::<LowBits<_>>(&tagged))
    });

    let tagged = black_box(gen_t0_set(&bump));
    group.bench_function("low_byte", |b| {
        b.iter(|| sum_chunk_t0::<LowByte<_>>(&tagged))
    });

    let tagged = black_box(gen_t0_set(&bump));
    group.bench_function("high_bits", |b| {
        b.iter(|| sum_chunk_t0::<HighBits<_>>(&tagged))
    });

    let tagged = black_box(gen_t0_set(&bump));
    group.bench_function("high_byte", |b| {
        b.iter(|| sum_chunk_t0::<HighByte<_>>(&tagged))
    });
}

fn all_benches(c: &mut Criterion) {
    bench_sum_t0(c);
    bench_sum_t1(c);
    bench_sum_t2(c);
    bench_sum_t1_t2(c);
    bench_count_t0(c);
    bench_count_t1(c);
    bench_count_t1_t2(c);
    bench_ilp_t0(c);
}

criterion_group!(benches, all_benches);
fn main() {
    benches();
    Criterion::default().configure_from_args().final_summary();

    println!("All tests passed");
    let i = &X::new(13);
    let x = LowByte::new(Basic::T1(i));
    black_box(untag_bit0(x));
    let i = X::new(45);
    let x = LowByte::new(Basic::T2(&i));
    black_box(untag_enum_2(&[x]));
    black_box(untag_raw_2(&[x]));
    black_box(untag_bit1(x));
    let i = &X::new(13);
    let x = LowBits::new(Basic::T1(i));
    black_box(count_low(black_box(&[x])));
    let x = HighBits::new(Basic::T1(i));
    black_box(count_high(black_box(&[x])));
}

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

#[inline(never)]
pub fn untag_enum_2(x: &[LowByte<Basic>]) -> i32 {
    let mut sum: i32 = 0;
    for i in x {
        match i.untag() {
            Basic::T1(x) => sum = sum.wrapping_add(unsafe { (*x).data }),
            Basic::T2(x) => sum = sum.wrapping_sub(unsafe { (*x).data }),
            _ => {}
        }
    }
    sum
}

#[inline(never)]
pub fn untag_raw_2(x: &[LowByte<Basic>]) -> i32 {
    let mut sum: i32 = 0;
    for i in x {
        match i.tag() {
            1 => sum = sum.wrapping_add(unsafe { (*i.data().cast::<X<7>>()).data }),
            2 => sum = sum.wrapping_sub(unsafe { (*i.data().cast::<X<4>>()).data }),
            _ => {}
        }
    }
    sum
}
