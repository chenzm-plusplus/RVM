use super::sbi::*;

const SBI_SET_TIMER: u64 = 0;
const SBI_CONSOLE_PUTCHAR: u64 = 1;
const SBI_CONSOLE_GETCHAR: u64 = 2;
const SBI_CLEAR_IPI: u64 = 3;
const SBI_SEND_IPI: u64 = 4;
const SBI_REMOTE_FENCE_I: u64 = 5;
const SBI_REMOTE_SFENCE_VMA: u64 = 6;
const SBI_REMOTE_SFENCE_VMA_ASID: u64 = 7;
const SBI_SHUTDOWN: u64 = 8;

pub fn rvm_call(which: u64, arg0: u64, arg1: u64, arg2: u64) -> u64{
    match which {
        SBI_SHUTDOWN => {
            return 0;
        },
        _ => sbi_call(which, arg0, arg1, arg2),
    }
}