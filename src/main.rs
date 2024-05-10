#![allow(dead_code)]
use bumpalo::Bump;
use std::{hint::unreachable_unchecked, marker::PhantomData};

use criterion::*;

trait Taggable {
    fn tag(&self) -> u8;
    fn ptr(&self) -> *const u8;
    fn from_raw(ptr: *const u8, tag: u8) -> Self;
}

trait TaggedPointer<T: Taggable>
where
    Self: Sized,
{
    fn new(val: T) -> Self {
        Self::from_raw(val.ptr(), val.tag())
    }
    fn untag(&self) -> T {
        T::from_raw(self.data(), self.tag())
    }
    fn from_raw(ptr: *const u8, tag: u8) -> Self;
    fn tag(&self) -> u8;
    fn data(&self) -> *const u8;
}

// Define a type with different offsets for the primary field
#[repr(C, align(8))]
#[derive(Debug)]
struct X<const N: usize> {
    _pad: [u32; N],
    data: i32,
}

impl<const N: usize> Default for X<N> {
    fn default() -> Self {
        Self {
            _pad: [0; N],
            data: 0,
        }
    }
}

impl<const N: usize> X<N> {
    fn new(data: i32) -> Self {
        Self { _pad: [0; N], data }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Basic {
    V1(*const X<0>) = 0,
    V2(*const X<7>) = 1,
    V3(*const X<4>) = 2,
    V4(*const X<6>) = 3,
    V5(*const X<2>) = 4,
    V6(*const X<5>) = 5,
    V7(*const X<3>) = 6,
    V8(*const X<1>) = 7,
}

impl Taggable for Basic {
    fn tag(&self) -> u8 {
        match self {
            Basic::V1(_) => 0,
            Basic::V2(_) => 1,
            Basic::V3(_) => 2,
            Basic::V4(_) => 3,
            Basic::V5(_) => 4,
            Basic::V6(_) => 5,
            Basic::V7(_) => 6,
            Basic::V8(_) => 7,
        }
    }

    fn ptr(&self) -> *const u8 {
        match self {
            Basic::V1(ptr) => *ptr as *const u8,
            Basic::V2(ptr) => *ptr as *const u8,
            Basic::V3(ptr) => *ptr as *const u8,
            Basic::V4(ptr) => *ptr as *const u8,
            Basic::V5(ptr) => *ptr as *const u8,
            Basic::V6(ptr) => *ptr as *const u8,
            Basic::V7(ptr) => *ptr as *const u8,
            Basic::V8(ptr) => *ptr as *const u8,
        }
    }

    fn from_raw(ptr: *const u8, tag: u8) -> Self {
        match tag {
            0 => Basic::V1(ptr as *const X<0>),
            1 => Basic::V2(ptr as *const X<7>),
            2 => Basic::V3(ptr as *const X<4>),
            3 => Basic::V4(ptr as *const X<6>),
            4 => Basic::V5(ptr as *const X<2>),
            5 => Basic::V6(ptr as *const X<5>),
            6 => Basic::V7(ptr as *const X<3>),
            7 => Basic::V8(ptr as *const X<1>),
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

struct LowBits<T> {
    data: *const u8,
    tag_type: PhantomData<T>,
}

impl<T: Taggable> TaggedPointer<T> for LowBits<T> {
    fn from_raw(ptr: *const u8, tag: u8) -> Self {
        let data = (ptr as usize | tag as usize) as *const u8;
        Self {
            data,
            tag_type: PhantomData,
        }
    }

    fn tag(&self) -> u8 {
        self.data as u8 & 0b111
    }

    fn data(&self) -> *const u8 {
        let mask = !0b111;
        (self.data as usize & mask) as *const u8
    }
}

struct LowByte<T> {
    data: *const u8,
    tag_type: PhantomData<T>,
}

impl<T: Taggable> TaggedPointer<T> for LowByte<T> {
    fn from_raw(ptr: *const u8, tag: u8) -> Self {
        let data = (((ptr as usize) << 8) | tag as usize) as *const u8;
        Self {
            data,
            tag_type: PhantomData,
        }
    }

    fn tag(&self) -> u8 {
        self.data as u8
    }

    fn data(&self) -> *const u8 {
        (self.data as usize >> 8) as *const u8
    }
}

struct HighBits<T> {
    data: *const u8,
    tag_type: PhantomData<T>,
}

impl<T> HighBits<T> {
    const BIT_SHIFT: usize = std::mem::size_of::<*const u8>() * 8 - 3;
}

impl<T: Taggable> TaggedPointer<T> for HighBits<T> {
    fn from_raw(ptr: *const u8, tag: u8) -> Self {
        let ptr = (ptr as usize) >> 3;
        let data = (ptr | (tag as usize) << Self::BIT_SHIFT) as *const u8;
        Self {
            data,
            tag_type: PhantomData,
        }
    }

    fn tag(&self) -> u8 {
        ((self.data as usize) >> Self::BIT_SHIFT) as u8
    }

    fn data(&self) -> *const u8 {
        ((self.data as usize) << 3) as *const u8
    }
}

struct HighByte<T> {
    data: *const u8,
    tag_type: PhantomData<T>,
}

impl<T> HighByte<T> {
    const BIT_SHIFT: usize = std::mem::size_of::<*const u8>() * 8 - 8;
}

impl<T: Taggable> TaggedPointer<T> for HighByte<T> {
    fn from_raw(ptr: *const u8, tag: u8) -> Self {
        let data = (ptr as usize | (tag as usize) << Self::BIT_SHIFT) as *const u8;
        Self {
            data,
            tag_type: PhantomData,
        }
    }

    fn tag(&self) -> u8 {
        ((self.data as usize) >> Self::BIT_SHIFT) as u8
    }

    fn data(&self) -> *const u8 {
        let mask = !(0xFF << Self::BIT_SHIFT);
        ((self.data as usize) & mask) as *const u8
    }
}

fn sum_enum<T: TaggedPointer<Basic>>(x: &[T]) -> i32 {
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

fn sum_raw<T: TaggedPointer<Basic>>(x: &[T]) -> i32 {
    let mut sum: i32 = 0;
    for i in x {
        match i.tag() {
            0 => sum = sum.wrapping_add(unsafe { (*i.data().cast::<X<0>>()).data }),
            1 => sum = sum.wrapping_add(unsafe { (*i.data().cast::<X<7>>()).data }),
            2 => sum = sum.wrapping_add(unsafe { (*i.data().cast::<X<4>>()).data }),
            3 => sum = sum.wrapping_add(unsafe { (*i.data().cast::<X<6>>()).data }),
            4 => sum = sum.wrapping_add(unsafe { (*i.data().cast::<X<2>>()).data }),
            5 => sum = sum.wrapping_add(unsafe { (*i.data().cast::<X<5>>()).data }),
            6 => sum = sum.wrapping_add(unsafe { (*i.data().cast::<X<3>>()).data }),
            7 => sum = sum.wrapping_add(unsafe { (*i.data().cast::<X<1>>()).data }),
            _ => unsafe { unreachable_unchecked() },
        }
    }
    sum
}

fn gen_basic_data<T: TaggedPointer<Basic>>(bump: &Bump) -> Vec<T> {
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
        x.push(T::new(basic));
    }
    x
}

fn compare_sums<T: TaggedPointer<Basic>>() {
    let bump = Bump::new();
    // generate a vector of random Basic Values
    let mut x = Vec::new();
    for _ in 0..1000 {
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
        x.push(T::new(basic));
    }
    assert_eq!(sum_enum(&x), sum_raw(&x));
}

fn bench_sum(c: &mut Criterion) {
    let mut group = c.benchmark_group("sum");
    let bump = Bump::new();
    let data = black_box(gen_basic_data(&bump));
    group.bench_function("sum_enum_low_bits", |b| {
        b.iter(|| sum_enum::<LowBits<_>>(&data))
    });

    let data = black_box(gen_basic_data(&bump));
    group.bench_function("sum_enum_low_byte", |b| {
        b.iter(|| sum_enum::<LowByte<_>>(&data))
    });

    let data = black_box(gen_basic_data(&bump));
    group.bench_function("sum_enum_high_bits", |b| {
        b.iter(|| sum_enum::<HighBits<_>>(&data))
    });

    let data = black_box(gen_basic_data(&bump));
    group.bench_function("sum_enum_high_byte", |b| {
        b.iter(|| sum_enum::<HighByte<_>>(&data))
    });
}

criterion_group!(benches, bench_sum,);
criterion_main!(benches);

// fn main() {
//     compare_sums();
//     println!("All tests passed");
// }
