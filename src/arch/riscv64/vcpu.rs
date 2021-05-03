use super::{
    Guest,
};

use alloc::{
    sync::Arc,
};

use super::config::*;
use crate::{packet::RvmExitPacket, RvmError, RvmResult, VcpuIo, VcpuState};

use super::regs::*;

//===================================================================================

/// Represents a virtual CPU within a guest.
pub struct Vcpu {
	guest: Arc<Guest>,//自己对应的Guest要保存起来方便找
}

#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct CPUContext{//0~34
    pub zero : u64,//0
	pub ra : u64,//1
	pub sp : u64,//2
	pub gp : u64,//3
	pub tp : u64,//4
	pub t0 : u64,//5
	pub t1 : u64,//6
	pub t2 : u64,//7
	pub s0 : u64,//8
	pub s1 : u64,//9
	pub a0 : u64,//10
	pub a1 : u64,//11
	pub a2 : u64,//12
	pub a3 : u64,//13
	pub a4 : u64,//14
	pub a5 : u64,//15
	pub a6 : u64,//16
	pub a7 : u64,//17
	pub s2 : u64,//18
	pub s3 : u64,//19
	pub s4 : u64,//20
	pub s5 : u64,//21
	pub s6 : u64,//22
	pub s7 : u64,//23
	pub s8 : u64,//24
	pub s9 : u64,//25
	pub s10 : u64,//26
	pub s11 : u64,//27
	pub t3 : u64,//28
	pub t4 : u64,//29
	pub t5 : u64,//30
	pub t6 : u64,//31
	pub sepc : u64,//32
	pub sstatus : u64,//33
	pub hstatus : u64,//34
	// union __riscv_fp_state fp;
}

#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct CPUCsr{
    pub vsstatus: u64,//0
	pub hie: u64,//1
	pub vstvec: u64,//2
	pub vsscratch: u64,//3
	pub vsepc: u64,//4
	pub vscause: u64,//5
	pub vstval: u64,//6
	pub hvip: u64,//7
	pub vsatp: u64,//8
	pub scounteren: u64,//9
}

/// Holds the register state used to restore a guest.
#[repr(C,packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct GuestState {
	pub zero : u64,//0
	pub ra : u64,//1
	pub sp : u64,//2
	pub gp : u64,//3
	pub tp : u64,//4
	pub t0 : u64,//5
	pub t1 : u64,//6
	pub t2 : u64,//7
	pub s0 : u64,//8
	pub s1 : u64,//9
	pub a0 : u64,//10
	pub a1 : u64,//11
	pub a2 : u64,//12
	pub a3 : u64,//13
	pub a4 : u64,//14
	pub a5 : u64,//15
	pub a6 : u64,//16
	pub a7 : u64,//17
	pub s2 : u64,//18
	pub s3 : u64,//19
	pub s4 : u64,//20
	pub s5 : u64,//21
	pub s6 : u64,//22
	pub s7 : u64,//23
	pub s8 : u64,//24
	pub s9 : u64,//25
	pub s10 : u64,//26
	pub s11 : u64,//27
	pub t3 : u64,//28
	pub t4 : u64,//29
	pub t5 : u64,//30
	pub t6 : u64,//31

	pub sepc : u64,//32
	pub sstatus : u64,//33
	pub hstatus : u64,//34
	pub scounteren: u64,//35
}

// 
/// Holds the register state used to restore a guest.
#[repr(C,packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct HostState {
	pub zero : u64,//0
	pub ra : u64,//1
	pub sp : u64,//2
	pub gp : u64,//3
	pub tp : u64,//4
	pub s0 : u64,//5
	pub s1 : u64,//6
	pub a0 : u64,//7
	pub a1 : u64,//8
	pub a2 : u64,//9
	pub a3 : u64,//10
	pub a4 : u64,//11
	pub a5 : u64,//12
	pub a6 : u64,//13
	pub a7 : u64,//14
	pub s2 : u64,//15
	pub s3 : u64,//16
	pub s4 : u64,//17
	pub s5 : u64,//18
	pub s6 : u64,//19
	pub s7 : u64,//20
	pub s8 : u64,//21
	pub s9 : u64,//22
	pub s10 : u64,//23
	pub s11 : u64,//24

	pub sepc : u64,//25
	pub sstatus : u64,//26
	pub hstatus : u64,//27

	pub sscratch: u64,//28
    pub stvec: u64,//29
    pub scounteren: u64,//30
}

/// Host and guest cpu register states.
/// [WARN]修改布局会导致汇编代码出错
#[repr(C,packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct RvmStateRiscv64 {
	// //host
    // host_context: CPUContext,//0~34
	// host_sscratch: u64,//35
    // host_stvec: u64,//36
    // host_scounteren: u64,//37
	// //guest
	// guest_context: CPUContext,//38~72
	// guest_csr: CPUCsr,//73~82
	host_state: HostState,//0~30
	guest_state: GuestState,//31~66
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
    //todo : resume

	pub fn resume(&mut self) -> RvmResult<RvmExitPacket> {
        // loop {
        //     let mut vmcs = AutoVmcs::new(self.vmcs_page.phys_addr())?;

        //     self.interrupt_state.try_inject_interrupt(&mut vmcs)?;
        //     // TODO: save/restore guest extended registers (x87/SSE)

        //     // VM Entry
        //     self.running.store(true, Ordering::SeqCst);
        //     trace!("[RVM] vmx entry");
        //     let has_err = unsafe { vmx_entry(&mut self.vmx_state) };
        //     trace!("[RVM] vmx exit");
        //     self.running.store(false, Ordering::SeqCst);

        //     if has_err {
        //         warn!(
        //             "[RVM] VCPU resume failed: {:?}",
        //             VmInstructionError::from(vmcs.read32(VM_INSTRUCTION_ERROR))
        //         );
        //         return Err(RvmError::Internal);
        //     }

        //     // VM Exit
        //     self.vmx_state.resume = true;
        //     match vmexit_handler(
        //         &mut vmcs,
        //         &mut self.vmx_state.guest_state,
        //         &mut self.interrupt_state,
        //         &self.guest.gpm,
        //         &self.guest.traps,
        //     )? {
        //         Some(packet) => return Ok(packet), // forward to user mode handler
        //         None => continue,
        //     }
        // }
		Err(RvmError::NotFound)
    }


}

global_asm!(include_str!("entry.S"));

extern "C" {
    //todo怎么把这里改成地址
    fn __riscv64_entry(_rvm_state: &mut RvmStateRiscv64) -> bool;
    /// This is effectively the second-half of vmx_entry. When we return from a
    /// VM exit, vmx_state argument is stored in RSP. We use this to restore the
    /// stack and registers to the state they were in when vmx_entry was called.
    fn __riscv64_exit() -> bool;
}