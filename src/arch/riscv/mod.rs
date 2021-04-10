#![allow(clippy::unnecessary_wraps)]

// 本来希望能在这里判断一下是否硬件上支持Hypervisor
// 就先假设它支持好了
// use raw_cpuid::CpuId;

mod consts;
mod ept;
mod feature;
mod guest;
mod msr;
mod structs;
mod timer;
mod utils;
mod vcpu;
mod vmcall;
mod vmcs;
mod vmexit;

pub use ept::EPageTable as ArchRvmPageTable;
pub use guest::Guest;
pub use vcpu::Vcpu;

//riscv或许有一种类似的方法是也可以判断是否硬件上支持Hypervisor的
//不会判断啦！先假设一定开启过H好了
//TODO:需要找一个包，支持快速判断硬件是否支持H。
pub fn check_hypervisor_feature() -> bool {
    // if let Some(feature) = CpuId::new().get_feature_info() {
    //     feature.has_vmx()
    // } else {
    //     false
    // }
    true
}
