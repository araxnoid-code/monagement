use crate::Node;

#[derive(Clone, Debug)]
pub struct FirstLevel {
    pub(crate) count: u64,
    pub(crate) bitmap: u64,
    pub(crate) sl_list: Vec<SecondLevel>,
}

#[derive(Clone, Debug)]
pub struct SecondLevelLink {
    pub(crate) index: usize,
    pub(crate) node_link: usize,
    pub(crate) front: Option<usize>,
    pub(crate) back: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct SecondLevel {
    pub(crate) count: u64,
    pub(crate) link: Vec<Option<usize>>,
    pub(crate) free_link_idx: Vec<usize>,
    // update
    pub(crate) head_link_list: Option<usize>,
    pub(crate) end_link_list: Option<usize>,
    pub(crate) link_list: Vec<Option<SecondLevelLink>>,
    pub(crate) free_link_list: Vec<usize>,
}
