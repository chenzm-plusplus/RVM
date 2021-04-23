use crate::memory::{HostPhysAddr,};

/// Extended page table
#[derive(Debug)]
pub struct EPageTable {
    ept_page_root: HostPhysAddr,
}

impl EPageTable{
    pub fn new() -> Self{
        ept_page_root: 0
    }
}