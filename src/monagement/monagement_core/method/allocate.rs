use std::num::NonZeroU64;

use crate::{
    SelectorOpt, SlIdx, get_fl_sl,
    monagement::{
        allocated::Allocated,
        level_core::SecondLevelLink,
        monagement_core::MonagementCore,
        node_core::{Node, NodeStatus},
    },
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
        // handler
        let mut allocated_link_handler = None;
        // let mut update_free_node_handler = None;
        let mut free_node_idx = None;
        let mut new_front_link = None;
        let mut new_size = None;
        let mut new_start = None;
        let mut allocated_start = None;
        let mut allocated_end = None;

        // searching in first_level
        let mut mask_first_level_map = self.bitmap & !((1 << fl_target) - 1);
        'main: loop {
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

                // update linked list link
                // SELECTOR(SCANNER/DIRECT)
                if let (Some(head_idx), Some(bottom_idx)) =
                    (second_level.head_link, second_level.end_link)
                {
                    let mut second_link_list_idx = head_idx;
                    loop {
                        // get first second level linked list
                        let second_link_list = second_level.link_list
                            .get(second_link_list_idx)
                            .ok_or("Error, Scanning Selector. index points to a non-existent node in the linked_list in second level")?
                            .as_ref()
                            .ok_or("Error, Scanning Selector. index points to an unallocated node address that has status None in second level")?;
                        let link_idx = second_link_list.node_link;

                        // get node
                        let free_node = self.linked_list
                            .get_mut(link_idx)
                            .ok_or("Error, Scanning Selector. index link points to a non-existent node in the linked_list")?
                            .as_mut()
                            .ok_or("Error, Scanning Selector. link index points to an unallocated node address that has status None in linked list")?;
                        free_node_idx = Some(link_idx);

                        if free_node.size < target {
                            if let SelectorOpt::DIRECT = self.selector_option {
                                // DIRECT
                                // go straight to the next category
                                break;
                            } else if let Some(front_idx) = second_link_list.front {
                                // SCANNER
                                // next linked
                                second_link_list_idx = front_idx;
                                continue;
                            } else {
                                // end of linked list
                                break;
                            }
                        }

                        let mut rest = free_node.size - target;
                        if rest < self.minimum_size {
                            rest = 0;
                        }

                        if rest == 0 {
                            // update node
                            free_node.status = NodeStatus::Used;

                            // allocated link
                            allocated_link_handler = Some(free_node.index);

                            // update handler
                            allocated_start = Some(free_node.start);
                            allocated_end = Some(free_node.end);

                            // update level
                            // // second_level
                            // // // linked_list
                            let front_link_list = second_link_list.front;
                            let back_link_list = second_link_list.back;
                            let second_level_link_idx = second_link_list.index;

                            second_level.link_list[second_level_link_idx] = None;
                            second_level.free_link_list.push(second_level_link_idx);

                            if second_level_link_idx == head_idx
                                && second_level_link_idx == bottom_idx
                            {
                                second_level.head_link = None;
                                second_level.end_link = None;
                            } else if second_level_link_idx == head_idx {
                                second_level.head_link = front_link_list;
                                if let Some(front_idx) = front_link_list {
                                    let front_node = second_level
                                        .link_list
                                        .get_mut(front_idx)
                                        .ok_or("Error, Update Head Front Second Level Link List. index link points to a non-existent node in the linked_list")?
                                        .as_mut()
                                        .ok_or("Error, Update Head Front Second Level Link List. link index points to an unallocated node address that has status None")?;
                                    front_node.back = None;
                                }
                            } else if second_level_link_idx == bottom_idx {
                                second_level.end_link = back_link_list;
                                if let Some(back_idx) = back_link_list {
                                    let back_node = second_level
                                        .link_list
                                        .get_mut(back_idx)
                                        .ok_or("Error, Update Bottom Back Second Level Link List. index link points to a non-existent node in the linked_list")?
                                        .as_mut()
                                        .ok_or("Error, Update Bottom Back Second Level Link List. link index points to an unallocated node address that has status None")?;
                                    back_node.front = None;
                                }
                            } else {
                                if let Some(front_idx) = front_link_list {
                                    let front_node = second_level
                                        .link_list
                                        .get_mut(front_idx)
                                        .ok_or("Error, Update Front Second Level Link List. index link points to a non-existent node in the linked_list")?
                                        .as_mut()
                                        .ok_or("Error, Update Front Second Level Link List. link index points to an unallocated node address that has status None")?;
                                    front_node.back = back_link_list;
                                }

                                if let Some(back_idx) = back_link_list {
                                    let back_node = second_level
                                        .link_list
                                        .get_mut(back_idx)
                                        .ok_or("Error, Update Back Second Level Link List. index link points to a non-existent node in the linked_list")?
                                        .as_mut()
                                        .ok_or("Error, Update Back Second Level Link List. link index points to an unallocated node address that has status None")?;
                                    back_node.front = front_link_list;
                                }
                            }

                            // // // bitmap counter
                            second_level.count -= 1;
                            if second_level.count == 0 {
                                first_level.bitmap &= !(1 << sl_idx);
                            }

                            // // first level
                            // // // bitmap counter
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
                            new_front_link = Some(Some(used_node_idx));
                            new_size = Some(rest);
                            new_start = Some(offset + target);
                            allocated_start = Some(offset + target - target);
                            allocated_end = new_start;

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

                            // update level
                            // // second_level
                            // // // linked_list
                            let front_link_list = second_link_list.front;
                            let back_link_list = second_link_list.back;
                            let second_level_link_idx = second_link_list.index;

                            second_level.link_list[second_link_list_idx] = None;
                            second_level.free_link_list.push(second_link_list_idx);

                            if second_level_link_idx == head_idx
                                && second_level_link_idx == bottom_idx
                            {
                                second_level.head_link = None;
                                second_level.end_link = None;
                            } else if second_level_link_idx == head_idx {
                                second_level.head_link = front_link_list;
                                if let Some(front_idx) = front_link_list {
                                    let front_node = second_level
                                        .link_list
                                        .get_mut(front_idx)
                                        .ok_or("Error, Update Head Front Second Level Link List. index link points to a non-existent node in the linked_list")?
                                        .as_mut()
                                        .ok_or("Error, Update Head Front Second Level Link List. link index points to an unallocated node address that has status None")?;
                                    front_node.back = None;
                                }
                            } else if second_level_link_idx == bottom_idx {
                                second_level.end_link = back_link_list;
                                if let Some(back_idx) = back_link_list {
                                    let back_node = second_level
                                        .link_list
                                        .get_mut(back_idx)
                                        .ok_or("Error, Update Bottom Back Second Level Link List. index link points to a non-existent node in the linked_list")?
                                        .as_mut()
                                        .ok_or("Error, Update Bottom Back Second Level Link List. link index points to an unallocated node address that has status None")?;
                                    back_node.front = None;
                                }
                            } else {
                                if let Some(front_idx) = front_link_list {
                                    let front_node = second_level
                                        .link_list
                                        .get_mut(front_idx)
                                        .ok_or("Error, Update Front Second Level Link List. index link points to a non-existent node in the linked_list")?
                                        .as_mut()
                                        .ok_or("Error, Update Front Second Level Link List. link index points to an unallocated node address that has status None")?;
                                    front_node.back = back_link_list;
                                }

                                if let Some(back_idx) = back_link_list {
                                    let back_node = second_level
                                        .link_list
                                        .get_mut(back_idx)
                                        .ok_or("Error, Update Back Second Level Link List. index link points to a non-existent node in the linked_list")?
                                        .as_mut()
                                        .ok_or("Error, Update Back Second Level Link List. link index points to an unallocated node address that has status None")?;
                                    back_node.front = front_link_list;
                                }
                            }
                            // // // bitmap counter
                            second_level.count -= 1;
                            if second_level.count == 0 {
                                first_level.bitmap &= !(1 << sl_idx);
                            }

                            // // first level
                            // // // bitmap counter
                            first_level.count -= 1;
                            if first_level.count == 0 {
                                self.bitmap &= !(1 << fl_idx);
                            }

                            allocated_link_handler = Some(used_node_idx);
                        }
                        break 'main;
                    }
                };

                // masking map to next second level
                mask_second_level_map &= mask_second_level_map - 1;
            }
            // masking map to next first level
            mask_first_level_map &= mask_first_level_map - 1;
        }

        if let Some(link) = allocated_link_handler {
            // update Free Node
            if let (Some(link_idx), Some(n_front_link), Some(n_size), Some(n_start)) =
                (free_node_idx, new_front_link, new_size, new_start)
            {
                let free_node = self.linked_list
                    .get_mut(link_idx)
                    .ok_or("Error, Update Free Node Error. index link points to a non-existent node in the linked_list")?
                    .as_mut()
                    .ok_or("Error, Update Free Node Error. link index points to an unallocated node address that has status None")?;
                let free_node_index = free_node.index;
                free_node.front = n_front_link;
                free_node.size = n_size;
                free_node.start = n_start;

                // new location
                let (fl, sl) = get_fl_sl(n_size, 0, 0, self.minimum_size_raw);
                let first_level = self.fl_list.get_mut(fl as usize).ok_or(format!(
                    "Error, Update Free Node Error. The first level with index {} does not exist",
                    fl
                ))?;

                let second_level = first_level.sl_list.get_mut(sl as usize).ok_or(format!(
                    "Error, Update Free Node Error. The second level with index {} in the first level with index {} does not exist",
                    sl, fl
                ))?;

                first_level.count += 1;
                self.bitmap |= 1 << fl;

                second_level.count += 1;
                first_level.bitmap |= 1 << sl;

                let (sl_idx, push_handler) = if let Some(idx) = second_level.free_link_list.pop() {
                    (idx, false)
                } else {
                    (second_level.link_list.len(), true)
                };
                free_node.status = NodeStatus::Free(fl, sl, SlIdx(sl_idx));

                if let None = second_level.head_link {
                    second_level.head_link = Some(sl_idx);
                    second_level.end_link = Some(sl_idx);
                    let second_level_link = SecondLevelLink {
                        index: sl_idx,
                        node_link: free_node_index,
                        back: None,
                        front: None,
                    };

                    if push_handler {
                        second_level.link_list.push(Some(second_level_link));
                    } else {
                        second_level.link_list[sl_idx] = Some(second_level_link);
                    }
                } else if let Some(end_idx) = second_level.end_link {
                    second_level.end_link = Some(sl_idx);
                    second_level
                        .link_list
                        .get_mut(end_idx)
                        .ok_or("Error, Locating Error. index link points to a non-existent node in the linked_list")?
                        .as_mut()
                        .ok_or("Error, Locating Error. link index points to an unallocated node address that has status None")?
                        .front = Some(sl_idx);

                    let second_level_link = SecondLevelLink {
                        index: sl_idx,
                        node_link: free_node_index,
                        back: Some(end_idx),
                        front: None,
                    };

                    if push_handler {
                        second_level.link_list.push(Some(second_level_link));
                    } else {
                        second_level.link_list[sl_idx] = Some(second_level_link);
                    }
                }
            }

            if let (Some(start), Some(end)) = (allocated_start, allocated_end) {
                Ok(Allocated {
                    module: None,
                    size: target,
                    start,
                    end,
                    link: link,
                })
            } else {
                Err(format!(
                    "Allocation Error, not finding a node that can be allocated for data of size {}",
                    target
                ))
            }
        } else {
            Err(format!(
                "Allocation Error, not finding a node that can be allocated for data of size {}",
                target
            ))
        }
    }
}
