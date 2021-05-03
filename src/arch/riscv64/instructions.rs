
use super::bits::STATUS_FS;

use super::regs::*;

/*
llvm_asm!(assembly template
   : output operands
   : input operands
   : clobbers
   : options
   );
*/
/*
.macro STORE reg, offset, regbytes, base
    sd  \reg, \offset*REGBYTES(a0)
.macro LOAD reg, offset, regbytes, base
    ld  \reg, \offset*REGBYTES(a0)
*/
/// atomic read

#[macro_export]
macro_rules! ld {
    ( $regname:expr, $offset:expr, $base:expr) => {
        format_args!("ld\t{}, {}({})",$regname,$offset,$base)
    };
}

#[macro_export]
macro_rules! sd {
    ( $regname:expr, $offset:expr, $base:expr) => {
        format_args!("sd\t{}, {}({})",$regname,$offset,$base)
    };
}

// #[macro_export]
// macro_rules! csrrw {
//     ( $regname:expr, $offset:expr, $base:expr) => {
//         format_args!("csrrw\t{}, {}, {}",$regname,$offset,$base)
//     };
// }

// #[macro_export]
// macro_rules! csrw {
//     ( $offset:expr, $regname:expr) => {
//         format_args!("csrrw\t{}, {}",$offset,$regname)
//     };
// }

#[macro_export]
macro_rules! la {
    ( $offset:expr, $regname:literal) => {
        format_args!("la\t{}, {}",$offset,$regname)
    };
}

#[macro_export]
macro_rules! load {
    ( $regname:expr, $offset:expr, $base:expr) => {{
        let value: u64;
        #[allow(unused_unsafe)]
        unsafe { llvm_asm!(
            "ld $0, $1($2)"
            :"=r"(value)
            :"i"($regname), "i"($offset), "i"($base)
            :
        ) };
        value
    }};
}

// /// atomic read from CSR
// #[macro_export]
// macro_rules! store {
//     ( $r:ident ) => {{
//         let value: u64;
//         #[allow(unused_unsafe)]
//         unsafe { llvm_asm!("sd $0, $1" : "=r"(value) : "i"(super::csr::$r)) };
//         value
//     }};
// }

/// atomic read from CSR
// #[macro_export]
// macro_rules! load {
//     ( $loadto:ident, $offset:expr, ) => {{
//         let value: u64;
//         #[allow(unused_unsafe)]
//         unsafe { llvm_asm!("ld $0, $1($2)" 
//         : "=r"(value) 
//         : "r"(super::csr::$loadto), "i"()) };
//         value
//     }};
// }

/// atomic read from CSR
#[macro_export]
macro_rules! csrr {
    ( $r:ident ) => {{
        let value: u64;
        #[allow(unused_unsafe)]
        unsafe { llvm_asm!("csrr $0, $1" : "=r"(value) : "i"(super::csr::$r)) };
        value
    }};
}

// / atomic write to CSR
#[macro_export]
macro_rules! csrw {
    ( $r:ident, $x:expr ) => {{
        let x: u64 = $x;
        llvm_asm!("csrw $0, $1" :: "i"(super::csr::$r), "r"(x) :: "volatile");
    }};
}

/// atomic write to CSR from immediate
#[macro_export]
macro_rules! csrwi {
    ( $r:ident, $x:expr ) => {{
        const X: u64 = $x;
        llvm_asm!("csrwi $0, $1" :: "i"(super::csr::$r), "i"(X) :: "volatile");
    }};
}

/// atomic read and set bits in CSR
#[macro_export]
macro_rules! csrs {
    ( $r:ident, $x:expr ) => {{
        let x: u64 = $x;
        llvm_asm!("csrs $0, $1" :: "i"(super::csr::$r), "r"(x) :: "volatile");
    }};
}

/// atomic read and set bits in CSR using immediate
#[macro_export]
macro_rules! csrsi {
    ( $r:ident, $x:expr ) => {{
        const X: u64 = $x;
        llvm_asm!("csrsi $0, $1" :: "i"(super::csr::$r), "i"(X) :: "volatile");
    }};
}

/// atomic read and clear bits in CSR
#[macro_export]
macro_rules! csrc {
    ( $r:ident, $x:expr ) => {{
        let x: u64 = $x;
        llvm_asm!("csrc $0, $1" :: "i"(super::csr::$r), "r"(x) :: "volatile");
    }};
}

/// atomic read and clear bits in CSR using immediate
#[macro_export]
macro_rules! csrci {
    ( $r:ident, $x:expr ) => {{
        const X: u64 = $x;
        llvm_asm!("csrci $0, $1" :: "i"(super::csr::$r), "i"(X) :: "volatile");
    }};
}

pub fn sfence_vma() {
    unsafe { llvm_asm!("sfence.vma" ::: "memory" : "volatile") }
}

pub fn sfence_vma_addr(vaddr: u64) {
    unsafe { llvm_asm!("sfence.vma $0" :: "r"(vaddr) : "memory" : "volatile") }
}

pub fn barrier() {
    unsafe { llvm_asm!("" ::: "memory" : "volatile") }
}

pub fn fence_i() {
    unsafe { llvm_asm!("fence.i" :::: "volatile") }
}

pub fn wfi() {
    unsafe { llvm_asm!("wfi" :::: "volatile") }
}

/// Set the `sepc` CSR to the indicated value.
///
/// Since traps from S-mode always cause a hyperivsor panic, the value of `sstatus.spp` will always
/// be zero. Thus, mret will always cause a vmexit and so any value for sepc is safe.
pub fn set_sepc(value: u64) {
    unsafe { csrw!(sepc, value) }
}

/// Set the `sscratch` CSR. This is safe because `sscratch` does not impact processor execution.
pub fn set_sscratch(value: u64) {
    unsafe { csrw!(sscratch, value) }
}

/// Clear the indicated bits of `sip`. This is safe because interrupt state is not used to enforce
/// safety invariants.
pub fn clear_sip(mask: u64) {
    unsafe { csrc!(sip, mask) }
}

/// Set the FS bits of `sstatus`. This is safe because rvirt does not use hardware floating point
/// support.
pub fn set_sstatus_fs(new: u64) {
    unsafe { csrw!(sstatus, (new & STATUS_FS) | (csrr!(sstatus) & !STATUS_FS)) }
}


pub fn test_instructions() -> bool {
    //STORE	ra, (RVMSTATE_HOST_RA)(a0)
    // load!(ra,)
    // info!("{}",ld!(ra,a,sp));
    // info!("{}",sd!(ra,a,sp));
    // load!(a6,a,sp);
    true
}

pub fn generate_switch_entry() {
    info!("{}",sd!(ra,HOST_STATE_OFFSET+HOST_OFFSET_RA,a0));
	info!("{}",sd!(sp,HOST_STATE_OFFSET+HOST_OFFSET_SP,a0));
	info!("{}",sd!(gp,HOST_STATE_OFFSET+HOST_OFFSET_GP,a0));
	info!("{}",sd!(tp,HOST_STATE_OFFSET+HOST_OFFSET_TP,a0));
	info!("{}",sd!(s0,HOST_STATE_OFFSET+HOST_OFFSET_S0,a0));
	info!("{}",sd!(s1,HOST_STATE_OFFSET+HOST_OFFSET_S1,a0));
	info!("{}",sd!(a1,HOST_STATE_OFFSET+HOST_OFFSET_A1,a0));
	info!("{}",sd!(a2,HOST_STATE_OFFSET+HOST_OFFSET_A2,a0));
	info!("{}",sd!(a3,HOST_STATE_OFFSET+HOST_OFFSET_A3,a0));
	info!("{}",sd!(a4,HOST_STATE_OFFSET+HOST_OFFSET_A4,a0));
	info!("{}",sd!(a5,HOST_STATE_OFFSET+HOST_OFFSET_A5,a0));
	info!("{}",sd!(a6,HOST_STATE_OFFSET+HOST_OFFSET_A6,a0));
	info!("{}",sd!(a7,HOST_STATE_OFFSET+HOST_OFFSET_A7,a0));
	info!("{}",sd!(s2,HOST_STATE_OFFSET+HOST_OFFSET_S2,a0));
	info!("{}",sd!(s3,HOST_STATE_OFFSET+HOST_OFFSET_S3,a0));
	info!("{}",sd!(s4,HOST_STATE_OFFSET+HOST_OFFSET_S4,a0));
	info!("{}",sd!(s5,HOST_STATE_OFFSET+HOST_OFFSET_S5,a0));
	info!("{}",sd!(s6,HOST_STATE_OFFSET+HOST_OFFSET_S6,a0));
	info!("{}",sd!(s7,HOST_STATE_OFFSET+HOST_OFFSET_S7,a0));
	info!("{}",sd!(s8,HOST_STATE_OFFSET+HOST_OFFSET_S8,a0));
	info!("{}",sd!(s9,HOST_STATE_OFFSET+HOST_OFFSET_S9,a0));
	info!("{}",sd!(s10,HOST_STATE_OFFSET+HOST_OFFSET_S10,a0));
	info!("{}",sd!(s11,HOST_STATE_OFFSET+HOST_OFFSET_S11,a0));

    	/* Save Host and Restore Guest SSTATUS */
	info!("{}",ld!(t0,GUEST_STATE_OFFSET+GUEST_OFFSET_T0,a0));
	// info!("{}",csrrw!(t0,super::csr::sstatus,t0));
	info!("{}",sd!(t0,HOST_STATE_OFFSET+HOST_OFFSET_SSTATUS,a0));

	/* Save Host and Restore Guest HSTATUS */
	info!("{}",ld!(t1,GUEST_STATE_OFFSET+GUEST_OFFSET_T1,a0));
	// info!("{}",csrrw!(t1,super::csr::hstatus,t1));
	info!("{}",sd!(t1,HOST_OFFSET_HSTATUS,a0));

	/* Save Host and Restore Guest SCOUNTEREN */
	info!("{}",ld!(t2,GUEST_STATE_OFFSET+GUEST_OFFSET_T2,a0));
	// info!("{}",csrrw!(t2,super::csr::scounteren,t2));
	info!("{}",sd!(t2,HOST_OFFSET_SCOUNTEREN,a0));

	/* Save Host SSCRATCH and change it to struct kvm_vcpu_arch pointer */
	// info!("{}",csrrw!(t3,super::csr::sscratch,a0));
	info!("{}",sd!(t3,HOST_OFFSET_SSCRATCH,a0));

	/* Save Host STVEC and change it to return path */
	// info!("{}",la!(t4,"__kvm_switch_return"));
	// info!("{}",csrrw!(t4,super::csr::stvec,t4));
	info!("{}",sd!(t4,HOST_OFFSET_STVEC,a0));
    	/* Restore Guest SEPC */
	info!("{}",ld!(t0,GUEST_STATE_OFFSET+GUEST_OFFSET_SEPC,a0));
	// info!("{}",csrw!(CSR_SEPC,t0));


    /* Restore Guest GPRs (except A0) */
	info!("{}",ld!(ra,GUEST_STATE_OFFSET+GUEST_OFFSET_RA,a0));
	info!("{}",ld!(sp,GUEST_STATE_OFFSET+GUEST_OFFSET_SP,a0));
	info!("{}",ld!(gp,GUEST_STATE_OFFSET+GUEST_OFFSET_GP,a0));
	info!("{}",ld!(tp,GUEST_STATE_OFFSET+GUEST_OFFSET_TP,a0));
	info!("{}",ld!(t0,GUEST_STATE_OFFSET+GUEST_OFFSET_T0,a0));
	info!("{}",ld!(t1,GUEST_STATE_OFFSET+GUEST_OFFSET_T1,a0));
	info!("{}",ld!(t2,GUEST_STATE_OFFSET+GUEST_OFFSET_T2,a0));
	info!("{}",ld!(s0,GUEST_STATE_OFFSET+GUEST_OFFSET_S0,a0));
	info!("{}",ld!(s1,GUEST_STATE_OFFSET+GUEST_OFFSET_S1,a0));
	info!("{}",ld!(a1,GUEST_STATE_OFFSET+GUEST_OFFSET_A1,a0));
	info!("{}",ld!(a2,GUEST_STATE_OFFSET+GUEST_OFFSET_A2,a0));
	info!("{}",ld!(a3,GUEST_STATE_OFFSET+GUEST_OFFSET_A3,a0));
	info!("{}",ld!(a4,GUEST_STATE_OFFSET+GUEST_OFFSET_A4,a0));
	info!("{}",ld!(a5,GUEST_STATE_OFFSET+GUEST_OFFSET_A5,a0));
	info!("{}",ld!(a6,GUEST_STATE_OFFSET+GUEST_OFFSET_A6,a0));
	info!("{}",ld!(a7,GUEST_STATE_OFFSET+GUEST_OFFSET_A7,a0));
	info!("{}",ld!(s2,GUEST_STATE_OFFSET+GUEST_OFFSET_S2,a0));
	info!("{}",ld!(s3,GUEST_STATE_OFFSET+GUEST_OFFSET_S3,a0));
	info!("{}",ld!(s4,GUEST_STATE_OFFSET+GUEST_OFFSET_S4,a0));
	info!("{}",ld!(s5,GUEST_STATE_OFFSET+GUEST_OFFSET_S5,a0));
	info!("{}",ld!(s6,GUEST_STATE_OFFSET+GUEST_OFFSET_S6,a0));
	info!("{}",ld!(s7,GUEST_STATE_OFFSET+GUEST_OFFSET_S7,a0));
	info!("{}",ld!(s8,GUEST_STATE_OFFSET+GUEST_OFFSET_S8,a0));
	info!("{}",ld!(s9,GUEST_STATE_OFFSET+GUEST_OFFSET_S9,a0));
	info!("{}",ld!(s10,GUEST_STATE_OFFSET+GUEST_OFFSET_S10,a0));
	info!("{}",ld!(s11,GUEST_STATE_OFFSET+GUEST_OFFSET_S11,a0));
	info!("{}",ld!(t3,GUEST_STATE_OFFSET+GUEST_OFFSET_T3,a0));
	info!("{}",ld!(t4,GUEST_STATE_OFFSET+GUEST_OFFSET_T4,a0));
	info!("{}",ld!(t5,GUEST_STATE_OFFSET+GUEST_OFFSET_T5,a0));
	info!("{}",ld!(t6,GUEST_STATE_OFFSET+GUEST_OFFSET_T6,a0));

    /* Restore Guest A0 */
	info!("{}",ld!(a0,GUEST_STATE_OFFSET+GUEST_OFFSET_A0,a0));

	/* Resume Guest */
	info!("sret");
}

pub fn generate_switch_exit(){
    // Store the guest registers not covered by the VMCS. At this point,
    // 
    /* Swap Guest A0 with SSCRATCH */
	info!("csrrw\ta0, CSR_SSCRATCH, a0");
	// csrrw	a0, CSR_SSCRATCH, a0

	/* Save Guest GPRs (except A0) */
	info!("{}",sd!(ra,GUEST_STATE_OFFSET+GUEST_OFFSET_RA,a0));
	info!("{}",sd!(sp,GUEST_STATE_OFFSET+GUEST_OFFSET_SP,a0));
	info!("{}",sd!(gp,GUEST_STATE_OFFSET+GUEST_OFFSET_GP,a0));
	info!("{}",sd!(tp,GUEST_STATE_OFFSET+GUEST_OFFSET_TP,a0));
	info!("{}",sd!(t0,GUEST_STATE_OFFSET+GUEST_OFFSET_T0,a0));
	info!("{}",sd!(t1,GUEST_STATE_OFFSET+GUEST_OFFSET_T1,a0));
	info!("{}",sd!(t2,GUEST_STATE_OFFSET+GUEST_OFFSET_T2,a0));
	info!("{}",sd!(s0,GUEST_STATE_OFFSET+GUEST_OFFSET_S0,a0));
	info!("{}",sd!(s1,GUEST_STATE_OFFSET+GUEST_OFFSET_S1,a0));
	info!("{}",sd!(a1,GUEST_STATE_OFFSET+GUEST_OFFSET_A1,a0));
	info!("{}",sd!(a2,GUEST_STATE_OFFSET+GUEST_OFFSET_A2,a0));
	info!("{}",sd!(a3,GUEST_STATE_OFFSET+GUEST_OFFSET_A3,a0));
	info!("{}",sd!(a4,GUEST_STATE_OFFSET+GUEST_OFFSET_A4,a0));
	info!("{}",sd!(a5,GUEST_STATE_OFFSET+GUEST_OFFSET_A5,a0));
	info!("{}",sd!(a6,GUEST_STATE_OFFSET+GUEST_OFFSET_A6,a0));
	info!("{}",sd!(a7,GUEST_STATE_OFFSET+GUEST_OFFSET_A7,a0));
	info!("{}",sd!(s2,GUEST_STATE_OFFSET+GUEST_OFFSET_S2,a0));
	info!("{}",sd!(s3,GUEST_STATE_OFFSET+GUEST_OFFSET_S3,a0));
	info!("{}",sd!(s4,GUEST_STATE_OFFSET+GUEST_OFFSET_S4,a0));
	info!("{}",sd!(s5,GUEST_STATE_OFFSET+GUEST_OFFSET_S5,a0));
	info!("{}",sd!(s6,GUEST_STATE_OFFSET+GUEST_OFFSET_S6,a0));
	info!("{}",sd!(s7,GUEST_STATE_OFFSET+GUEST_OFFSET_S7,a0));
	info!("{}",sd!(s8,GUEST_STATE_OFFSET+GUEST_OFFSET_S8,a0));
	info!("{}",sd!(s9,GUEST_STATE_OFFSET+GUEST_OFFSET_S9,a0));
	info!("{}",sd!(s10,GUEST_STATE_OFFSET+GUEST_OFFSET_S10,a0));
	info!("{}",sd!(s11,GUEST_STATE_OFFSET+GUEST_OFFSET_S11,a0));
	info!("{}",sd!(t3,GUEST_STATE_OFFSET+GUEST_OFFSET_T3,a0));
	info!("{}",sd!(t4,GUEST_STATE_OFFSET+GUEST_OFFSET_T4,a0));
	info!("{}",sd!(t5,GUEST_STATE_OFFSET+GUEST_OFFSET_T5,a0));
	info!("{}",sd!(t6,GUEST_STATE_OFFSET+GUEST_OFFSET_T6,a0));

	/* Save Guest SEPC */
	info!("csrr	t0, CSR_SEPC");
	info!("{}",sd!(t0,GUEST_STATE_OFFSET+GUEST_OFFSET_SEPC,a0));

	/* Restore Host STVEC */
	info!("{}",ld!(t1,HOST_STATE_OFFSET+HOST_OFFSET_STVEC,a0));
	info!("csrw	CSR_STVEC, t1");

	/* Save Guest A0 and Restore Host SSCRATCH */
	info!("{}",ld!(t2,HOST_STATE_OFFSET+HOST_OFFSET_SSCRATCH,a0));
	info!("csrrw	t2, CSR_SSCRATCH, t2");
	info!("{}",sd!(t2,GUEST_STATE_OFFSET+GUEST_OFFSET_A0,a0));

	/* Save Guest and Restore Host SCOUNTEREN */
	info!("{}",ld!(t3,HOST_STATE_OFFSET+HOST_OFFSET_SCOUNTEREN,a0));
	info!("csrrw	t3, CSR_SCOUNTEREN, t3");
	info!("{}",sd!(t3,GUEST_STATE_OFFSET+GUEST_OFFSET_SCOUNTEREN,a0));

	/* Save Guest and Restore Host HSTATUS */
	info!("{}",ld!(t4,HOST_STATE_OFFSET+HOST_OFFSET_HSTATUS,a0));
	info!("csrrw	t4, CSR_HSTATUS, t4");
	info!("{}",sd!(t4,GUEST_STATE_OFFSET+GUEST_OFFSET_HSTATUS,a0));

	/* Save Guest and Restore Host SSTATUS */
	info!("{}",ld!(t5,HOST_STATE_OFFSET+HOST_OFFSET_SSTATUS,a0));
	info!("csrrw	t5, CSR_SSTATUS, t5");
	info!("{}",sd!(t5,GUEST_STATE_OFFSET+GUEST_OFFSET_SSTATUS,a0));

	/* Restore Host GPRs (except A0 and T0-T6) */
	info!("{}",ld!(ra,HOST_STATE_OFFSET+HOST_OFFSET_RA,a0));
	info!("{}",ld!(sp,HOST_STATE_OFFSET+HOST_OFFSET_SP,a0));
	info!("{}",ld!(gp,HOST_STATE_OFFSET+HOST_OFFSET_GP,a0));
	info!("{}",ld!(tp,HOST_STATE_OFFSET+HOST_OFFSET_TP,a0));
	info!("{}",ld!(s0,HOST_STATE_OFFSET+HOST_OFFSET_S0,a0));
	info!("{}",ld!(s1,HOST_STATE_OFFSET+HOST_OFFSET_S1,a0));
	info!("{}",ld!(a1,HOST_STATE_OFFSET+HOST_OFFSET_A1,a0));
	info!("{}",ld!(a2,HOST_STATE_OFFSET+HOST_OFFSET_A2,a0));
	info!("{}",ld!(a3,HOST_STATE_OFFSET+HOST_OFFSET_A3,a0));
	info!("{}",ld!(a4,HOST_STATE_OFFSET+HOST_OFFSET_A4,a0));
	info!("{}",ld!(a5,HOST_STATE_OFFSET+HOST_OFFSET_A5,a0));
	info!("{}",ld!(a6,HOST_STATE_OFFSET+HOST_OFFSET_A6,a0));
	info!("{}",ld!(a7,HOST_STATE_OFFSET+HOST_OFFSET_A7,a0));
	info!("{}",ld!(s2,HOST_STATE_OFFSET+HOST_OFFSET_S2,a0));
	info!("{}",ld!(s3,HOST_STATE_OFFSET+HOST_OFFSET_S3,a0));
	info!("{}",ld!(s4,HOST_STATE_OFFSET+HOST_OFFSET_S4,a0));
	info!("{}",ld!(s5,HOST_STATE_OFFSET+HOST_OFFSET_S5,a0));
	info!("{}",ld!(s6,HOST_STATE_OFFSET+HOST_OFFSET_S6,a0));
	info!("{}",ld!(s7,HOST_STATE_OFFSET+HOST_OFFSET_S7,a0));
	info!("{}",ld!(s8,HOST_STATE_OFFSET+HOST_OFFSET_S8,a0));
	info!("{}",ld!(s9,HOST_STATE_OFFSET+HOST_OFFSET_S9,a0));
	info!("{}",ld!(s10,HOST_STATE_OFFSET+HOST_OFFSET_S10,a0));
	info!("{}",ld!(s11,HOST_STATE_OFFSET+HOST_OFFSET_S11,a0));

	/* Return to Rust code */
	info!("ret")
}