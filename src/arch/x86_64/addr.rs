use core::ops::{Add, AddAssign, Sub, SubAssign};
use core::fmt;

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PAddr(u64);

impl PAddr {
    /// Convert to `u64`
    pub const fn as_u64(&self) -> u64 {
        self.0
    }
    /// Convert from `u64`
    pub const fn from_u64(v: u64) -> Self {
        PAddr(v)
    }
}

impl Add for PAddr {
    type Output = PAddr;
    
    fn add(self, _rhs: PAddr) -> PAddr {
        PAddr(self.0 + _rhs.0)
    }
}

impl AddAssign for PAddr {
    fn add_assign(&mut self, _rhs: PAddr) {
        self.0 += _rhs.0;
    }
}

impl Sub for PAddr {
    type Output = PAddr;
    
    fn sub(self, _rhs: PAddr) -> PAddr {
        PAddr(self.0 - _rhs.0)
    }
}

impl SubAssign for PAddr {
    fn sub_assign(&mut self, _rhs: PAddr) {
        self.0 -= _rhs.0
    }
}

impl fmt::Binary for PAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for PAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::LowerHex for PAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Octal for PAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::UpperHex for PAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Represent a virtual (linear) memory address
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct VAddr(u64);

impl Add for VAddr {
    type Output = VAddr;
    
    fn add(self, _rhs: VAddr) -> VAddr {
        VAddr(self.0 + _rhs.0)
    }
}

impl AddAssign for VAddr {
    fn add_assign(&mut self, _rhs: VAddr) {
        self.0 += _rhs.0;
    }
}

impl Sub for VAddr {
    type Output = VAddr;

    fn sub(self, _rhs: VAddr) -> VAddr {
        VAddr(self.0 - _rhs.0)
    }
}

impl SubAssign for VAddr {
    fn sub_assign(&mut self, _rhs: VAddr) {
        self.0 -= _rhs.0
    }
}

impl VAddr {
    /// Convert to `usize`
    pub const fn as_u64(&self) -> u64 {
        self.0
    }
    /// Convert from `usize`
    pub const fn from_u64(v: u64) -> Self {
        VAddr(v)
    }
}

impl fmt::Binary for VAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for VAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::LowerHex for VAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Octal for VAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::UpperHex for VAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
