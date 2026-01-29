mod monagement_core;
pub use monagement_core::MonagementInit;
mod test;
use std::{
    cell::{Ref, RefCell},
    num::NonZeroU64,
    rc::Rc,
};

mod tools;
pub use tools::*;

use crate::monagement::{allocated::Allocated, monagement_core::MonagementCore};
mod allocated;
mod level_core;
mod node_core;
pub use node_core::*;

pub struct Monagement {
    core: Rc<RefCell<MonagementCore>>,
}

impl Monagement {
    pub fn init(monagement_init: MonagementInit) -> Result<Self, String> {
        Ok(Self {
            core: Rc::new(RefCell::new(MonagementCore::init(monagement_init)?)),
        })
    }

    pub fn allocate(&self, size: NonZeroU64) -> Result<allocated::Allocated, String> {
        let mut allocated = self.core.borrow_mut().allocate(size)?;
        allocated.module = Some(self.core.clone());

        Ok(allocated)
    }

    pub fn free(&self, allocated: Allocated) -> Result<(), String> {
        self.core.borrow_mut().free(&allocated)?;
        Ok(())
    }

    pub fn borrow_core(&self) -> Ref<'_, MonagementCore> {
        self.core.borrow()
    }

    pub fn borrow_mut_core(&self) -> std::cell::RefMut<'_, MonagementCore> {
        self.core.borrow_mut()
    }

    pub fn get_core(&self) -> &Rc<RefCell<MonagementCore>> {
        &self.core
    }
}
