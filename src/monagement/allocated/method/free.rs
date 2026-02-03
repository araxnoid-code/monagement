use crate::monagement::allocated::Allocated;

impl Allocated {
    pub fn free(self) {
        drop(self);
    }
}

impl Drop for Allocated {
    fn drop(&mut self) {
        if let Some(module) = &self.module {
            if let Ok(_) = module.borrow_mut().free(self) {};
        }
    }
}
