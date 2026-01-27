mod method;

use std::{cell::RefCell, rc::Rc};

use crate::monagement::{level_core::FirstLevel, node_core::Node};

#[derive(Debug)]
pub struct MonagementCore {
    pub(crate) max_size: u32,
    pub(crate) bitmap: i32,
    pub(crate) minimum_size: u32,
    pub(crate) second_level_count: u32,
    pub(crate) fl_list: Vec<FirstLevel>,
    pub(crate) linked_list: Vec<Option<Node>>,
    pub(crate) free_linked_list_index: Vec<usize>,

    //
    update_counter: Option<(u32, u32, usize)>,
    update_back_link: (Option<usize>, Option<usize>),
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

    pub fn bitmap(&self) -> i32 {
        self.bitmap
    }
}
