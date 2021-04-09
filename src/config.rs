

//for memory
pub const PAGE_SIZE: usize = 0x1000;//一个page有4096个字节
pub const PAGE_SIZE_BITS: usize = 0xc;//地址的单位是字节，4096个字节需要0xc个bit去表示其地址