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
    // debug!("in trap_handler......");
    // set_kernel_trap_entry(); //在kernel里面不允许发生其他trap
    let scause = scause::read();
    let stval = stval::read();
    //info!("[RVM] trap: {:#x?}, trap {:?}, guest_state.sepc {:#x}",scause.bits(), scause.cause(),guest_state.sepc);
    match scause.cause() {
        Trap::Exception(Exception::VirtualSupervisorEnvCall) => {
            // jump to next instruction anyway
            // let mut cx = guest_state;
            // cx.sepc += 4;
            guest_state.sepc += 4;
            info!("[RVM] trap: {:?}, guest_state.sepc {:#x}",scause.bits(),guest_state.sepc);

            // get system call return value
            // let result = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]);
            // let result = syscall5(cx.x[17], [cx.x[10], cx.x[11], cx.x[12], cx.x[13], cx.x[14]]);
            let result = rvm_call(guest_state.a7, guest_state.a0,
                guest_state.a1,guest_state.a2);
            // cx is changed during sys_exec, so we have to call it again
            // cx = guest_state;
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
                
                let hst : u64 = csrr!(hstatus);
                let sst : u64 = csrr!(sstatus);
                info!("[RVM] pc is {:#x}, vst i {:#b}",pc,vst);
                info!("[RVM] hst is {:#x}, sst i {:#x}",hst,sst);
                info!("[RVM] guest_hst is {:#b}, guest_sst i {:#b}",guest_state.hstatus,guest_state.sstatus);
                
                //修改csr：
                //vscause,vstval,sepc
                csrw!(vscause, scause.bits() as u64);
                // csrw!(vsepc, guest_state.sepc);
                
                guest_state.sstatus |= 1<<8;  
                //这里修改gueststatus的SPP位是因为先恢复寄存器才执行sret
                //理由和init时设置这一位的理由相同

            }
            // panic!("[RVM] trap: {:?}, guest_state.sepc {:#x}",scause.bits(),guest_state.sepc);
        }
        _ => {
            // exit_current_and_run_next(-10);
            // kernel_println!("[kernel] Upsupported trap of app {},core dumped.", get_task_current());
            // exit_current_and_run_next();
            debug!("[RVM] trap_handler...guest_state {:#x?}",guest_state);
            panic!("[RVM] Unsupported trap {:#x?}, stval = {:#x}, guest_state.sepc = {:#x} !", 
                scause.bits(), stval, guest_state.sepc);
        }
    }
}