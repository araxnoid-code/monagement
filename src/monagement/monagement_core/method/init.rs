use crate::monagement::{
    level_core::{FirstLevel, SecondLevel},
    monagement_core::{MonagementCore, init::MonagementInit},
    node_core::{Node, NodeStatus, SlIdx},
};

impl MonagementCore {
    pub fn init(monagement_init: MonagementInit) -> Result<Self, String> {
        let max_size = monagement_init.get_maximum();
        let start = monagement_init.get_minimum();
        let start_raw = monagement_init.get_raw_minimum();

        if max_size < start {
            let msg = format!("Error, Allocator Minimum Size Is {}", start);
            return Err(msg);
        }

        let mut monagement = Self {
            max_size,
            bitmap: 0,
            minimum_size: start,
            minimum_size_raw: start_raw,
            second_level_count: start,
            fl_list: vec![],
            linked_list: Vec::with_capacity(1),
            free_linked_list_index: vec![],
            selector_option: monagement_init.selector_opt,
        };

        let (fl_indexing, sl_indexing) = monagement.get_fl_sl(max_size);
        let second_level = SecondLevel {
            count: 0,
            link: vec![],
            free_link_idx: vec![],
            direct_node: None,
            bitmap: 0,
        };

        let first_level_len = fl_indexing + 1;
        let first_level = FirstLevel {
            count: 0,
            bitmap: 0,
            sl_list: vec![second_level; start as usize],
        };

        monagement.bitmap = 1 << fl_indexing;
        monagement.fl_list = vec![first_level; first_level_len as usize];

        // Node
        let node = Node {
            index: 0,
            size: max_size,
            status: NodeStatus::Free(fl_indexing, sl_indexing, SlIdx(0)),
            back: None,
            front: None,
            start: 0,
            end: max_size,
        };

        // first level
        let first_level = &mut monagement.fl_list[fl_indexing as usize];
        first_level.count += 1;
        first_level.bitmap = 1 << sl_indexing;

        // second level
        let second_level = &mut first_level.sl_list[sl_indexing as usize];
        second_level.count += 1;
        second_level.direct_node = Some((0, max_size));
        second_level.bitmap = 1;
        second_level.link = vec![Some(0)];

        monagement.linked_list.push(Some(node));

        Ok(monagement)
    }
}
