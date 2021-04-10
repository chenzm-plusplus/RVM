
/// Whether need to inject timer interrupt to guest when an interrupt occurs on host
#[cfg(target_arch = "x86_64")]
pub fn is_host_timer_interrupt(vector: u8) -> bool {
    unsafe { rvm_is_host_timer_interrupt(vector) }
}

/// Whether need to inject serial interrupt to guest when an interrupt occurs on host
#[cfg(target_arch = "x86_64")]
pub fn is_host_serial_interrupt(vector: u8) -> bool {
    unsafe { rvm_is_host_serial_interrupt(vector) }
}

extern "Rust" {
    #[cfg(target_arch = "x86_64")]
    fn rvm_is_host_timer_interrupt(vector: u8) -> bool;
    #[cfg(target_arch = "x86_64")]
    fn rvm_is_host_serial_interrupt(vector: u8) -> bool;
}