use super::sbi::*;
use super::rvmcall::*;

use riscv::register::{
    mtvec::TrapMode,
    stvec,
    scause::{
        self,
        Trap,
        Exception,
        Interrupt,
    },
    stval,
    sie,
    sepc,
};

use super::vcpu::{
    GuestState,
};

pub fn trap_handler(guest_state: &mut GuestState){
    // set_kernel_trap_entry(); //在kernel里面不允许发生其他trap
    let scause = scause::read();
    let stval = stval::read();
    //info!("[RVM] trap: {:#x?}, trap {:?}, guest_state.sepc {:#x}",scause.bits(), scause.cause(),guest_state.sepc);
    match scause.cause() {
        Trap::Exception(Exception::VirtualSupervisorEnvCall) => {
            // jump to next instruction anyway
            guest_state.sepc += 4;
            info!("[RVM] trap: {:?}, guest_state.sepc {:#x}",scause.bits(),guest_state.sepc);

            // get system call return value
            let result = rvm_call(guest_state.a7, guest_state.a0,
                guest_state.a1,guest_state.a2);
            guest_state.a0 = result as u64;
        }
        Trap::Exception(Exception::IllegalInstruction) |
        Trap::Exception(Exception::StoreFault)
        =>{
            trace!("[RVM] trap: {:?}, guest_state.sepc {:#x}",scause.bits(),guest_state.sepc);

            unsafe{
                let pc : u64 = csrr!(vstvec); //就是手动跳转到guest的中断处理函数入口
                //guest-sepc
                guest_state.sepc = pc; 

                //vsstatus
                let mut vst : u64 = csrr!(vsstatus);
                //8: SPP
                vst &= !(1<<8);
                if (guest_state.sstatus & (1<<8)) > 0 {
                    vst |= 1<<8;
                } 
                //5:SPIE
                vst &= !(1<<5);
                if (vst & (1<<1)) > 0{
                    vst |= (1<<5);
                }
                //SIE
                vst &= !(1<<1);
                csrw!(vsstatus, vst);
                
                //修改csr：
                //vscause,vstval,sepc
                csrw!(vscause, scause.bits() as u64);
                // csrw!(vsepc, guest_state.sepc);
                
                guest_state.sstatus |= 1<<8;  
                //这里修改gueststatus的SPP位是因为先恢复寄存器才执行sret
                //理由和init时设置这一位的理由相同

                //------debug info----
                let hst : u64 = csrr!(hstatus);
                let sst : u64 = csrr!(sstatus);
                trace!("[RVM] pc is {:#x}, vst i {:#b}",pc,vst);
                trace!("[RVM] hst is {:#x}, sst i {:#x}",hst,sst);
                trace!("[RVM] guest_hst is {:#b}, guest_sst i {:#b}",guest_state.hstatus,guest_state.sstatus);
            }
        }
        _ => {
            debug!("[RVM] trap_handler...guest_state {:#x?}",guest_state);
            panic!("[RVM] Unsupported trap {:#x?}, stval = {:#x}, guest_state.sepc = {:#x} !", 
                scause.bits(), stval, guest_state.sepc);
        }
    }
}