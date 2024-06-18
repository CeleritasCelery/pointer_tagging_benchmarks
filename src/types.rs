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

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Basic {
    // T0(*const X<0>) = 0,
    // T1(*const X<7>) = 1,
    // T2(*const X<4>) = 2,
    // T3(*const X<6>) = 3,
    // T4(*const X<2>) = 4,
    // T5(*const X<5>) = 5,
    // T6(*const X<3>) = 6,
    // T7(*const X<1>) = 7,
    T0(*const X<0>) = 0,
    T1(*const X<0>) = 1,
    T2(*const X<0>) = 2,
    T3(*const X<0>) = 3,
    T4(*const X<0>) = 4,
    T5(*const X<0>) = 5,
    T6(*const X<0>) = 6,
    T7(*const X<0>) = 7,
}

#[repr(u8)]
pub enum BasicTag {
    T0 = 0,
    T1 = 1,
    T2 = 2,
    T3 = 3,
    T4 = 4,
    T5 = 5,
    T6 = 6,
    T7 = 7,
}

impl Taggable for Basic {
    #[inline(always)]
    fn tag(&self) -> u8 {
        match self {
            Basic::T0(_) => 0,
            Basic::T1(_) => 1,
            Basic::T2(_) => 2,
            Basic::T3(_) => 3,
            Basic::T4(_) => 4,
            Basic::T5(_) => 5,
            Basic::T6(_) => 6,
            Basic::T7(_) => 7,
        }
    }

    #[inline(always)]
    fn ptr(&self) -> *const u8 {
        match self {
            Basic::T0(ptr) => *ptr as *const u8,
            Basic::T1(ptr) => *ptr as *const u8,
            Basic::T2(ptr) => *ptr as *const u8,
            Basic::T3(ptr) => *ptr as *const u8,
            Basic::T4(ptr) => *ptr as *const u8,
            Basic::T5(ptr) => *ptr as *const u8,
            Basic::T6(ptr) => *ptr as *const u8,
            Basic::T7(ptr) => *ptr as *const u8,
        }
    }

    #[inline(always)]
    fn from_raw(ptr: *const u8, tag: u8) -> Self {
        match tag {
            // 0 => Basic::T0(ptr as *const X<0>),
            // 1 => Basic::T1(ptr as *const X<7>),
            // 2 => Basic::T2(ptr as *const X<4>),
            // 3 => Basic::T3(ptr as *const X<6>),
            // 4 => Basic::T4(ptr as *const X<2>),
            // 5 => Basic::T5(ptr as *const X<5>),
            // 6 => Basic::T6(ptr as *const X<3>),
            // 7 => Basic::T7(ptr as *const X<1>),
            0 => Basic::T0(ptr as *const X<0>),
            1 => Basic::T1(ptr as *const X<0>),
            2 => Basic::T2(ptr as *const X<0>),
            3 => Basic::T3(ptr as *const X<0>),
            4 => Basic::T4(ptr as *const X<0>),
            5 => Basic::T5(ptr as *const X<0>),
            6 => Basic::T6(ptr as *const X<0>),
            7 => Basic::T7(ptr as *const X<0>),
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
        // match self.data as usize & 0b111 {
        //     0 => (self.data as usize) as *const u8,
        //     1 => (self.data as usize - 1) as *const u8,
        //     2 => (self.data as usize - 2) as *const u8,
        //     3 => (self.data as usize - 3) as *const u8,
        //     4 => (self.data as usize - 4) as *const u8,
        //     5 => (self.data as usize - 5) as *const u8,
        //     6 => (self.data as usize - 6) as *const u8,
        //     7 => (self.data as usize - 7) as *const u8,
        //     _ => unsafe { unreachable_unchecked() },
        // }
        let lower = self.tag() as usize;
        (self.data as usize - lower) as *const u8
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

    // #[inline(always)]
    // fn untag(&self) -> Basic {
    //     let tag = self.tag();
    //     let data = self.data();

    //     unsafe { std::mem::transmute::<(u8, *const u8), Basic>((tag, data)) }
    // }
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

#[derive(Copy, Clone)]
pub struct BaseLine {
    data: Basic,
}

impl TaggedPointer<Basic> for BaseLine {
    #[inline(always)]
    fn from_raw(ptr: *const u8, tag: u8) -> Self {
        let data = Basic::from_raw(ptr, tag);
        Self { data }
    }

    #[inline(always)]
    fn tag(&self) -> u8 {
        self.data.tag()
    }

    #[inline(always)]
    fn data(&self) -> *const u8 {
        self.data.ptr()
    }

    fn untag(&self) -> Basic {
        self.data
    }
}
