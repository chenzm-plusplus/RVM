pub struct Guest {

}

impl Guest {
    /// Create a new Guest.
    pub fn new(gpm: Arc<dyn GuestPhysMemorySetTrait>) -> RvmResult<Arc<Self>> {

        Ok(Arc::new(Self {
        }))
    }
}