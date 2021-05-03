use super::{
    Guest,
};

use alloc::{
    sync::Arc,
	boxed::Box,
};

use riscv::register::sstatus::{
	Sstatus,SPP,self,
};

use core::fmt;
use core::pin::Pin;
use core::sync::atomic::{AtomicBool, Ordering};

use super::config::*;
use crate::{packet::RvmExitPacket, RvmError, RvmResult, VcpuIo, VcpuState};

use super::regs::*;
use super::test::*;

//===================================================================================

/// Represents a virtual CPU within a guest.
pub struct Vcpu {
	guest: Arc<Guest>,
	running: AtomicBool,
	rvmstate_riscv64: Pin<Box<RvmStateRiscv64>>,//就是这块内存放在一个堆上，pin保证了这块内存不会被移动
	// rvmstate_riscv64: RvmStateRiscv64,
}

impl fmt::Debug for Vcpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut f = f.debug_struct("Vcpu");
        f.field("guest", &(self.guest.as_ref() as *const _ as usize))
            .field("running", &self.running)
            .field("rvmstate_riscv64", &self.rvmstate_riscv64)
            .finish()
    }
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

/// Host and guest cpu register states.
/// [WARN]修改布局会导致汇编代码出错
#[repr(C,packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct RvmStateRiscv64 {
	host_state: HostState,//0~30
	guest_state: GuestState,//31~66
}

#[no_mangle]
unsafe extern "C" 
    fn test_switch(){
        info!("[RVM] switch entry success!");
		__test();
    }

impl Vcpu{
	pub fn new(entry: u64, guest: Arc<Guest>) -> RvmResult<Self> {
        // if entry > guest.gpm.size() {
        //     return Err(RvmError::InvalidParam);
        // }
        let mut vcpu = Self {
			running: AtomicBool::new(false),
            guest: guest.clone(),
			rvmstate_riscv64: Box::pin(RvmStateRiscv64::default()),
			// rvmstate_riscv64: RvmStateRiscv64::default(),
        };
        vcpu.init(entry)?;

        Ok(vcpu)
    }
	fn init(&mut self, entry: u64) -> RvmResult {
        Ok(())
    }
    //todo : resume

	pub fn resume(&mut self) -> RvmResult<RvmExitPacket> {
		info!("vcpu::resume");

		// VM Entry
		self.running.store(true, Ordering::SeqCst);
		
		// sstatus::
		//测试一下！
		self.rvmstate_riscv64.guest_state.sepc = test_switch as u64;
		self.rvmstate_riscv64.guest_state.sstatus = 0x8000000000006100 as u64;

		info!("[RVM] riscv64 entry");

		//test
		let rvmstate_riscv64_address = &self.rvmstate_riscv64.host_state.zero as *const _ as u64;
		trace!("[RVM] a0 is {:#x}",&self.rvmstate_riscv64.host_state.a0 as *const _ as u64);
		trace!("[RVM] guest_sepc address is {:#x}",&self.rvmstate_riscv64.host_state.zero as *const _ as u64 + 504);
		let guest_sepc_address = &self.rvmstate_riscv64.guest_state.sepc as *const _ as u64;
		let guest_sstatus_address = &self.rvmstate_riscv64.guest_state.sstatus as *const _ as u64;
		let host_stvec_address = &self.rvmstate_riscv64.host_state.stvec as *const _ as u64;
		info!("[RVM] rvmstate_riscv64 address is {:#x}",rvmstate_riscv64_address);
		info!("[RVM] host_stvec address is {:#x}",host_stvec_address);
		info!("[RVM] guest_sstatus address is {:#x}",guest_sstatus_address);
		info!("[RVM] host_stvec offset is {}, {:#x}",host_stvec_address-rvmstate_riscv64_address,host_stvec_address-rvmstate_riscv64_address);
		// info!("[RVM] host_sstatus is {:#x}",sstatus::read());
		unsafe{trace!("[RVM] guest_sepc is {:#x}",*((&self.rvmstate_riscv64.host_state.zero as *const _ as u64 + 504) as *const u64));}
		trace!("[RVM] test_switch is {:#x}",self.rvmstate_riscv64.guest_state.sepc);

		//todo: check ra value
		let has_err = unsafe { __riscv64_entry(&mut self.rvmstate_riscv64) };

		info!("[RVM] riscv64 exit");

		self.running.store(false, Ordering::SeqCst);

		if has_err {
			warn!(
				// "[RVM] VCPU resume failed: {:?}",
				// VmInstructionError::from(vmcs.read32(VM_INSTRUCTION_ERROR))
				"[RVM] VCPU resume failed"
			);
			return Err(RvmError::Internal);
		}

		// VM Exit
		// match vmexit_handler(
		// 	&mut vmcs,
		// 	&mut self.vmx_state.guest_state,
		// 	&mut self.interrupt_state,
		// 	&self.guest.gpm,
		// 	&self.guest.traps,
		// )? {
		// 	Some(packet) => return Ok(packet), // forward to user mode handler
		// 	None => continue,
		// }
		return Err(RvmError::Success);

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
	fn __test();
}