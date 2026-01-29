#[derive(Clone, Debug)]
pub(crate) struct FirstLevel {
    pub(crate) count: u64,
    pub(crate) bitmap: i32,
    pub(crate) sl_list: Vec<SecondLevel>,
}

#[derive(Clone, Debug)]
pub(crate) struct SecondLevel {
    pub(crate) count: u64,
    pub(crate) link: Vec<Option<usize>>,
    pub(crate) free_link_idx: Vec<usize>,
}
