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

pub fn enable_timer_interrupt() {
    unsafe { 
        sie::set_stimer(); 
    }
}

pub fn unable_timer_interrupt() {
    unsafe { 
        // sie::clear_stimer(); 
        csrw!(sie,0x0 as u64);
    }
}

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
            //csrrw又回触发illegalinstruction，好像sret让vs返回了u态？
            unsafe{
                let pc : u64 = csrr!(vstvec); //就是手动跳转到guest的中断处理函数入口
                let vst : u64 = csrr!(vsstatus);
                //只修改pc不够
                //8: SPP
                csrw!(vsstatus, (vst | (1 << 8) )& !(1 <<5));
                let hst : u64 = csrr!(hstatus);
                let sst : u64 = csrr!(sstatus);
                info!("[RVM] pc is {:#x}, vst i {:#b}",pc,vst);
                info!("[RVM] hst is {:#x}, sst i {:#x}",hst,sst);
                info!("[RVM] guest_hst is {:#b}, guest_sst i {:#b}",guest_state.hstatus,guest_state.sstatus);
                guest_state.sepc = pc; 
                guest_state.sstatus |= 1<<8; 
                csrw!(vscause, scause.bits() as u64);
            }
            // panic!("[RVM] trap: {:?}, guest_state.sepc {:#x}",scause.bits(),guest_state.sepc);
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            //发现时钟中断：
            info!("[RVM] trap_handler::Interrupt::SupervisorTimer");
            info!("[RVM] trap: {:#x?}, guest_state.sepc {:#x}",scause.bits(),guest_state.sepc);
            //先重新设置一个 10ms 的计时器
            //然后转发给guest处理
            // unsafe{
            //     let pc : u64 = csrr!(vstvec); //手动跳转到guestos的中断处理函数入口
            //     let vst : u64 = csrr!(vsstatus);
            //     //8: SPP
            //     csrw!(vsstatus, (vst | (1 << 8) )& !(1 <<5));
            //     let hst : u64 = csrr!(hstatus);
            //     let sst : u64 = csrr!(sstatus);
            //     // info!("[RVM] pc is {:#x}, vst i {:#b}",pc,vst);
            //     // info!("[RVM] hst is {:#x}, sst i {:#x}",hst,sst);
            //     // info!("[RVM] guest_hst is {:#b}, guest_sst i {:#b}",guest_state.hstatus,guest_state.sstatus);
            //     guest_state.sepc = pc; 
            //     guest_state.sstatus |= 1<<8; 
            //     csrw!(vscause, scause.bits() as u64);
            // }
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