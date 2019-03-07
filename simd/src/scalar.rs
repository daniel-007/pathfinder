// pathfinder/simd/src/scalar.rs
//
// Copyright © 2019 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::f32;
use std::fmt::{self, Debug, Formatter};
use std::mem;
use std::ops::{Add, Index, IndexMut, Mul, Sub};

// 32-bit floats

#[derive(Clone, Copy, Default, PartialEq)]
pub struct F32x4(pub [f32; 4]);

impl F32x4 {
    #[inline]
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> F32x4 {
        F32x4([a, b, c, d])
    }

    #[inline]
    pub fn splat(x: f32) -> F32x4 {
        F32x4([x; 4])
    }

    #[inline]
    pub fn min(self, other: F32x4) -> F32x4 {
        F32x4([
            self[0].min(other[0]),
            self[1].min(other[1]),
            self[2].min(other[2]),
            self[3].min(other[3]),
        ])
    }

    #[inline]
    pub fn max(self, other: F32x4) -> F32x4 {
        F32x4([
            self[0].max(other[0]),
            self[1].max(other[1]),
            self[2].max(other[2]),
            self[3].max(other[3]),
        ])
    }

    #[inline]
    pub fn packed_eq(self, other: F32x4) -> U32x4 {
        U32x4([
            if self[0] == other[0] { !0 } else { 0 },
            if self[1] == other[1] { !0 } else { 0 },
            if self[2] == other[2] { !0 } else { 0 },
            if self[3] == other[3] { !0 } else { 0 },
        ])
    }

    // Casts these packed floats to 64-bit floats.
    //
    // NB: This is a pure bitcast and does no actual conversion; only use this if you know what
    // you're doing.
    #[inline]
    pub fn as_f64x2(self) -> F64x2 {
        unsafe {
            F64x2(*mem::transmute::<&[f32; 4], &[f64; 2]>(&self.0))
        }
    }

    // Converts these packed floats to integers.
    #[inline]
    pub fn to_i32x4(self) -> I32x4 {
        unsafe {
            I32x4(*mem::transmute::<&[f32; 4], &[i32; 4]>(&self.0))
        }
    }

    // Shuffles

    #[inline]
    pub fn xxyy(self) -> F32x4 {
        F32x4([self[0], self[0], self[1], self[1]])
    }

    #[inline]
    pub fn xyxy(self) -> F32x4 {
        F32x4([self[0], self[1], self[0], self[1]])
    }

    #[inline]
    pub fn xyyx(self) -> F32x4 {
        F32x4([self[0], self[1], self[1], self[0]])
    }

    #[inline]
    pub fn xzxz(self) -> F32x4 {
        F32x4([self[0], self[2], self[0], self[2]])
    }

    #[inline]
    pub fn ywyw(self) -> F32x4 {
        F32x4([self[1], self[3], self[1], self[3]])
    }

    #[inline]
    pub fn zzww(self) -> F32x4 {
        F32x4([self[2], self[2], self[3], self[3]])
    }

    #[inline]
    pub fn zwxy(self) -> F32x4 {
        F32x4([self[2], self[3], self[0], self[1]])
    }

    #[inline]
    pub fn zwzw(self) -> F32x4 {
        F32x4([self[2], self[3], self[2], self[3]])
    }

    #[inline]
    pub fn wxyz(self) -> F32x4 {
        F32x4([self[3], self[0], self[1], self[2]])
    }

    #[inline]
    pub fn interleave(self, other: F32x4) -> (F32x4, F32x4) {
        (F32x4([self[0], other[0], self[1], other[1]]),
            F32x4([self[2], other[2], self[3], other[3]]))
    }

    #[inline]
    pub fn cross(&self, other: F32x4) -> F32x4 {
        unimplemented!()
    }
}

impl Index<usize> for F32x4 {
    type Output = f32;
    #[inline]
    fn index(&self, index: usize) -> &f32 {
        &self.0[index]
    }
}

impl IndexMut<usize> for F32x4 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        &mut self.0[index]
    }
}

impl Debug for F32x4 {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "<{}, {}, {}, {}>", self[0], self[1], self[2], self[3])
    }
}

impl Add<F32x4> for F32x4 {
    type Output = F32x4;
    #[inline]
    fn add(self, other: F32x4) -> F32x4 {
        F32x4([self[0] + other[0], self[1] + other[1], self[2] + other[2], self[3] + other[3]])
    }
}

impl Mul<F32x4> for F32x4 {
    type Output = F32x4;
    #[inline]
    fn mul(self, other: F32x4) -> F32x4 {
        F32x4([self[0] * other[0], self[1] * other[1], self[2] * other[2], self[3] * other[3]])
    }
}

impl Sub<F32x4> for F32x4 {
    type Output = F32x4;
    #[inline]
    fn sub(self, other: F32x4) -> F32x4 {
        F32x4([self[0] - other[0], self[1] - other[1], self[2] - other[2], self[3] - other[3]])
    }
}

// 64-bit floats

#[derive(Clone, Copy)]
pub struct F64x2(pub [f64; 2]);

impl F64x2 {
    // Shuffles

    #[inline]
    pub fn interleave(self, other: F64x2) -> (F64x2, F64x2) {
        (F64x2([self.0[0], other.0[0]]), F64x2([self.0[1], other.0[1]]))
    }

    // Creates `<self[0], other[1]>`.
    #[inline]
    pub fn combine_low_high(self, other: F64x2) -> F64x2 {
        F64x2([self.0[0], other.0[1]])
    }

    // Casts these packed floats to 32-bit floats.
    //
    // NB: This is a pure bitcast and does no actual conversion; only use this if you know what
    // you're doing.
    #[inline]
    pub fn as_f32x4(self) -> F32x4 {
        unsafe {
            let vector: &[f32; 4] = mem::transmute::<&[f64; 2], &[f32; 4]>(&self.0);
            F32x4([vector[0], vector[1], vector[2], vector[3]])
        }
    }
}

// 32-bit signed integers

#[derive(Clone, Copy)]
pub struct I32x4([i32; 4]);

impl I32x4 {
    #[inline]
    pub fn new(a: i32, b: i32, c: i32, d: i32) -> I32x4 {
        I32x4([a, b, c, d])
    }

    #[inline]
    pub fn splat(x: i32) -> I32x4 {
        I32x4([x; 4])
    }

    #[inline]
    pub fn as_u8x16(self) -> U8x16 {
        unsafe {
            U8x16(*mem::transmute::<&[i32; 4], &[u8; 16]>(&self.0))
        }
    }

    #[inline]
    pub fn min(self, other: I32x4) -> I32x4 {
        I32x4([
            self[0].min(other[0]),
            self[1].min(other[1]),
            self[2].min(other[2]),
            self[3].min(other[3]),
        ])
    }
}

impl Index<usize> for I32x4 {
    type Output = i32;
    #[inline]
    fn index(&self, index: usize) -> &i32 {
        &self.0[index]
    }
}

impl Sub<I32x4> for I32x4 {
    type Output = I32x4;
    #[inline]
    fn sub(self, other: I32x4) -> I32x4 {
        I32x4([self[0] - other[0], self[1] - other[1], self[2] - other[2], self[3] - other[3]])
    }
}

// 32-bit unsigned integers

#[derive(Clone, Copy)]
pub struct U32x4(pub [u32; 4]);

impl U32x4 {
    #[inline]
    fn is_all_ones(&self) -> bool {
        self[0] == !0 && self[1] == !0 && self[2] == !0 && self[3] == !0
    }
}

impl Index<usize> for U32x4 {
    type Output = u32;
    #[inline]
    fn index(&self, index: usize) -> &u32 {
        &self.0[index]
    }
}

// 8-bit unsigned integers

#[derive(Clone, Copy)]
pub struct U8x16([u8; 16]);

impl U8x16 {
    #[inline]
    pub fn as_i32x4(self) -> I32x4 {
        unsafe {
            I32x4(*mem::transmute::<&[u8; 16], &[i32; 4]>(&self.0))
        }
    }

    #[inline]
    pub fn shuffle(self, table: U8x16) -> U8x16 {
        let mut result = [0; 16];
        for index in 0..16 {
            result[index] = table.0[index]
        }
        U8x16(result)
    }
}