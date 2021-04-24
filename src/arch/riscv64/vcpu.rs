
use super::{
    Guest,
};

use alloc::{
    sync::Arc,
};

use crate::{
    RvmResult,
};

/// Represents a virtual CPU within a guest.
pub struct Vcpu {
	guest: Arc<Guest>,//自己对应的Guest要保存起来方便找
}

/// Holds the register state used to restore a guest.
#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct GuestState {
    pub zero: u64,
    pub ra: u64,
    pub sp: u64,
    pub gp: u64,
    pub tp: u64,
    pub t0: u64,
    pub t1: u64,
    pub t2: u64,
    pub s0: u64,
    pub s1: u64,
    pub a0: u64,
    pub a1: u64,
    pub a2: u64,
    pub a3: u64,
    pub a4: u64,
    pub a5: u64,
    pub a6: u64,
    pub a7: u64,
    //x18~27
    pub s2: u64,
    pub s3: u64,
    pub s4: u64,
    pub s5: u64,
    pub s6: u64,
    pub s7: u64,
    pub s8: u64,
    pub s9: u64,
    pub s10: u64,
    pub s11: u64,
    //x28~31
    pub t3: u64,
    pub t4: u64,
    pub t5: u64,
    pub t6: u64,
}

impl Vcpu{
	pub fn new(entry: u64, guest: Arc<Guest>) -> RvmResult<Self> {

        // if entry > guest.gpm.size() {
        //     return Err(RvmError::InvalidParam);
        // }

        let mut vcpu = Self {
            guest: guest.clone(),
        };

        vcpu.init(entry)?;

        Ok(vcpu)
    }

	fn init(&mut self, entry: u64) -> RvmResult {
		
        Ok(())
    }
}