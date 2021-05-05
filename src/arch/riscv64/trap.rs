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
    match scause.cause() {
        Trap::Exception(Exception::VirtHyperEnvCall) => {
            // jump to next instruction anyway
            // let mut cx = guest_state;
            // cx.sepc += 4;
            guest_state.sepc += 4;
            debug!("[RVM] trap_handler: cx.sepc {:#x}",guest_state.sepc);

            // get system call return value
            // let result = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]);
            // let result = syscall5(cx.x[17], [cx.x[10], cx.x[11], cx.x[12], cx.x[13], cx.x[14]]);
            let result = rvm_call(guest_state.a7, guest_state.a0,
                guest_state.a1,guest_state.a2);
            // cx is changed during sys_exec, so we have to call it again
            // cx = guest_state;
            guest_state.a0 = result as u64;
            
        }
        _ => {
            // exit_current_and_run_next(-10);
            // kernel_println!("[kernel] Upsupported trap of app {},core dumped.", get_task_current());
            // exit_current_and_run_next();
            debug!("[RVM] trap_handler...guest_state {:#x?}",guest_state);
            panic!("[RVM] Unsupported trap {:?}, stval = {:#x}, guest_state.sepc = {:#x} !", 
                scause.bits(), stval, guest_state.sepc);
        }
    }
}