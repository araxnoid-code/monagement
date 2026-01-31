use std::num::NonZeroU64;

use crate::monagement::{
    allocated::Allocated,
    monagement_core::MonagementCore,
    node_core::{Node, NodeStatus},
};

impl MonagementCore {
    pub fn allocate(&mut self, size: NonZeroU64) -> Result<Allocated, String> {
        let target = size.get();
        let (fl_target, sl_target) = self.get_fl_sl(target);
        if target > self.max_size {
            let msg = format!(
                "The allocator cannot accommodate data with size {} because the maximum capacity of the allocator is {}",
                size, self.max_size
            );

            return Err(msg);
        }
        let mut allocated_link_handler = None;
        let mut update_free_node_handler = None;

        // searching in first_level
        let mut mask_first_level_map = self.bitmap & !((1 << fl_target) - 1);
        loop {
            // calculate idx of active bit
            let fl_idx = mask_first_level_map.trailing_zeros() as u64;
            if fl_idx == 64 {
                // reach the limit
                break;
            }
            let searching_from_zero = fl_target == fl_idx;

            let first_level = self.fl_list.get_mut(fl_idx as usize).ok_or(format!(
                "Error, The first level with index {} does not exist",
                fl_idx
            ))?;

            let mut mask_second_level_map = if searching_from_zero {
                first_level.bitmap & !((1 << sl_target) - 1)
            } else {
                first_level.bitmap
            };

            loop {
                let sl_idx = mask_second_level_map.trailing_zeros() as u64;
                if sl_idx == 64 {
                    // reach the limit
                    break;
                }

                let second_level = first_level.sl_list.get_mut(sl_idx as usize).ok_or(format!(
                    "Error, The second level with index {} in the first level with index {} does not exist",
                    sl_idx, fl_idx
                ))?;

                for (link_i, free_node_idx) in second_level.link.iter().enumerate() {
                    let free_node = if let Some(idx) = free_node_idx {
                        self.linked_list.get_mut(*idx).ok_or(format!(
                            "Error, index link points to a non-existent node in the linked_list"
                        ))?.as_mut().ok_or("Error, link index points to an unallocated node address that has status None")?
                    } else {
                        continue;
                    };

                    if free_node.size >= target {
                        let mut rest = free_node.size - target;
                        if rest < self.minimum_size {
                            rest = 0;
                        }

                        if rest == 0 {
                            // update node
                            free_node.status = NodeStatus::Used;
                            // allocated link
                            allocated_link_handler = Some(free_node.index);

                            // update level
                            // // second_level
                            second_level.link[link_i] = None;
                            second_level.free_link_idx.push(link_i);
                            second_level.count -= 1;
                            if second_level.count == 0 {
                                first_level.bitmap &= !(1 << sl_idx);
                            }

                            // // first level
                            first_level.count -= 1;
                            if first_level.count == 0 {
                                self.bitmap &= !(1 << fl_idx);
                            }
                        } else {
                            // get metadata for new used node
                            let back_node_link = Some(free_node.index);
                            let front_node_link = free_node.front;
                            let offset = free_node.start;
                            let (used_node_idx, push_handler) =
                                if let Some(idx) = self.free_linked_list_index.pop() {
                                    (idx, true)
                                } else {
                                    (self.linked_list.len(), false)
                                };

                            // used node
                            let used_node = Node {
                                index: used_node_idx,
                                size: target,
                                status: NodeStatus::Used,
                                back: back_node_link,
                                front: front_node_link,
                                start: offset,
                                end: offset + target,
                            };

                            // enter node into linked list
                            if !push_handler {
                                self.linked_list.push(Some(used_node));
                            } else {
                                self.linked_list[used_node_idx] = Some(used_node);
                            }

                            // update front_node
                            if let Some(front_node_idx) = front_node_link {
                                if let Some(front_node) = self.linked_list.get_mut(front_node_idx) {
                                    let front_node = front_node
                                        .as_mut()
                                        .ok_or(format!("Error, index link front points to None"))?;
                                    front_node.back = Some(used_node_idx);
                                };
                            }

                            // update_handler
                            update_free_node_handler = Some((
                                None,
                                Some(rest),
                                None::<Option<usize>>,
                                Some(Some(used_node_idx)),
                                fl_idx,
                                sl_idx,
                                link_i,
                                offset + target,
                            ));

                            allocated_link_handler = Some(used_node_idx);
                        }
                        break;
                    }
                }

                // masking map to next second level
                mask_second_level_map &= mask_second_level_map - 1;
            }
            // masking map to next first level
            mask_first_level_map &= mask_first_level_map - 1;
        }

        if let Some(link) = allocated_link_handler {
            if let Some(handler) = update_free_node_handler {
                self.update_free_node(
                    handler.0, handler.1, handler.2, handler.3, handler.4, handler.5, handler.6,
                    handler.7,
                )?;

                Ok(Allocated {
                    module: None,
                    size: target,
                    start: handler.7 - target,
                    end: handler.7,
                    link: link,
                })
            } else {
                Err(format!("Allocation Error, Update Free Node Error"))
            }
        } else {
            Err(format!(
                "Allocation Error, not finding a node that can be allocated for data of size {}",
                size
            ))
        }
    }
}
