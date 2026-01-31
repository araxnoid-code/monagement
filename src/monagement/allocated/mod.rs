use std::{cell::RefCell, rc::Rc};

use crate::{Node, monagement::MonagementCore};

mod method;

#[derive(Debug)]
pub struct Allocated {
    pub(crate) module: Option<Rc<RefCell<MonagementCore>>>,
    pub(crate) size: u64,
    pub(crate) start: u64,
    pub(crate) end: u64,
    pub(crate) link: usize,
}

impl Allocated {
    pub fn get_module(&self) -> Result<&Rc<RefCell<MonagementCore>>, String> {
        Ok(self
            .module
            .as_ref()
            .ok_or("Error, module is not initialized")?)
    }

    pub fn get_size(&self) -> u64 {
        self.size
    }

    pub fn get_range(&self) -> (u64, u64) {
        (self.start, self.end)
    }

    pub fn get_link(&self) -> usize {
        self.link
    }
}
