use crate::monagement::allocated::Allocated;

impl Allocated {
    pub fn free(self) -> Result<(), String> {
        drop(self);

        // self.module
        //     .as_ref()
        //     .ok_or("freeing memory failed because module is not defined")?
        //     .borrow_mut()
        //     .free(&self)?;

        Ok(())
    }
}

// impl Drop for Allocated {
//     fn drop(&mut self) {
//         if let Some(module) = &self.module {
//             if let Ok(_) = module.borrow_mut().free(self) {};
//         }
//     }
// }
