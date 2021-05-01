#![allow(clippy::unnecessary_wraps)]

mod bits;
mod csr;
mod ept;
mod guest;
mod vcpu;
mod config;
#[macro_use]
mod instructions;

pub use ept::EPageTable as ArchRvmPageTable;
pub use guest::Guest;
pub use vcpu::Vcpu;

use riscv::register::{
    misa,
};

//判断是否在硬件上支持Hypervisor
pub fn check_hypervisor_feature() -> bool {
    //访问misa寄存器
    // 这段代码暂时注释掉，因为misa寄存器只有在M态才能访问。
    // 现在真的只能假装它一定好用
    // info!("rvm info test");
    // if let Some(isa) = misa::read(){
    //     info!("rvm has H extension:{}",isa.has_extension('H'));
    // }else{
    //     info!("error");
    // }
    true
}

pub fn test_instructions() -> bool {
    //STORE	ra, (RVMSTATE_HOST_RA)(a0)
    // load!(ra,)
    let a = "3";
    let sp = "sp";
    let ra = "ra";
    info!("{}",load_instruction_and_format!(ra,a,sp));
    true
}