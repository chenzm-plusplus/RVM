use super::vcpu::{HostState,GuestState,RvmStateRiscv64};

//gueststate
pub const ra : &str = "ra";//1
pub const sp : &str = "sp";//2
pub const gp : &str = "gp";//3
pub const tp : &str = "tp";//4
pub const t0 : &str = "t0";//5
pub const t1 : &str = "t1";//6
pub const t2 : &str = "t2";//7
pub const s0 : &str = "s0";//8
pub const s1 : &str = "s1";//9
pub const a0 : &str = "a0";//10
pub const a1 : &str = "a1";//11
pub const a2 : &str = "a2";//12
pub const a3 : &str = "a3";//13
pub const a4 : &str = "a4";//14
pub const a5 : &str = "a5";//15
pub const a6 : &str = "a6";//16
pub const a7 : &str = "a7";//17
pub const s2 : &str = "s2";//18
pub const s3 : &str = "s3";//19
pub const s4 : &str = "s4";//20
pub const s5 : &str = "s5";//21
pub const s6 : &str = "s6";//22
pub const s7 : &str = "s7";//23
pub const s8 : &str = "s8";//24
pub const s9 : &str = "s9";//25
pub const s10 : &str = "s10";//26
pub const s11 : &str = "s11";//27
pub const t3 : &str = "t3";//28
pub const t4 : &str = "t4";//29
pub const t5 : &str = "t5";//30
pub const t6 : &str = "t6";//31

pub const REGBYTES : u64 = 8;

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
// pub const GUEST_STATE_OFFSET: u64 = offset_of!(RvmStateRiscv64, guest_state) as u64;
// pub const GUEST_OFFSET_zero : u64 = offset_of!(GuestState,zero) as u64;//0
// pub const GUEST_OFFSET_ra : u64 = offset_of!(GuestState,ra) as u64;//1
// pub const GUEST_OFFSET_sp : u64 = offset_of!(GuestState,sp) as u64;//2
// pub const GUEST_OFFSET_gp : u64 = offset_of!(GuestState,gp) as u64;//3
// pub const GUEST_OFFSET_tp : u64 = offset_of!(GuestState,tp) as u64;//4
// pub const GUEST_OFFSET_t0 : u64 = offset_of!(GuestState,t0) as u64;//5
// pub const GUEST_OFFSET_t1 : u64 = offset_of!(GuestState,t1) as u64;//6
// pub const GUEST_OFFSET_t2 : u64 = offset_of!(GuestState,t2) as u64;//7
// pub const GUEST_OFFSET_s0 : u64 = offset_of!(GuestState,s0) as u64;//8
// pub const GUEST_OFFSET_s1 : u64 = offset_of!(GuestState,s1) as u64;//9
// pub const GUEST_OFFSET_a0 : u64 = offset_of!(GuestState,a0) as u64;//10
// pub const GUEST_OFFSET_a1 : u64 = offset_of!(GuestState,a1) as u64;//11
// pub const GUEST_OFFSET_a2 : u64 = offset_of!(GuestState,a2) as u64;//12
// pub const GUEST_OFFSET_a3 : u64 = offset_of!(GuestState,a3) as u64;//13
// pub const GUEST_OFFSET_a4 : u64 = offset_of!(GuestState,a4) as u64;//14
// pub const GUEST_OFFSET_a5 : u64 = offset_of!(GuestState,a5) as u64;//15
// pub const GUEST_OFFSET_a6 : u64 = offset_of!(GuestState,a6) as u64;//16
// pub const GUEST_OFFSET_a7 : u64 = offset_of!(GuestState,a7) as u64;//17
// pub const GUEST_OFFSET_s2 : u64 = offset_of!(GuestState,s2) as u64;//18
// pub const GUEST_OFFSET_s3 : u64 = offset_of!(GuestState,s3) as u64;//19
// pub const GUEST_OFFSET_s4 : u64 = offset_of!(GuestState,s4) as u64;//20
// pub const GUEST_OFFSET_s5 : u64 = offset_of!(GuestState,s5) as u64;//21
// pub const GUEST_OFFSET_s6 : u64 = offset_of!(GuestState,s6) as u64;//22
// pub const GUEST_OFFSET_s7 : u64 = offset_of!(GuestState,s7) as u64;//23
// pub const GUEST_OFFSET_s8 : u64 = offset_of!(GuestState,s8) as u64;//24
// pub const GUEST_OFFSET_s9 : u64 = offset_of!(GuestState,s9) as u64;//25
// pub const GUEST_OFFSET_s10 : u64 = offset_of!(GuestState,s10) as u64;//26
// pub const GUEST_OFFSET_s11 : u64 = offset_of!(GuestState,s11) as u64;//27
// pub const GUEST_OFFSET_t3 : u64 = offset_of!(GuestState,t3) as u64;//28
// pub const GUEST_OFFSET_t4 : u64 = offset_of!(GuestState,t4) as u64;//29
// pub const GUEST_OFFSET_t5 : u64 = offset_of!(GuestState,t5) as u64;//30
// pub const GUEST_OFFSET_t6 : u64 = offset_of!(GuestState,t6) as u64;//31

// pub const GUEST_OFFSET_sepc : u64 = offset_of!(GuestState,sepc) as u64;//32
// pub const GUEST_OFFSET_sstatus :u64 = offset_of!(GuestState,sstatus) as u64;//33
// pub const GUEST_OFFSET_hstatus :u64 = offset_of!(GuestState,hstatus) as u64;//34
// pub const GUEST_OFFSET_scounteren :u64 = offset_of!(GuestState,scounteren) as u64;//35

//offset:host
// pub const HOST_STATE_OFFSET : u64 = offset_of!(RvmStateRiscv64,host_state) as u64;
// pub const HOST_OFFSET_zero : u64 = offset_of!(HostState,zero) as u64;//0
// pub const HOST_OFFSET_ra : u64 = offset_of!(HostState,ra) as u64;//1
// pub const HOST_OFFSET_sp : u64 = offset_of!(HostState,sp) as u64;//2
// pub const HOST_OFFSET_gp : u64 = offset_of!(HostState,gp) as u64;//3
// pub const HOST_OFFSET_tp : u64 = offset_of!(HostState,tp) as u64;//4
// pub const HOST_OFFSET_s0 : u64 = offset_of!(HostState,s0) as u64;//5
// pub const HOST_OFFSET_s1 : u64 = offset_of!(HostState,s1) as u64;//6
// pub const HOST_OFFSET_a0 : u64 = offset_of!(HostState,a0) as u64;//7
// pub const HOST_OFFSET_a1 : u64 = offset_of!(HostState,a1) as u64;//8
// pub const HOST_OFFSET_a2 : u64 = offset_of!(HostState,a2) as u64;//9
// pub const HOST_OFFSET_a3 : u64 = offset_of!(HostState,a3) as u64;//10
// pub const HOST_OFFSET_a4 : u64 = offset_of!(HostState,a4) as u64;//11
// pub const HOST_OFFSET_a5 : u64 = offset_of!(HostState,a5) as u64;//12
// pub const HOST_OFFSET_a6 : u64 = offset_of!(HostState,a6) as u64;//13
// pub const HOST_OFFSET_a7 : u64 = offset_of!(HostState,a7) as u64;//14
// pub const HOST_OFFSET_s2 : u64 = offset_of!(HostState,s2) as u64;//15
// pub const HOST_OFFSET_s3 : u64 = offset_of!(HostState,s3) as u64;//16
// pub const HOST_OFFSET_s4 : u64 = offset_of!(HostState,s4) as u64;//17
// pub const HOST_OFFSET_s5 : u64 = offset_of!(HostState,s5) as u64;//18
// pub const HOST_OFFSET_s6 : u64 = offset_of!(HostState,s6) as u64;//19
// pub const HOST_OFFSET_s7 : u64 = offset_of!(HostState,s7) as u64;//20
// pub const HOST_OFFSET_s8 : u64 = offset_of!(HostState,s8) as u64;//21
// pub const HOST_OFFSET_s9 : u64 = offset_of!(HostState,s9) as u64;//22
// pub const HOST_OFFSET_s10 : u64 = offset_of!(HostState,s10) as u64;//23
// pub const HOST_OFFSET_s11 : u64 = offset_of!(HostState,s11) as u64;//24

// pub const HOST_OFFSET_sepc : u64 = offset_of!(HostState,sepc) as u64;//25
// pub const HOST_OFFSET_sstatus : u64 = offset_of!(HostState,sstatus) as u64;//26
// pub const HOST_OFFSET_hstatus : u64 = offset_of!(HostState,hstatus) as u64;//27

// pub const HOST_OFFSET_sscratch: u64 = offset_of!(HostState,sscratch) as u64;//28
// pub const HOST_OFFSET_stvec: u64 = offset_of!(HostState,stvec) as u64;//29
// pub const HOST_OFFSET_scounteren: u64 = offset_of!(HostState,scounteren) as u64;//30