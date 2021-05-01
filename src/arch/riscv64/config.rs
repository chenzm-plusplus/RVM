// use super::vcpu::*;

// use core::mem::size_of;

// #[macro_use]
// extern crate memoffset;
// /// Host and guest cpu register states.
// // #[repr(C)]
// // #[derive(Debug, Default)]
// // struct RvmStateRiscv64 {
// //     host_sscratch: u64,
// //     host_stvec: u64,
// //     host_scounteren: u64,
// //     host_context: CPUContext,
// //     guest_state: GuestState,
// //     resume: bool,//???????
// // }
// pub const RVMSTATE_GUEST_STATE: u64 = size_of::<u64> as u64 * 3 + size_of::<CPUContext>;
// pub const RVMSTATE_GUEST_CONTEXT: u64 = RVMSTATE_GUEST_STATE + 0;
// pub const RVMSTATE_GUEST_CSR: u64 = RVMSTATE_GUEST_STATE + size_of::<CPUCsr>();
// pub const RVMSTATE_HOST_CONTEXT: u64 = size_of::<u64> as u64 * 3;
// pub const RVMSTATE_HOST_RA: u64 = RVMSTATE_HOST_CONTEXT + size_of::<u64> as u64 * 0;
// pub const RVMSTATE_HOST_SP: u64 = RVMSTATE_HOST_CONTEXT + size_of::<u64> as u64 * 1;
// pub const RVMSTATE_HOST_GP: u64 = RVMSTATE_HOST_CONTEXT + size_of::<u64> as u64 * 2;
// pub const RVMSTATE_HOST_TP: u64 = RVMSTATE_HOST_CONTEXT + size_of::<u64> as u64 * 3;
// pub const RVMSTATE_HOST_S0: u64 = RVMSTATE_HOST_CONTEXT + size_of::<u64> as u64 * 4;
// pub const RVMSTATE_HOST_S1: u64 = RVMSTATE_HOST_CONTEXT + size_of::<u64> as u64 * 5;
// pub const RVMSTATE_HOST_A1: u64 = RVMSTATE_HOST_CONTEXT + size_of::<u64> as u64 * 6;
// pub const RVMSTATE_HOST_A2: u64 = RVMSTATE_HOST_CONTEXT + size_of::<u64> as u64 * 7;
// pub const RVMSTATE_HOST_A3: u64 = RVMSTATE_HOST_CONTEXT + size_of::<u64> as u64 * 8;
// pub const RVMSTATE_HOST_A4: u64 = RVMSTATE_HOST_CONTEXT + size_of::<u64> as u64 * 9;
// pub const RVMSTATE_HOST_A5: u64 = RVMSTATE_HOST_CONTEXT + size_of::<u64> as u64 * 10;
// pub const RVMSTATE_HOST_A6: u64 = RVMSTATE_HOST_CONTEXT + size_of::<u64> as u64 * 11;
// pub const RVMSTATE_HOST_A7: u64 = RVMSTATE_HOST_CONTEXT + size_of::<u64> as u64 * 12;
// pub const RVMSTATE_HOST_S2: u64 = RVMSTATE_HOST_CONTEXT + size_of::<u64> as u64 * 13;
// pub const RVMSTATE_HOST_S3: u64 = RVMSTATE_HOST_CONTEXT + size_of::<u64> as u64 * 14;
// pub const RVMSTATE_HOST_S4: u64 = RVMSTATE_HOST_CONTEXT + size_of::<u64> as u64 * 15;
// pub const RVMSTATE_HOST_S5: u64 = RVMSTATE_HOST_CONTEXT + size_of::<u64> as u64 * 16;
// pub const RVMSTATE_HOST_S6: u64 = RVMSTATE_HOST_CONTEXT + size_of::<u64> as u64 * 17;
// pub const RVMSTATE_HOST_S7: u64 = RVMSTATE_HOST_CONTEXT + size_of::<u64> as u64 * 18;
// pub const RVMSTATE_HOST_S8: u64 = RVMSTATE_HOST_CONTEXT + size_of::<u64> as u64 * 19;
// pub const RVMSTATE_HOST_S9: u64 = RVMSTATE_HOST_CONTEXT + size_of::<u64> as u64 * 20;
// pub const RVMSTATE_HOST_S10: u64 = RVMSTATE_HOST_CONTEXT + size_of::<u64> as u64 * 21;
// pub const RVMSTATE_HOST_S11: u64 = RVMSTATE_HOST_CONTEXT + size_of::<u64> as u64 * 22;
// pub const RVMSTATE_GUEST_SSTATUS: u64 = RVMSTATE_GUEST_CONTEXT + size_of::<u64> as u64 * 33;
// pub const RVMSTATE_HOST_SSTATUS: u64 = RVMSTATE_HOST_CONTEXT + size_of::<u64> as u64 * 33;
// pub const RVMSTATE_GUEST_HSTATUS: u64 = RVMSTATE_GUEST_CONTEXT + size_of::<u64> as u64 * 34;
// pub const RVMSTATE_HOST_HSTATUS: u64 = RVMSTATE_HOST_CONTEXT + size_of::<u64> as u64 * 34;
// pub const RVMSTATE_GUEST_SCOUNTEREN: u64 = RVMSTATE_GUEST_CSR + size_of::<u64> as u64* 9;
// pub const RVMSTATE_HOST_SCOUNTEREN: u64 = size_of::<u64> as u64 * 2;