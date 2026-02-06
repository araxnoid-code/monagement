use crate::monagement::allocated::Allocated;

impl Allocated {
    pub fn free(self) {
        drop(self);
    }

    pub unsafe fn free_unchecked(mut self) {
        self.free_unchecked = true;
        drop(self);
    }
}

impl Drop for Allocated {
    fn drop(&mut self) {
        if let Some(module) = &self.module {
            if self.free_unchecked {
                if let Ok(_) = module.borrow_mut().free_unchecked(self) {};
            } else {
                if let Ok(_) = module.borrow_mut().free(self) {};
            }
        }
    }
}
