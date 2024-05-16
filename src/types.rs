use std::{hint::unreachable_unchecked, marker::PhantomData};

pub trait Taggable {
    fn tag(&self) -> u8;
    fn ptr(&self) -> *const u8;
    fn from_raw(ptr: *const u8, tag: u8) -> Self;
}

pub trait TaggedPointer<T: Taggable>
where
    Self: Sized,
{
    #[inline(always)]
    fn new(val: T) -> Self {
        Self::from_raw(val.ptr(), val.tag())
    }
    #[inline(always)]
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
pub struct X<const N: usize> {
    _pad: [u32; N],
    pub data: i32,
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
    pub fn new(data: i32) -> Self {
        Self { _pad: [0; N], data }
    }
}

#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Basic {
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
    #[inline(always)]
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

    #[inline(always)]
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

    #[inline(always)]
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

#[derive(Copy, Clone)]
pub struct LowBits<T> {
    data: *const u8,
    tag_type: PhantomData<T>,
}

impl<T: Taggable> TaggedPointer<T> for LowBits<T> {
    #[inline(always)]
    fn from_raw(ptr: *const u8, tag: u8) -> Self {
        let data = (ptr as usize | tag as usize) as *const u8;
        Self {
            data,
            tag_type: PhantomData,
        }
    }

    #[inline(always)]
    fn tag(&self) -> u8 {
        self.data as u8 & 0b111
    }

    #[inline(always)]
    fn data(&self) -> *const u8 {
        match self.data as usize & 0b111 {
            0 => (self.data as usize) as *const u8,
            1 => (self.data as usize - 1) as *const u8,
            2 => (self.data as usize - 2) as *const u8,
            3 => (self.data as usize - 3) as *const u8,
            4 => (self.data as usize - 4) as *const u8,
            5 => (self.data as usize - 5) as *const u8,
            6 => (self.data as usize - 6) as *const u8,
            7 => (self.data as usize - 7) as *const u8,
            _ => unsafe { unreachable_unchecked() },
        }
        // let mask = !0b111;
        // (self.data as usize & mask) as *const u8
    }
}

#[derive(Copy, Clone)]
pub struct LowByte<T> {
    data: *const u8,
    tag_type: PhantomData<T>,
}

impl<T: Taggable> TaggedPointer<T> for LowByte<T> {
    #[inline(always)]
    fn from_raw(ptr: *const u8, tag: u8) -> Self {
        let data = (((ptr as usize) << 8) | tag as usize) as *const u8;
        Self {
            data,
            tag_type: PhantomData,
        }
    }

    #[inline(always)]
    fn tag(&self) -> u8 {
        self.data as u8
    }

    #[inline(always)]
    fn data(&self) -> *const u8 {
        (self.data as usize >> 8) as *const u8
    }
}

#[derive(Copy, Clone)]
pub struct HighBits<T> {
    data: *const u8,
    tag_type: PhantomData<T>,
}

impl<T> HighBits<T> {
    const BIT_SHIFT: usize = std::mem::size_of::<*const u8>() * 8 - 3;
}

impl<T: Taggable> TaggedPointer<T> for HighBits<T> {
    #[inline(always)]
    fn from_raw(ptr: *const u8, tag: u8) -> Self {
        let ptr = (ptr as usize) >> 3;
        let data = (ptr | (tag as usize) << Self::BIT_SHIFT) as *const u8;
        Self {
            data,
            tag_type: PhantomData,
        }
    }

    #[inline(always)]
    fn tag(&self) -> u8 {
        ((self.data as usize) >> Self::BIT_SHIFT) as u8
    }

    #[inline(always)]
    fn data(&self) -> *const u8 {
        ((self.data as usize) << 3) as *const u8
    }
}

#[derive(Copy, Clone)]
pub struct HighByte<T> {
    data: *const u8,
    tag_type: PhantomData<T>,
}

impl<T> HighByte<T> {
    const BIT_SHIFT: usize = std::mem::size_of::<*const u8>() * 8 - 8;
}

impl<T: Taggable> TaggedPointer<T> for HighByte<T> {
    #[inline(always)]
    fn from_raw(ptr: *const u8, tag: u8) -> Self {
        let data = (ptr as usize | (tag as usize) << Self::BIT_SHIFT) as *const u8;
        Self {
            data,
            tag_type: PhantomData,
        }
    }

    #[inline(always)]
    fn tag(&self) -> u8 {
        ((self.data as usize) >> Self::BIT_SHIFT) as u8
    }

    #[inline(always)]
    fn data(&self) -> *const u8 {
        let mask = !(0xFF << Self::BIT_SHIFT);
        ((self.data as usize) & mask) as *const u8
    }
}
