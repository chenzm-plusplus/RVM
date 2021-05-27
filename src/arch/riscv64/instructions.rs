
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
    sd  \REG, \offset*REGBYTES(A0)
.macro LOAD reg, offset, regbytes, base
    ld  \reg, \offset*REGBYTES(A0)
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
    //STORE	ra, (RVMSTATE_HOST_RA)(A0)
    // load!(ra,)
    // info!("{}",ld!(ra,a,sp));
    // info!("{}",sd!(RA,a,sp));
    // load!(a6,a,sp);
    true
}

pub fn generate_switch_entry() {
    info!("{}",sd!(RA,HOST_STATE_OFFSET+HOST_OFFSET_RA,A0));
	info!("{}",sd!(SP,HOST_STATE_OFFSET+HOST_OFFSET_SP,A0));
	info!("{}",sd!(GP,HOST_STATE_OFFSET+HOST_OFFSET_GP,A0));
	info!("{}",sd!(TP,HOST_STATE_OFFSET+HOST_OFFSET_TP,A0));
	info!("{}",sd!(S0,HOST_STATE_OFFSET+HOST_OFFSET_S0,A0));
	info!("{}",sd!(S1,HOST_STATE_OFFSET+HOST_OFFSET_S1,A0));
	info!("{}",sd!(A1,HOST_STATE_OFFSET+HOST_OFFSET_A1,A0));
	info!("{}",sd!(A2,HOST_STATE_OFFSET+HOST_OFFSET_A2,A0));
	info!("{}",sd!(A3,HOST_STATE_OFFSET+HOST_OFFSET_A3,A0));
	info!("{}",sd!(A4,HOST_STATE_OFFSET+HOST_OFFSET_A4,A0));
	info!("{}",sd!(A5,HOST_STATE_OFFSET+HOST_OFFSET_A5,A0));
	info!("{}",sd!(A6,HOST_STATE_OFFSET+HOST_OFFSET_A6,A0));
	info!("{}",sd!(A7,HOST_STATE_OFFSET+HOST_OFFSET_A7,A0));
	info!("{}",sd!(S2,HOST_STATE_OFFSET+HOST_OFFSET_S2,A0));
	info!("{}",sd!(S3,HOST_STATE_OFFSET+HOST_OFFSET_S3,A0));
	info!("{}",sd!(S4,HOST_STATE_OFFSET+HOST_OFFSET_S4,A0));
	info!("{}",sd!(S5,HOST_STATE_OFFSET+HOST_OFFSET_S5,A0));
	info!("{}",sd!(S6,HOST_STATE_OFFSET+HOST_OFFSET_S6,A0));
	info!("{}",sd!(S7,HOST_STATE_OFFSET+HOST_OFFSET_S7,A0));
	info!("{}",sd!(S8,HOST_STATE_OFFSET+HOST_OFFSET_S8,A0));
	info!("{}",sd!(S9,HOST_STATE_OFFSET+HOST_OFFSET_S9,A0));
	info!("{}",sd!(S10,HOST_STATE_OFFSET+HOST_OFFSET_S10,A0));
	info!("{}",sd!(S11,HOST_STATE_OFFSET+HOST_OFFSET_S11,A0));

    	/* Save Host and Restore Guest SSTATUS */
	info!("{}",ld!(T0,GUEST_STATE_OFFSET+GUEST_OFFSET_T0,A0));
	// info!("{}",csrrw!(t0,super::csr::sstatus,t0));
	info!("{}",sd!(T0,HOST_STATE_OFFSET+HOST_OFFSET_SSTATUS,A0));

	/* Save Host and Restore Guest HSTATUS */
	info!("{}",ld!(T1,GUEST_STATE_OFFSET+GUEST_OFFSET_T1,A0));
	// info!("{}",csrrw!(t1,super::csr::hstatus,t1));
	info!("{}",sd!(T1,HOST_OFFSET_HSTATUS,A0));

	/* Save Host and Restore Guest SCOUNTEREN */
	info!("{}",ld!(T2,GUEST_STATE_OFFSET+GUEST_OFFSET_T2,A0));
	// info!("{}",csrrw!(t2,super::csr::scounteren,t2));
	info!("{}",sd!(T2,HOST_OFFSET_SCOUNTEREN,A0));

	/* Save Host SSCRATCH and change it to struct kvm_vcpu_arch pointer */
	// info!("{}",csrrw!(t3,super::csr::sscratch,A0));
	info!("{}",sd!(T3,HOST_OFFSET_SSCRATCH,A0));

	/* Save Host STVEC and change it to return path */
	// info!("{}",la!(t4,"__kvm_switch_return"));
	// info!("{}",csrrw!(t4,super::csr::stvec,t4));
	info!("{}",sd!(T4,HOST_OFFSET_STVEC,A0));
    	/* Restore Guest SEPC */
	info!("{}",ld!(T0,GUEST_STATE_OFFSET+GUEST_OFFSET_SEPC,A0));
	// info!("{}",csrw!(CSR_SEPC,t0));


    /* Restore Guest GPRs (except A0) */
	info!("{}",ld!(RA,GUEST_STATE_OFFSET+GUEST_OFFSET_RA,A0));
	info!("{}",ld!(SP,GUEST_STATE_OFFSET+GUEST_OFFSET_SP,A0));
	info!("{}",ld!(GP,GUEST_STATE_OFFSET+GUEST_OFFSET_GP,A0));
	info!("{}",ld!(TP,GUEST_STATE_OFFSET+GUEST_OFFSET_TP,A0));
	info!("{}",ld!(T0,GUEST_STATE_OFFSET+GUEST_OFFSET_T0,A0));
	info!("{}",ld!(T1,GUEST_STATE_OFFSET+GUEST_OFFSET_T1,A0));
	info!("{}",ld!(T2,GUEST_STATE_OFFSET+GUEST_OFFSET_T2,A0));
	info!("{}",ld!(S0,GUEST_STATE_OFFSET+GUEST_OFFSET_S0,A0));
	info!("{}",ld!(S1,GUEST_STATE_OFFSET+GUEST_OFFSET_S1,A0));
	info!("{}",ld!(A1,GUEST_STATE_OFFSET+GUEST_OFFSET_A1,A0));
	info!("{}",ld!(A2,GUEST_STATE_OFFSET+GUEST_OFFSET_A2,A0));
	info!("{}",ld!(A3,GUEST_STATE_OFFSET+GUEST_OFFSET_A3,A0));
	info!("{}",ld!(A4,GUEST_STATE_OFFSET+GUEST_OFFSET_A4,A0));
	info!("{}",ld!(A5,GUEST_STATE_OFFSET+GUEST_OFFSET_A5,A0));
	info!("{}",ld!(A6,GUEST_STATE_OFFSET+GUEST_OFFSET_A6,A0));
	info!("{}",ld!(A7,GUEST_STATE_OFFSET+GUEST_OFFSET_A7,A0));
	info!("{}",ld!(S2,GUEST_STATE_OFFSET+GUEST_OFFSET_S2,A0));
	info!("{}",ld!(S3,GUEST_STATE_OFFSET+GUEST_OFFSET_S3,A0));
	info!("{}",ld!(S4,GUEST_STATE_OFFSET+GUEST_OFFSET_S4,A0));
	info!("{}",ld!(S5,GUEST_STATE_OFFSET+GUEST_OFFSET_S5,A0));
	info!("{}",ld!(S6,GUEST_STATE_OFFSET+GUEST_OFFSET_S6,A0));
	info!("{}",ld!(S7,GUEST_STATE_OFFSET+GUEST_OFFSET_S7,A0));
	info!("{}",ld!(S8,GUEST_STATE_OFFSET+GUEST_OFFSET_S8,A0));
	info!("{}",ld!(S9,GUEST_STATE_OFFSET+GUEST_OFFSET_S9,A0));
	info!("{}",ld!(S10,GUEST_STATE_OFFSET+GUEST_OFFSET_S10,A0));
	info!("{}",ld!(S11,GUEST_STATE_OFFSET+GUEST_OFFSET_S11,A0));
	info!("{}",ld!(T3,GUEST_STATE_OFFSET+GUEST_OFFSET_T3,A0));
	info!("{}",ld!(T4,GUEST_STATE_OFFSET+GUEST_OFFSET_T4,A0));
	info!("{}",ld!(T5,GUEST_STATE_OFFSET+GUEST_OFFSET_T5,A0));
	info!("{}",ld!(T6,GUEST_STATE_OFFSET+GUEST_OFFSET_T6,A0));

    /* Restore Guest A0 */
	info!("{}",ld!(A0,GUEST_STATE_OFFSET+GUEST_OFFSET_A0,A0));

	/* Resume Guest */
	info!("sret");
}

pub fn generate_switch_exit(){
    // Store the guest registers not covered by the VMCS. At this point,
    // 
    /* Swap Guest A0 with SSCRATCH */
	info!("csrrw\ta0, CSR_SSCRATCH, A0");
	// csrrw	A0, CSR_SSCRATCH, A0

	/* Save Guest GPRs (except A0) */
	info!("{}",sd!(RA,GUEST_STATE_OFFSET+GUEST_OFFSET_RA,A0));
	info!("{}",sd!(SP,GUEST_STATE_OFFSET+GUEST_OFFSET_SP,A0));
	info!("{}",sd!(GP,GUEST_STATE_OFFSET+GUEST_OFFSET_GP,A0));
	info!("{}",sd!(TP,GUEST_STATE_OFFSET+GUEST_OFFSET_TP,A0));
	info!("{}",sd!(T0,GUEST_STATE_OFFSET+GUEST_OFFSET_T0,A0));
	info!("{}",sd!(T1,GUEST_STATE_OFFSET+GUEST_OFFSET_T1,A0));
	info!("{}",sd!(T2,GUEST_STATE_OFFSET+GUEST_OFFSET_T2,A0));
	info!("{}",sd!(S0,GUEST_STATE_OFFSET+GUEST_OFFSET_S0,A0));
	info!("{}",sd!(S1,GUEST_STATE_OFFSET+GUEST_OFFSET_S1,A0));
	info!("{}",sd!(A1,GUEST_STATE_OFFSET+GUEST_OFFSET_A1,A0));
	info!("{}",sd!(A2,GUEST_STATE_OFFSET+GUEST_OFFSET_A2,A0));
	info!("{}",sd!(A3,GUEST_STATE_OFFSET+GUEST_OFFSET_A3,A0));
	info!("{}",sd!(A4,GUEST_STATE_OFFSET+GUEST_OFFSET_A4,A0));
	info!("{}",sd!(A5,GUEST_STATE_OFFSET+GUEST_OFFSET_A5,A0));
	info!("{}",sd!(A6,GUEST_STATE_OFFSET+GUEST_OFFSET_A6,A0));
	info!("{}",sd!(A7,GUEST_STATE_OFFSET+GUEST_OFFSET_A7,A0));
	info!("{}",sd!(S2,GUEST_STATE_OFFSET+GUEST_OFFSET_S2,A0));
	info!("{}",sd!(S3,GUEST_STATE_OFFSET+GUEST_OFFSET_S3,A0));
	info!("{}",sd!(S4,GUEST_STATE_OFFSET+GUEST_OFFSET_S4,A0));
	info!("{}",sd!(S5,GUEST_STATE_OFFSET+GUEST_OFFSET_S5,A0));
	info!("{}",sd!(S6,GUEST_STATE_OFFSET+GUEST_OFFSET_S6,A0));
	info!("{}",sd!(S7,GUEST_STATE_OFFSET+GUEST_OFFSET_S7,A0));
	info!("{}",sd!(S8,GUEST_STATE_OFFSET+GUEST_OFFSET_S8,A0));
	info!("{}",sd!(S9,GUEST_STATE_OFFSET+GUEST_OFFSET_S9,A0));
	info!("{}",sd!(S10,GUEST_STATE_OFFSET+GUEST_OFFSET_S10,A0));
	info!("{}",sd!(S11,GUEST_STATE_OFFSET+GUEST_OFFSET_S11,A0));
	info!("{}",sd!(T3,GUEST_STATE_OFFSET+GUEST_OFFSET_T3,A0));
	info!("{}",sd!(T4,GUEST_STATE_OFFSET+GUEST_OFFSET_T4,A0));
	info!("{}",sd!(T5,GUEST_STATE_OFFSET+GUEST_OFFSET_T5,A0));
	info!("{}",sd!(T6,GUEST_STATE_OFFSET+GUEST_OFFSET_T6,A0));

	/* Save Guest SEPC */
	info!("csrr	t0, CSR_SEPC");
	info!("{}",sd!(T0,GUEST_STATE_OFFSET+GUEST_OFFSET_SEPC,A0));

	/* Restore Host STVEC */
	info!("{}",ld!(T1,HOST_STATE_OFFSET+HOST_OFFSET_STVEC,A0));
	info!("csrw	CSR_STVEC, t1");

	/* Save Guest A0 and Restore Host SSCRATCH */
	info!("{}",ld!(T2,HOST_STATE_OFFSET+HOST_OFFSET_SSCRATCH,A0));
	info!("csrrw	t2, CSR_SSCRATCH, t2");
	info!("{}",sd!(T2,GUEST_STATE_OFFSET+GUEST_OFFSET_A0,A0));

	/* Save Guest and Restore Host SCOUNTEREN */
	info!("{}",ld!(T3,HOST_STATE_OFFSET+HOST_OFFSET_SCOUNTEREN,A0));
	info!("csrrw	t3, CSR_SCOUNTEREN, t3");
	info!("{}",sd!(T3,GUEST_STATE_OFFSET+GUEST_OFFSET_SCOUNTEREN,A0));

	/* Save Guest and Restore Host HSTATUS */
	info!("{}",ld!(T4,HOST_STATE_OFFSET+HOST_OFFSET_HSTATUS,A0));
	info!("csrrw	t4, CSR_HSTATUS, t4");
	info!("{}",sd!(T4,GUEST_STATE_OFFSET+GUEST_OFFSET_HSTATUS,A0));

	/* Save Guest and Restore Host SSTATUS */
	info!("{}",ld!(T5,HOST_STATE_OFFSET+HOST_OFFSET_SSTATUS,A0));
	info!("csrrw	t5, CSR_SSTATUS, t5");
	info!("{}",sd!(T5,GUEST_STATE_OFFSET+GUEST_OFFSET_SSTATUS,A0));

	/* Restore Host GPRs (except A0 and T0-T6) */
	info!("{}",ld!(RA,HOST_STATE_OFFSET+HOST_OFFSET_RA,A0));
	info!("{}",ld!(SP,HOST_STATE_OFFSET+HOST_OFFSET_SP,A0));
	info!("{}",ld!(GP,HOST_STATE_OFFSET+HOST_OFFSET_GP,A0));
	info!("{}",ld!(TP,HOST_STATE_OFFSET+HOST_OFFSET_TP,A0));
	info!("{}",ld!(S0,HOST_STATE_OFFSET+HOST_OFFSET_S0,A0));
	info!("{}",ld!(S1,HOST_STATE_OFFSET+HOST_OFFSET_S1,A0));
	info!("{}",ld!(A1,HOST_STATE_OFFSET+HOST_OFFSET_A1,A0));
	info!("{}",ld!(A2,HOST_STATE_OFFSET+HOST_OFFSET_A2,A0));
	info!("{}",ld!(A3,HOST_STATE_OFFSET+HOST_OFFSET_A3,A0));
	info!("{}",ld!(A4,HOST_STATE_OFFSET+HOST_OFFSET_A4,A0));
	info!("{}",ld!(A5,HOST_STATE_OFFSET+HOST_OFFSET_A5,A0));
	info!("{}",ld!(A6,HOST_STATE_OFFSET+HOST_OFFSET_A6,A0));
	info!("{}",ld!(A7,HOST_STATE_OFFSET+HOST_OFFSET_A7,A0));
	info!("{}",ld!(S2,HOST_STATE_OFFSET+HOST_OFFSET_S2,A0));
	info!("{}",ld!(S3,HOST_STATE_OFFSET+HOST_OFFSET_S3,A0));
	info!("{}",ld!(S4,HOST_STATE_OFFSET+HOST_OFFSET_S4,A0));
	info!("{}",ld!(S5,HOST_STATE_OFFSET+HOST_OFFSET_S5,A0));
	info!("{}",ld!(S6,HOST_STATE_OFFSET+HOST_OFFSET_S6,A0));
	info!("{}",ld!(S7,HOST_STATE_OFFSET+HOST_OFFSET_S7,A0));
	info!("{}",ld!(S8,HOST_STATE_OFFSET+HOST_OFFSET_S8,A0));
	info!("{}",ld!(S9,HOST_STATE_OFFSET+HOST_OFFSET_S9,A0));
	info!("{}",ld!(S10,HOST_STATE_OFFSET+HOST_OFFSET_S10,A0));
	info!("{}",ld!(S11,HOST_STATE_OFFSET+HOST_OFFSET_S11,A0));

	/* Return to Rust code */
	info!("ret")
}