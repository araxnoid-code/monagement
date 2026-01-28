use std::{cell::RefCell, rc::Rc};

use crate::monagement::MonagementCore;

mod method;

#[derive(Debug)]
pub struct Allocated {
    pub(crate) module: Option<Rc<RefCell<MonagementCore>>>,
    pub size: u64,
    pub(crate) link: usize,
}
