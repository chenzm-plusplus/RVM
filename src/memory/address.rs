use core::fmt::{self, Debug, Formatter};
use core::ops::{
    Add,Sub,
};

use crate::config::{
    PAGE_SIZE,
    // PAGE_SIZE_BITS,
};

/// Definitions
#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct GuestPhysAddr(pub usize);

/// Definitions
#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct HostPhysAddr(pub usize);

#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct HostVirtAddr(pub usize);


/// Debugging

impl Debug for HostVirtAddr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("HVA:{:#x}", self.0))
    }
}

impl Debug for GuestPhysAddr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("GPA:{:#x}", self.0))
    }
}
impl Debug for HostPhysAddr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("HPA:{:#x}", self.0))
    }
}

// calculating...
impl Sub for HostVirtAddr{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            0: self.0 - other.0,
        }
    }
}

impl Sub for HostPhysAddr{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            0: self.0 - other.0,
        }
    }
}

impl Sub for GuestPhysAddr{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            0: self.0 - other.0,
        }
    }
}

impl Add for HostVirtAddr{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            0: self.0 + other.0,
        }
    }
}

impl Add for HostPhysAddr{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            0: self.0 + other.0,
        }
    }
}

impl Add for GuestPhysAddr{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            0: self.0 + other.0,
        }
    }
}

// 类型转换

impl From<usize> for GuestPhysAddr {
    fn from(v: usize) -> Self { Self(v) }
}
impl From<usize> for HostPhysAddr {
    fn from(v: usize) -> Self { Self(v) }
}
impl From<usize> for HostVirtAddr {
    fn from(v: usize) -> Self { Self(v) }
}

impl From<GuestPhysAddr> for usize {
    fn from(v: GuestPhysAddr) -> Self { v.0 }
}
impl From<HostPhysAddr> for usize {
    fn from(v: HostPhysAddr) -> Self { v.0 }
}
impl From<HostVirtAddr> for usize {
    fn from(v: HostVirtAddr) -> Self { v.0 }
}


// 位移等
impl HostVirtAddr {
    // pub fn floor(&self) -> VirtPageNum { VirtPageNum(self.0 / PAGE_SIZE) }
    // pub fn ceil(&self) -> VirtPageNum  { VirtPageNum((self.0 - 1 + PAGE_SIZE) / PAGE_SIZE) }
    pub fn page_offset(&self) -> usize { self.0 & (PAGE_SIZE - 1) }
    pub fn aligned(&self) -> bool { self.page_offset() == 0 }
}

impl GuestPhysAddr {
    // pub fn floor(&self) -> PhysPageNum { PhysPageNum(self.0 / PAGE_SIZE) }
    // pub fn ceil(&self) -> PhysPageNum { PhysPageNum((self.0 - 1 + PAGE_SIZE) / PAGE_SIZE) }
    pub fn page_offset(&self) -> usize { self.0 & (PAGE_SIZE - 1) }
    pub fn aligned(&self) -> bool { self.page_offset() == 0 }
}
impl HostPhysAddr {
    // pub fn floor(&self) -> PhysPageNum { PhysPageNum(self.0 / PAGE_SIZE) }
    // pub fn ceil(&self) -> PhysPageNum { PhysPageNum((self.0 - 1 + PAGE_SIZE) / PAGE_SIZE) }
    pub fn page_offset(&self) -> usize { self.0 & (PAGE_SIZE - 1) }
    pub fn aligned(&self) -> bool { self.page_offset() == 0 }
}

//get mut
impl GuestPhysAddr {
    pub fn get_mut<T>(&self) -> &'static mut T {
        unsafe {
            (self.0 as *mut T).as_mut().unwrap()
        }
    }
}
impl HostPhysAddr {
    pub fn get_mut<T>(&self) -> &'static mut T {
        unsafe {
            (self.0 as *mut T).as_mut().unwrap()
        }
    }
}