/// Xorshift RNGs algorithm from George Marsaglia - The Florida State University
/// see https://www.jstatsoft.org/article/view/v008i14/916
/// NOT for use for cryptographic values
use serde::{Deserialize, Serialize};

// === rand8 ================================================================

/// A randomized 8-bit value generator using the xorshift algorithm
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, Eq, Hash, PartialEq)]
#[cfg(any(feature = "rand8"))]
pub struct XorShift8 {
    state: u8,
}

#[cfg(any(feature = "rand8"))]
impl XorShift8 {
    pub fn new(seed: u8) -> Self {
        if seed == 0 {
            panic!("The seed cannot be 0 for an xorshift generator.");
        }
        XorShift8 { state: seed }
    }

    /// Next f32 random number using next rand8
    pub fn next_f32(&mut self) -> f32 {
        let max = u8::MAX as f32;
        self.next().unwrap() as f32 / max
    }
}

/// Next rand8 number using an Iterator
#[cfg(any(feature = "rand8"))]
impl Iterator for XorShift8 {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.state ^= self.state << 7;
        self.state ^= self.state >> 3;
        self.state ^= self.state << 5;
        Some(self.state)
    }
}

// === rand16 ===============================================================

/// A randomized 16-bit value generator using the xorshift algorithm
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, Eq, Hash, PartialEq)]
#[cfg(any(feature = "rand16"))]
pub struct XorShift16 {
    state: u16,
}

#[cfg(any(feature = "rand16"))]
impl XorShift16 {
    pub fn new(seed: u16) -> Self {
        if seed == 0 {
            panic!("The seed cannot be 0 for an xorshift generator.");
        }
        XorShift16 { state: seed }
    }

    /// Next f32 random number using next rand16
    pub fn next_f32(&mut self) -> f32 {
        let max = u16::MAX as f32;
        self.next().unwrap() as f32 / max
    }
}

/// Next rand16 number using an Iterator
#[cfg(any(feature = "rand16"))]
impl Iterator for XorShift16 {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        self.state ^= self.state << 7;
        self.state ^= self.state >> 9;
        self.state ^= self.state << 13;
        Some(self.state)
    }
}

// === rand32 ===============================================================

/// A randomized 32-bit value generator using the xorshift algorithm
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct XorShift32 {
    state: u32,
}

impl XorShift32 {
    pub fn new(seed: u32) -> Self {
        if seed == 0 {
            panic!("The seed cannot be 0 for an xorshift generator.");
        }
        XorShift32 { state: seed }
    }

    /// Next f32 random number using next rand32
    pub fn next_f32(&mut self) -> f32 {
        let max = u32::MAX as f32;
        self.next().unwrap() as f32 / max
    }
}

/// Next rand32 number using an Iterator
impl Iterator for XorShift32 {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 17;
        self.state ^= self.state << 5;
        Some(self.state)
    }
}

// === rand64 ===============================================================

/// A randomized 64-bit value generator using the xorshift algorithm
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, Eq, Hash, PartialEq)]
#[cfg(any(feature = "rand64"))]
pub struct XorShift64 {
    state: u64,
}

#[cfg(any(feature = "rand64"))]
impl XorShift64 {
    pub fn new(seed: u64) -> Self {
        if seed == 0 {
            panic!("The seed cannot be 0 for an xorshift generator.");
        }
        XorShift64 { state: seed }
    }

    /// Next f64 random number using next rand64
    pub fn next_f64(&mut self) -> f64 {
        let max = u64::MAX as f64;
        self.next().unwrap() as f64 / max
    }
}

/// Next rand64 number using an Iterator
#[cfg(any(feature = "rand64"))]
impl Iterator for XorShift64 {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        Some(self.state)
    }
}
