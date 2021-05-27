use super::vcpu::{HostState,GuestState,RvmStateRiscv64};

//gueststate
pub const RA : &str = "ra";//1
pub const SP : &str = "sp";//2
pub const GP : &str = "gp";//3
pub const TP : &str = "tp";//4
pub const T0 : &str = "t0";//5
pub const T1 : &str = "t1";//6
pub const T2 : &str = "t2";//7
pub const S0 : &str = "s0";//8
pub const S1 : &str = "s1";//9
pub const A0 : &str = "a0";//10
pub const A1 : &str = "a1";//11
pub const A2 : &str = "a2";//12
pub const A3 : &str = "a3";//13
pub const A4 : &str = "a4";//14
pub const A5 : &str = "a5";//15
pub const A6 : &str = "a6";//16
pub const A7 : &str = "a7";//17
pub const S2 : &str = "s2";//18
pub const S3 : &str = "s3";//19
pub const S4 : &str = "s4";//20
pub const S5 : &str = "s5";//21
pub const S6 : &str = "s6";//22
pub const S7 : &str = "s7";//23
pub const S8 : &str = "s8";//24
pub const S9 : &str = "s9";//25
pub const S10 : &str = "s10";//26
pub const S11 : &str = "s11";//27
pub const T3 : &str = "t3";//28
pub const T4 : &str = "t4";//29
pub const T5 : &str = "t5";//30
pub const T6 : &str = "t6";//31

pub const REGBYTES : u64 = 8;

//offset:host
pub const HOST_STATE_OFFSET : u64 = 0*REGBYTES;
pub const HOST_OFFSET_ZERO : u64 = 0*REGBYTES;
pub const HOST_OFFSET_RA : u64 = 1*REGBYTES;
pub const HOST_OFFSET_SP : u64 = 2*REGBYTES;
pub const HOST_OFFSET_GP : u64 = 3*REGBYTES;
pub const HOST_OFFSET_TP : u64 = 4*REGBYTES;
pub const HOST_OFFSET_S0 : u64 = 5*REGBYTES;
pub const HOST_OFFSET_S1 : u64 = 6*REGBYTES;
pub const HOST_OFFSET_A0 : u64 = 7*REGBYTES;
pub const HOST_OFFSET_A1 : u64 = 8*REGBYTES;
pub const HOST_OFFSET_A2 : u64 = 9*REGBYTES;
pub const HOST_OFFSET_A3 : u64 = 10*REGBYTES;
pub const HOST_OFFSET_A4 : u64 = 11*REGBYTES;
pub const HOST_OFFSET_A5 : u64 = 12*REGBYTES;
pub const HOST_OFFSET_A6 : u64 = 13*REGBYTES;
pub const HOST_OFFSET_A7 : u64 = 14*REGBYTES;
pub const HOST_OFFSET_S2 : u64 = 15*REGBYTES;
pub const HOST_OFFSET_S3 : u64 = 16*REGBYTES;
pub const HOST_OFFSET_S4 : u64 = 17*REGBYTES;
pub const HOST_OFFSET_S5 : u64 = 18*REGBYTES;
pub const HOST_OFFSET_S6 : u64 = 19*REGBYTES;
pub const HOST_OFFSET_S7 : u64 = 20*REGBYTES;
pub const HOST_OFFSET_S8 : u64 = 21*REGBYTES;
pub const HOST_OFFSET_S9 : u64 = 22*REGBYTES;
pub const HOST_OFFSET_S10 : u64 = 23*REGBYTES;
pub const HOST_OFFSET_S11 : u64 = 24*REGBYTES;

pub const HOST_OFFSET_SEPC : u64 = 25*REGBYTES;
pub const HOST_OFFSET_SSTATUS : u64 = 26*REGBYTES;
pub const HOST_OFFSET_HSTATUS : u64 = 27*REGBYTES;

pub const HOST_OFFSET_SSCRATCH: u64 = 28*REGBYTES;
pub const HOST_OFFSET_STVEC: u64 = 29*REGBYTES;
pub const HOST_OFFSET_SCOUNTEREN: u64 = 30*REGBYTES;

//offsets:guest
pub const GUEST_STATE_OFFSET: u64 = 31*REGBYTES;
pub const GUEST_OFFSET_ZERO : u64 = 0*REGBYTES;
pub const GUEST_OFFSET_RA : u64 = 1*REGBYTES;
pub const GUEST_OFFSET_SP : u64 = 2*REGBYTES;
pub const GUEST_OFFSET_GP : u64 = 3*REGBYTES;
pub const GUEST_OFFSET_TP : u64 = 4*REGBYTES;
pub const GUEST_OFFSET_T0 : u64 = 5*REGBYTES;
pub const GUEST_OFFSET_T1 : u64 = 6*REGBYTES;
pub const GUEST_OFFSET_T2 : u64 = 7*REGBYTES;
pub const GUEST_OFFSET_S0 : u64 = 8*REGBYTES;
pub const GUEST_OFFSET_S1 : u64 = 9*REGBYTES;
pub const GUEST_OFFSET_A0 : u64 = 10*REGBYTES;
pub const GUEST_OFFSET_A1 : u64 = 11*REGBYTES;
pub const GUEST_OFFSET_A2 : u64 = 12*REGBYTES;
pub const GUEST_OFFSET_A3 : u64 = 13*REGBYTES;
pub const GUEST_OFFSET_A4 : u64 = 14*REGBYTES;
pub const GUEST_OFFSET_A5 : u64 = 15*REGBYTES;
pub const GUEST_OFFSET_A6 : u64 = 16*REGBYTES;
pub const GUEST_OFFSET_A7 : u64 = 17*REGBYTES;
pub const GUEST_OFFSET_S2 : u64 = 18*REGBYTES;
pub const GUEST_OFFSET_S3 : u64 = 19*REGBYTES;
pub const GUEST_OFFSET_S4 : u64 = 20*REGBYTES;
pub const GUEST_OFFSET_S5 : u64 = 21*REGBYTES;
pub const GUEST_OFFSET_S6 : u64 = 22*REGBYTES;
pub const GUEST_OFFSET_S7 : u64 = 23*REGBYTES;
pub const GUEST_OFFSET_S8 : u64 = 24*REGBYTES;
pub const GUEST_OFFSET_S9 : u64 = 25*REGBYTES;
pub const GUEST_OFFSET_S10 : u64 = 26*REGBYTES;
pub const GUEST_OFFSET_S11 : u64 = 27*REGBYTES;
pub const GUEST_OFFSET_T3 : u64 = 28*REGBYTES;
pub const GUEST_OFFSET_T4 : u64 = 29*REGBYTES;
pub const GUEST_OFFSET_T5 : u64 = 30*REGBYTES;
pub const GUEST_OFFSET_T6 : u64 = 31*REGBYTES;

pub const GUEST_OFFSET_SEPC : u64 = 32*REGBYTES;
pub const GUEST_OFFSET_SSTATUS :u64 = 33*REGBYTES;
pub const GUEST_OFFSET_HSTATUS :u64 = 34*REGBYTES;
pub const GUEST_OFFSET_SCOUNTEREN :u64 = 35*REGBYTES;