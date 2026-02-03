mod init;
pub use init::MonagementInit;
mod method;

use crate::monagement::{level_core::FirstLevel, node_core::Node};

#[derive(Debug)]
pub enum SelectorOpt {
    DIRECT,
    SCANNING,
}
impl Default for SelectorOpt {
    fn default() -> Self {
        SelectorOpt::DIRECT
    }
}

#[derive(Debug)]
pub struct MonagementCore {
    pub(crate) max_size: u64,
    pub(crate) bitmap: u64,
    pub(crate) minimum_size: u64,
    pub(crate) minimum_size_raw: u64,
    pub(crate) second_level_count: u64,
    pub(crate) fl_list: Vec<FirstLevel>,
    pub(crate) linked_list: Vec<Option<Node>>,
    pub(crate) free_linked_list_index: Vec<usize>,
    pub(crate) selector_option: SelectorOpt,
}

impl MonagementCore {
    pub fn get_linked_list(&self) -> &Vec<Option<Node>> {
        &self.linked_list
    }

    pub fn get_free_linked_list_index(&self) -> &Vec<usize> {
        &self.free_linked_list_index
    }

    pub fn get_fl_list(&self) -> &Vec<FirstLevel> {
        &self.fl_list
    }

    pub fn bitmap(&self) -> u64 {
        self.bitmap
    }

    pub fn get_max_size(&self) -> u64 {
        self.max_size
    }
}
