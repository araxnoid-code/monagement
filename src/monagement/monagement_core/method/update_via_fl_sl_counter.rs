use crate::monagement::{
    monagement_core::MonagementCore,
    node_core::{NodeStatus, Sl_Idx},
};

impl MonagementCore {
    pub(crate) fn update_via_fl_sl_counter(&mut self) {
        if let Some((fl, sl, free_node_idx)) = self.update_counter {
            let first_level_bitmap = 1 << fl;
            self.bitmap |= first_level_bitmap;

            let first_level = &mut self.fl_list[fl as usize];
            first_level.count += 1;

            let second_level_bitmap = 1 << sl;
            first_level.bitmap |= second_level_bitmap;

            let second_level = &mut first_level.sl_list[sl as usize];
            second_level.count += 1;
            let sl_index = if let Some(idx) = second_level.free_link_idx.pop() {
                second_level.link[idx] = Some(free_node_idx);
                idx
            } else {
                let index = second_level.link.len();
                second_level.link.push(Some(free_node_idx));
                index
            };

            self.linked_list
                .get_mut(free_node_idx)
                .unwrap()
                .as_mut()
                .unwrap()
                .status = NodeStatus::Free(fl, sl, Sl_Idx(sl_index));

            self.update_counter = None;
        }
    }
}
