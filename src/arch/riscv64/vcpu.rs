use super::{
    Guest,
};

use alloc::{
    sync::Arc,
	boxed::Box,
};

use riscv::register::{
	sstatus::{
	Sstatus,SPP,self,
	},
	scause::{
        self,
        Trap,
        Exception,
        Interrupt,
    },
};

use core::fmt;
use core::pin::Pin;
use core::sync::atomic::{AtomicBool, Ordering};

use super::config::*;
use crate::{packet::RvmExitPacket, RvmError, RvmResult, VcpuIo, VcpuState};

use super::regs::*;
use super::test::*;
use super::csr::*;

use super::trap::trap_handler;

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
	pub scounteren : u64,//35
}
/*
impl fmt::Debug for GuestState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GuestState")
		 .field("zero", &self.zero)//0
		 .field("ra", &self.ra)//1
		 .field("sp", &self.sp)//2
		 .field("gp", &self.gp)//3
		 .field("tp", &self.tp)//4
		 .field("t0", &self.t0)//5
		 .field("t1", &self.t1)//6
		 .field("t2", &self.t2)//7
		 .field("s0", &self.s0)//8
		 .field("s1", &self.s1)//9
		 .field("a0", &self.a0)//10
		 .field("a1", &self.a1)//11
		 .field("a2", &self.a2)//12
		 .field("a3", &self.a3)//13
		 .field("a4", &self.a4)//14
		 .field("a5", &self.a5)//15
		 .field("a6", &self.a6)//16
		 .field("a7", &self.a7)//17
		 .field("s2", &self.s2)//18
		 .field("s3", &self.s3)//19
		 .field("s4", &self.s4)//20
		 .field("s5", &self.s5)//21
		 .field("s6", &self.s6)//22
		 .field("s7", &self.s7)//23
		 .field("s8", &self.s8)//24
		 .field("s9", &self.s9)//25
		 .field("s10", &self.s10)//26
		 .field("s11", &self.s11)//27
		 .field("t3", &self.t3)//28
		 .field("t4", &self.t4)//29
		 .field("t5", &self.t5)//30
		 .field("t6", &self.t6)//31
		 .field("sepc", &self.sepc)//32
		 .field("sstatus", &self.sstatus)//33
		 .field("hstatus", &self.hstatus)//34
		 .field("scounteren",&self.scounteren)//35
         .finish()
    }
}
*/
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
		__test_write_general_registers();
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

	fn tracer(&self){
		let rvmstate_riscv64_address = &self.rvmstate_riscv64.host_state.zero as *const _ as u64;
		trace!("[RVM] a0 is {:#x}",&self.rvmstate_riscv64.host_state.a0 as *const _ as u64);
		trace!("[RVM] guest_sepc address is {:#x}",&self.rvmstate_riscv64.host_state.zero as *const _ as u64 + 504);
		let guest_sepc_address = &self.rvmstate_riscv64.guest_state.sepc as *const _ as u64;
		let guest_sstatus_address = &self.rvmstate_riscv64.guest_state.sstatus as *const _ as u64;
		let host_scounteren_address = &self.rvmstate_riscv64.host_state.scounteren as *const _ as u64;
		let guest_scounteren_address = &self.rvmstate_riscv64.guest_state.scounteren as *const _ as u64;
		let host_stvec_address = &self.rvmstate_riscv64.host_state.stvec as *const _ as u64;
		trace!("[RVM] rvmstate_riscv64 address is {:#x}",rvmstate_riscv64_address);
		trace!("[RVM] host_stvec address is {:#x}",host_stvec_address);
		trace!("[RVM] guest_sstatus address is {:#x}",guest_sstatus_address);
		trace!("[RVM] host_stvec offset is {}, {:#x}",host_stvec_address-rvmstate_riscv64_address,host_stvec_address-rvmstate_riscv64_address);
		trace!("[RVM] host_scounteren offset is {}, {:#x}",host_scounteren_address-rvmstate_riscv64_address,host_scounteren_address-rvmstate_riscv64_address);
		trace!("[RVM] guest_scounteren offset is {}, {:#x}",guest_scounteren_address-rvmstate_riscv64_address,guest_scounteren_address-rvmstate_riscv64_address);
		// info!("[RVM] host_sstatus is {:#x}",sstatus::read());
		unsafe{trace!("[RVM] guest_sepc is {:#x}",*((&self.rvmstate_riscv64.host_state.zero as *const _ as u64 + 504) as *const u64));}
		trace!("[RVM] test_switch is {:#x}",self.rvmstate_riscv64.guest_state.sepc);
	}

	fn init(&mut self, entry: u64) -> RvmResult {
		//8:userexception委托给VS
		//2:illegal instruction

		let hedeleg_value = 0x1ff | 1<<12|1<<13|1<<15 ;//  0xffff_ffff;//a1<<8 | 1<<2 | 1<<7;
		debug!("[RVM] hedeleg_value {:#x}",hedeleg_value);
		unsafe{ csrw!(hedeleg, hedeleg_value);}
		self.rvmstate_riscv64.guest_state.sepc = 0x0000_0000_9000_0000 as u64;
		self.rvmstate_riscv64.guest_state.sstatus = 0x8000_0000_0000_6100 as u64; 
					//需要设置hstatus
			//SPV = 1 : 表示在h态之前V=1，因此执行sret可以进入这个态
			//SPVP = 1 : V=1时这一位有效，表示S（1）U（0）
		self.rvmstate_riscv64.guest_state.hstatus = (1 << 7) | (1<<8) | (0<<21) as u64;
		self.rvmstate_riscv64.guest_state.hstatus |= (0 << 22) as u64;
        Ok(())
    }

	pub fn resume(&mut self) -> RvmResult<RvmExitPacket> {
		loop{
			info!("vcpu::resume");

			// VM Entry
			self.running.store(true, Ordering::SeqCst);
			
			// self.rvmstate_riscv64.guest_state.sepc = test_switch as u64;

			//这里需要设置SEIP=1，表示在trap之前处于S态。否则在entry的最后一行执行sret就会跳到U态，权限就不对了QAQ

			trace!("[RVM] [entry] check host state...{:#x?}",self.rvmstate_riscv64.host_state);
			debug!("[RVM] [entry] check guest state...{:#x?}",self.rvmstate_riscv64.guest_state);

			info!("[RVM] riscv64 entry");

			self.tracer();

			let has_err = unsafe { __riscv64_entry(&mut self.rvmstate_riscv64) };

			info!("[RVM] riscv64 exit");

			let s = scause::read();
			
			info!("[RVM] scause is {:#x}",s.bits() as u64);

			trace!("[RVM] [exit] check host state...{:#x?}",self.rvmstate_riscv64.host_state);
			debug!("[RVM] [exit] check guest state...{:#x?}",self.rvmstate_riscv64.guest_state);

			self.running.store(false, Ordering::SeqCst);

			if self.rvmstate_riscv64.guest_state.a7 == 8 {
				return Err(RvmError::Success);
			}

			trap_handler(&mut self.rvmstate_riscv64.guest_state);

			info!("[RVM] after trap_handler...guest sepc is {:#x}",self.rvmstate_riscv64.guest_state.sepc);

		}
		return Err(RvmError::Success);
    }
}

global_asm!(include_str!("entry.S"));
global_asm!(include_str!("entrytest.S"));

extern "C" {
    //todo怎么把这里改成地址
    fn __riscv64_entry(_rvm_state: &mut RvmStateRiscv64) -> bool;
    /// This is effectively the second-half of vmx_entry. When we return from a
    /// VM exit, vmx_state argument is stored in RSP. We use this to restore the
    /// stack and registers to the state they were in when vmx_entry was called.
    fn __riscv64_exit() -> bool;
	fn __test();
	fn __test_write_general_registers();
}