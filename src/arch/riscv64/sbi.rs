#![allow(unused)]

const SBI_SET_TIMER: u64 = 0;
const SBI_CONSOLE_PUTCHAR: u64 = 1;
const SBI_CONSOLE_GETCHAR: u64 = 2;
const SBI_CLEAR_IPI: u64 = 3;
const SBI_SEND_IPI: u64 = 4;
const SBI_REMOTE_FENCE_I: u64 = 5;
const SBI_REMOTE_SFENCE_VMA: u64 = 6;
const SBI_REMOTE_SFENCE_VMA_ASID: u64 = 7;
const SBI_SHUTDOWN: u64 = 8;

#[inline(always)]
pub fn sbi_call(which: u64, arg0: u64, arg1: u64, arg2: u64) -> u64 {
    let mut ret;
    //warn!("[RVM] sbi_call, {:#x}, {:#x}, {:#x}, {:#x}",which, arg0, arg1, arg2);
    unsafe {
        llvm_asm!("ecall"
            : "={x10}" (ret)
            : "{x10}" (arg0), "{x11}" (arg1), "{x12}" (arg2), "{x17}" (which)
            : "memory"
            : "volatile"
        );
    }
    ret
}

pub fn set_timer(timer: u64) {
    sbi_call(SBI_SET_TIMER, timer, 0, 0);
}

pub fn console_putchar(c: u64) {
    sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0);
}

/// 从控制台中读取一个字符
///
/// 没有读取到字符则返回 -1
pub fn console_getchar() -> u64 {
    loop {
        let t = sbi_call(SBI_CONSOLE_GETCHAR, 0, 0, 0);
        if t == u64::MAX { continue; }
        return t;
    }
}

pub fn shutdown() -> ! {
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);
    panic!("It should shutdown!");
}

// #[naked]
// #[inline(never)]
// fn ecall(_a0: u64, _a1: u64, _a2: u64, _a3: u64, _a4: u64, _a5: u64, _a6: u64, _a7: u64) {
//     unsafe { asm!("ecall" :: : "a0" : "volatile") }
// }

// pub fn set_timer(stime_value: u64) {
//     ecall(stime_value, 0, 0, 0, 0, 0, 0, 0);
// }

// pub fn clear_ipi() {
//     ecall(0, 0, 0, 0, 0, 0, 0, 3);
// }

// pub fn send_ipi(hart_mask_pointer: u64) {
//     ecall(hart_mask_pointer, 0, 0, 0, 0, 0, 0, 4);
// }

// pub fn shutdown() {
//     ecall(0, 0, 0, 0, 0, 0, 0, 8);
// }

// pub fn send_ipi_to_hart(hart: u64) {
//     let mask: u64 = 1 << hart;
//     send_ipi(&mask as *const u64 as u64);
// }