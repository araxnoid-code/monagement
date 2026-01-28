use crate::monagement::{
    level_core::{FirstLevel, SecondLevel},
    monagement_core::MonagementCore,
    node_core::{Node, NodeStatus, SlIdx},
};

// pub struct MonagementInit {
//     minimum: u
// }

impl MonagementCore {
    pub fn init(max_size: u64) -> Result<Self, &'static str> {
        if max_size < 4 {
            return Err("Minimum Size Is 4");
        }

        let mut monagement = Self {
            max_size,
            bitmap: 0,
            minimum_size: 4,
            second_level_count: 4,
            fl_list: vec![],
            linked_list: Vec::with_capacity(1),
            free_linked_list_index: vec![],

            //
            update_counter: None,
            update_back_link: (None, None),
        };

        let (fl_indexing, sl_indexing) = monagement.get_fl_sl(max_size);
        let second_level = SecondLevel {
            count: 0,
            link: vec![],
            free_link_idx: vec![],
        };

        let first_level_len = fl_indexing + 1;
        let first_level = FirstLevel {
            count: 0,
            bitmap: 0,
            sl_list: vec![second_level; 4],
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
        };

        // first level
        let first_level = &mut monagement.fl_list[fl_indexing as usize];
        first_level.count += 1;
        // first_level.size = max_size;
        first_level.bitmap = 1 << sl_indexing;

        // second level
        let second_level = &mut first_level.sl_list[sl_indexing as usize];
        second_level.count += 1;
        // second_level.size = max_size;
        second_level.link = vec![Some(0)];

        monagement.linked_list.push(Some(node));

        Ok(monagement)
    }
}
