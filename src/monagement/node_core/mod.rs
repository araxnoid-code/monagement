#[derive(Debug)]
pub(crate) struct Sl_Idx(pub usize);

#[derive(Debug)]
pub(crate) enum NodeStatus {
    Free(u32, u32, Sl_Idx),
    Used,
}

#[derive(Debug)]
pub struct Node {
    pub(crate) index: usize,
    pub(crate) status: NodeStatus,
    pub(crate) size: u32,
    pub(crate) back: Option<usize>,
    pub(crate) front: Option<usize>,
}
