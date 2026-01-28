#[derive(Debug)]
pub struct SlIdx(pub usize);

#[derive(Debug)]
pub enum NodeStatus {
    Free(u32, u32, SlIdx),
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

impl Node {
    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_status(&self) -> &NodeStatus {
        &self.status
    }

    pub fn get_size(&self) -> u32 {
        self.size
    }

    pub fn get_back_link_id(&self) -> Option<usize> {
        self.back
    }

    pub fn get_front_link_id(&self) -> Option<usize> {
        self.front
    }
}
