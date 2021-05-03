#![allow(clippy::unnecessary_wraps)]

mod bits;
mod csr;
mod ept;
mod guest;
mod vcpu;
mod config;
#[macro_use]
mod instructions;
mod regs;

use regs::*;

pub use ept::EPageTable as ArchRvmPageTable;
pub use guest::Guest;
pub use vcpu::Vcpu;

use riscv::register::{
    misa,
};

//判断是否在硬件上支持Hypervisor
pub fn check_hypervisor_feature() -> bool {
    //访问misa寄存器
    // 这段代码暂时注释掉，因为misa寄存器只有在M态才能访问。
    // 现在真的只能假装它一定好用
    // info!("rvm info test");
    // if let Some(isa) = misa::read(){
    //     info!("rvm has H extension:{}",isa.has_extension('H'));
    // }else{
    //     info!("error");
    // }
    true
}

pub fn test_instructions() -> bool {
    //STORE	ra, (RVMSTATE_HOST_RA)(a0)
    // load!(ra,)
    // info!("{}",ld!(ra,a,sp));
    // info!("{}",sd!(ra,a,sp));
    // load!(a6,a,sp);
    true
}

/*


*/
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