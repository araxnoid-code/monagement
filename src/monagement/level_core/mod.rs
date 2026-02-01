use crate::Node;

#[derive(Clone, Debug)]
pub struct FirstLevel {
    pub(crate) count: u64,
    pub(crate) bitmap: u64,
    pub(crate) sl_list: Vec<SecondLevel>,
}

pub struct SecondLayerLink {
    index: usize,
    node: Node,
    front: Option<usize>,
    back: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct SecondLevel {
    pub(crate) count: u64,
    pub(crate) link: Vec<Option<usize>>,
    pub(crate) free_link_idx: Vec<usize>,
    pub(crate) direct_node: Option<(usize, u64)>,
}
