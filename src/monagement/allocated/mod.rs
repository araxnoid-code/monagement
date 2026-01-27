use std::{cell::RefCell, rc::Rc};

use crate::monagement::MonagementCore;

mod method;

pub struct Allocated {
    pub(crate) module: Option<Rc<RefCell<MonagementCore>>>,
    pub size: u32,
    pub(crate) link: usize,
}
