#![allow(clippy::unnecessary_wraps)]

mod ept;
mod guest;
mod vcpu;


pub use ept::EPageTable as ArchRvmPageTable;
pub use guest::Guest;
pub use vcpu::Vcpu;

use riscv::register::{
    misa,
};

//判断是否在硬件上支持Hypervisor
pub fn check_hypervisor_feature() -> bool {
    //访问misa寄存器
    misa.has_extension(h)
}