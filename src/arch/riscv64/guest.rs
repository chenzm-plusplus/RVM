use alloc::sync::Arc;

use crate::memory::{GuestPhysMemorySetTrait,};

use crate::{
    RvmResult,
};

pub struct Guest {

}

//todo: 
impl Guest {
    /// Create a new Guest.
    // pub fn new(gpm: Arc<dyn GuestPhysMemorySetTrait>) -> RvmResult<Arc<Self>> {
    //     Ok(Arc::new(Self {
    //     }))
    // }
    // rust好像不允许函数重载嘎嘎嘎zmb！
    pub fn new() -> RvmResult<Arc<Self>> {
        Ok(Arc::new(Self {
        }))
    }
}