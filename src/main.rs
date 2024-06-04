#![allow(dead_code)]
#![allow(non_snake_case)]
use bumpalo::Bump;
use concat_idents::concat_idents;

mod types;
use types::*;

use criterion::*;

const ILP: usize = 8;

macro_rules! bench_all {
    ($test:ident, $gen:ident, $c:ident) => {
        bench_all!($test, $test, $gen, $c);
    };
    ($name:ident, $test:ident, $gen:ident, $c:ident) => {{
        let mut group = $c.benchmark_group(stringify!($name));
        let bump = Bump::new();
        let tagged = black_box($gen(&bump));
        group.bench_function("baseline", |b| b.iter(|| $test::<types::BaseLine>(&tagged)));

        let tagged = black_box($gen(&bump));
        group.bench_function("low_bits", |b| b.iter(|| $test::<LowBits<_>>(&tagged)));

        let tagged = black_box($gen(&bump));
        group.bench_function("low_byte", |b| b.iter(|| $test::<LowByte<_>>(&tagged)));

        let tagged = black_box($gen(&bump));
        group.bench_function("high_bits", |b| b.iter(|| $test::<HighBits<_>>(&tagged)));

        let tagged = black_box($gen(&bump));
        group.bench_function("high_byte", |b| b.iter(|| $test::<HighByte<_>>(&tagged)));
    }};
}

macro_rules! gen {
    ($variant:ident) => {
        concat_idents!(fn_name = gen_, $variant {
            fn fn_name<T: TaggedPointer<Basic> + Clone>(bump: &Bump) -> Vec<T> {
                let basic = Basic::$variant(bump.alloc(X::new(37)));
                vec![T::new(basic); 10000]
            }
        });

        concat_idents!(fn_name = sum_, $variant {
            fn fn_name<T: TaggedPointer<Basic>>(x: &[T]) -> i32 {
                sum(x, |i| match i.untag() {
                    Basic::$variant(x) => unsafe { (*x).data },
                    _ => 0,
                })
            }
        });

        concat_idents!(fn_name = count_, $variant {
            fn fn_name<T: TaggedPointer<Basic>>(x: &[T]) -> i32 {
                count(x, |i| matches!(i.untag(), Basic::$variant(_)))
            }
        });
    }
}

gen!(T0);
gen!(T1);
gen!(T2);
gen!(T3);
gen!(T4);
gen!(T5);
gen!(T6);
gen!(T7);

macro_rules! gen2 {
    ($var1:ident, $var2:ident) => {
        concat_idents!(fn_name = gen_, $var1, _, $var2 {
            fn fn_name<T: TaggedPointer<Basic> + Clone>(bump: &Bump) -> Vec<T> {
                let t0 = Basic::$var1(bump.alloc(X::new(37)));
                let t1 = Basic::$var2(bump.alloc(X::new(33)));
                let mut vec = Vec::new();
                for _ in 0..5000 {
                    vec.push(T::new(t0));
                    vec.push(T::new(t1));
                }
                vec
            }
        });

        concat_idents!(fn_name = sum_, $var1, _, $var2 {
            fn fn_name<T: TaggedPointer<Basic>>(x: &[T]) -> i32 {
                sum(x, |i| match i.untag() {
                    Basic::$var1(x) => unsafe { (*x).data },
                    Basic::$var2(x) => unsafe { (*x).data },
                    _ => 0,
                })
            }
        });

        concat_idents!(fn_name = count_, $var1, _, $var2 {
            fn fn_name<T: TaggedPointer<Basic>>(x: &[T]) -> i32 {
                count(x, |i| matches!(i.untag(), Basic::$var1(_) | Basic::$var2(_)))
            }
        });
    }
}

gen2!(T0, T1);
gen2!(T0, T2);
gen2!(T1, T2);
gen2!(T1, T3);

macro_rules! gen3 {
    ($var1:ident, $var2:ident, $var3:ident) => {
        concat_idents!(fn_name = gen_, $var1, _, $var2, _, $var3 {
            fn fn_name<T: TaggedPointer<Basic> + Clone>(bump: &Bump) -> Vec<T> {
                let t0 = Basic::$var1(bump.alloc(X::new(37)));
                let t1 = Basic::$var2(bump.alloc(X::new(33)));
                let t2 = Basic::$var3(bump.alloc(X::new(17)));
                let mut vec = Vec::new();
                for _ in 0..3333 {
                    vec.push(T::new(t0));
                    vec.push(T::new(t1));
                    vec.push(T::new(t2));
                }
                vec
            }
        });

        concat_idents!(fn_name = sum_, $var1, _, $var2, _, $var3 {
            fn fn_name<T: TaggedPointer<Basic>>(x: &[T]) -> i32 {
                sum(x, |i| match i.untag() {
                    Basic::$var1(x) => unsafe { (*x).data },
                    Basic::$var2(x) => unsafe { (*x).data },
                    Basic::$var3(x) => unsafe { (*x).data },
                    _ => 0,
                })
            }
        });

        concat_idents!(fn_name = count_, $var1, _, $var2, _, $var3 {
            fn fn_name<T: TaggedPointer<Basic>>(x: &[T]) -> i32 {
                count(x, |i| matches!(i.untag(), Basic::$var1(_) | Basic::$var2(_) | Basic::$var3(_)))
            }
        });
    }
}

gen3!(T0, T1, T2);
gen3!(T0, T2, T4);
gen3!(T1, T2, T3);
gen3!(T1, T3, T5);

fn sum<T: TaggedPointer<Basic>>(x: &[T], f: impl Fn(&T) -> i32) -> i32 {
    let mut sum = 0;
    for i in x {
        sum += f(i);
    }
    sum
}

fn count<T: TaggedPointer<Basic>>(x: &[T], f: impl Fn(&T) -> bool) -> i32 {
    let mut sum = 0;
    for i in x {
        if f(i) {
            sum += 1;
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

fn call7<T: TaggedPointer<Basic> + Copy>(x: &(Vec<T>, fn(T, T, T, T, T, T, T) -> i32)) -> i32 {
    let mut sum: i32 = 0;
    let f = x.1;
    let x = &x.0;
    for i in x {
        sum += f(*i, *i, *i, *i, *i, *i, *i);
    }
    sum
}

fn call8<T: TaggedPointer<Basic> + Copy>(x: &(Vec<T>, fn(T, T, T, T, T, T, T, T) -> i32)) -> i32 {
    let mut sum: i32 = 0;
    let f = x.1;
    let x = &x.0;
    for i in x {
        sum += f(*i, *i, *i, *i, *i, *i, *i, *i);
    }
    sum
}

fn count_t0_to_t3<T: TaggedPointer<Basic>>(x: &[T]) -> i32 {
    let mut sum: i32 = 0;
    for i in x {
        if matches!(
            i.untag(),
            Basic::T0(_) | Basic::T1(_) | Basic::T2(_) | Basic::T3(_)
        ) {
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

fn gen_t0_set<T: TaggedPointer<Basic> + Copy>(bump: &Bump) -> Vec<[T; 8]> {
    let basic = Basic::T0(bump.alloc(X::new(37)));
    let array = [T::new(basic); 8];
    vec![array; 10000]
}

fn gen_t1_call7<T: TaggedPointer<Basic> + Clone + Copy>(
    bump: &Bump,
) -> (Vec<T>, fn(T, T, T, T, T, T, T) -> i32) {
    let basic = Basic::T1(bump.alloc(X::new(37)));
    (
        vec![T::new(basic); 10000],
        black_box(|_, _, _, _, _, _, _| 13),
    )
}

fn gen_t1_call8<T: TaggedPointer<Basic> + Clone + Copy>(
    bump: &Bump,
) -> (Vec<T>, fn(T, T, T, T, T, T, T, T) -> i32) {
    let basic = Basic::T1(bump.alloc(X::new(37)));
    (
        vec![T::new(basic); 10000],
        black_box(|_, _, _, _, _, _, _, _| 13),
    )
}

fn all_benches(c: &mut Criterion) {
    bench_all!(sum_T0, gen_T0, c);
    bench_all!(sum_T1, gen_T1, c);
    bench_all!(count_T0, gen_T0, c);
    bench_all!(count_T1, gen_T1, c);
    bench_all!(count_T7, gen_T7, c);
    bench_all!(sum_T0_T1, gen_T0_T1, c);
    bench_all!(sum_T0_T2, gen_T0_T1, c);
    bench_all!(sum_T1_T2, gen_T1_T2, c);
    bench_all!(sum_T1_T3, gen_T1_T3, c);
    bench_all!(count_T0_T1, gen_T0_T1, c);
    bench_all!(count_T0_T2, gen_T0_T1, c);
    bench_all!(count_T1_T2, gen_T1_T2, c);
    bench_all!(count_T1_T3, gen_T1_T3, c);
    bench_all!(sum_T0_T1_T2, gen_T0_T1_T2, c);
    bench_all!(sum_T0_T2_T4, gen_T0_T2_T4, c);
    bench_all!(sum_T1_T2_T3, gen_T1_T2_T3, c);
    bench_all!(sum_T1_T3_T5, gen_T1_T3_T5, c);
    bench_all!(count_T0_T1_T2, gen_T0_T1_T2, c);
    bench_all!(count_T0_T2_T4, gen_T0_T2_T4, c);
    bench_all!(count_T1_T2_T3, gen_T1_T2_T3, c);
    bench_all!(count_T1_T3_T5, gen_T1_T3_T5, c);

    bench_all!(count_t0_to_t3, gen_T1_T2, c);
    bench_all!(sum_chunk_t0, gen_t0_set, c);
    bench_all!(call7, gen_t1_call7, c);
    bench_all!(call8, gen_t1_call8, c);
}

criterion_group!(benches, all_benches);

fn main() {
    benches();
    Criterion::default().configure_from_args().final_summary();

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
    count_T0(x)
}

#[inline(never)]
pub fn count_low(x: &[LowBits<Basic>]) -> i32 {
    count_T0(x)
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
