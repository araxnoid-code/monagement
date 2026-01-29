use std::num::{NonZero, NonZeroU64};

use rand::seq::index;

use crate::monagement::{
    allocated::Allocated,
    get_fl_sl,
    monagement_core::MonagementCore,
    node_core::{Node, NodeStatus},
};

impl MonagementCore {
    pub fn allocate(&mut self, size: NonZeroU64) -> Result<Allocated, String> {
        let size = size.get();
        if size > self.max_size {
            let msg = format!(
                "The allocator cannot accommodate data with size {} because the maximum capacity of the allocator is {}",
                size, self.max_size
            );

            return Err(msg);
        }

        let minimum_size = self.minimum_size;
        let second_level_count = self.second_level_count;
        let mut link = None;
        let (fl, sl) = self.get_fl_sl(size);

        let mut sl_start = sl;
        'main: for fl_idx in fl as usize..self.fl_list.len() {
            let first_map = (self.bitmap >> fl_idx) & 1;

            if first_map == 0 {
                sl_start = 0;
                continue;
            }

            let first_level = &mut self.fl_list[fl_idx];
            for sl_idx in sl_start..self.second_level_count {
                let second_map = (first_level.bitmap >> sl_idx) & 1;
                if second_map == 0 {
                    continue;
                }

                let second_level = &mut first_level.sl_list[sl_idx as usize];
                for (i, node_idx) in second_level.link.iter().enumerate() {
                    let linked_list_len = self.linked_list.len();
                    let node_idx = match node_idx {
                        Some(node_idx) => node_idx,
                        None => continue,
                    };

                    if let None = self.linked_list[*node_idx] {
                        continue;
                    }

                    let node_mut = self.linked_list[*node_idx].as_mut().unwrap();
                    let node_mut_idx = node_mut.index;
                    let node_mut_front = node_mut.front;
                    if size > node_mut.size {
                        continue;
                    }

                    let mut rest = node_mut.size - size;
                    if rest < self.minimum_size {
                        rest = 0;
                    }

                    if rest == 0 {
                        // node
                        node_mut.status = NodeStatus::Used;
                        link = Some(node_mut_idx);

                        second_level.link[i] = None;
                        second_level.free_link_idx.push(i);
                        // second_level.link.remove(i);

                        // update sl
                        second_level.count -= 1;
                        if second_level.count == 0 {
                            let update_map = first_level.bitmap & !(1 << sl_idx);
                            first_level.bitmap = update_map;
                        }

                        // update fl
                        first_level.count -= 1;
                        if first_level.count == 0 {
                            let update_map = self.bitmap & !(1 << fl_idx);
                            self.bitmap = update_map;
                        }
                    } else {
                        // crate free node data
                        let free_node_index = node_mut.index;

                        let (idx_used_node, push_handler) =
                            if self.free_linked_list_index.is_empty() {
                                (linked_list_len, true)
                            } else {
                                let index = self.free_linked_list_index.pop().unwrap();
                                (index, false)
                            };
                        link = Some(idx_used_node);

                        // update free node
                        node_mut.size = rest;
                        node_mut.front = Some(idx_used_node);

                        // update front_node
                        if let Some(front_node) = &node_mut_front {
                            let front_node = self.linked_list[*front_node].as_mut().unwrap();
                            front_node.back = Some(idx_used_node);
                        }

                        let used_node = Node {
                            index: idx_used_node,
                            size,
                            status: NodeStatus::Used,
                            back: Some(free_node_index),
                            front: match &node_mut_front {
                                None => None,
                                Some(node_idx) => Some(*node_idx),
                            },
                        };

                        if push_handler {
                            self.linked_list.push(Some(used_node));
                        } else {
                            self.linked_list[idx_used_node] = Some(used_node);
                        }

                        // get new fl sl
                        let (new_fl, new_sl) = get_fl_sl(
                            rest,
                            minimum_size,
                            second_level_count,
                            self.minimum_size_raw,
                        );

                        // update map
                        if fl != new_fl {
                            // masking sl
                            second_level.count -= 1;
                            if second_level.count == 0 {
                                let mask = first_level.bitmap & !(1 << sl_idx);
                                first_level.bitmap = mask;
                            }

                            // masking fl
                            first_level.count -= 1;
                            if first_level.count == 0 {
                                let mask = self.bitmap & !(1 << fl_idx);
                                self.bitmap = mask;
                            }

                            second_level.link[i] = None;
                            second_level.free_link_idx.push(i);
                        } else if sl != new_sl {
                            // masking sl
                            second_level.count -= 1;
                            if second_level.count == 0 {
                                let mask = first_level.bitmap & !(1 << sl_idx);
                                first_level.bitmap = mask;
                            }

                            second_level.link[i] = None;
                            second_level.free_link_idx.push(i);
                        }
                        self.update_counter = Some((new_fl, new_sl, node_mut_idx));
                    }
                    break 'main;
                }
            }
            sl_start = 0;
        }
        if let Some(link) = link {
            self.update_via_fl_sl_counter();
            Ok(Allocated {
                module: None,
                allocated: true,
                size,
                link: link,
            })
        } else {
            Err("error, unable to allocate memory".to_string())
        }
    }

    pub fn _allocate(&mut self, size: NonZeroU64) -> Result<Allocated, String> {
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

            // println!("{:b}", !((1 << sl_target) - 1));
            let mut mask_second_level_map = if searching_from_zero {
                first_level.bitmap & !((1 << sl_target) - 1)
            } else {
                first_level.bitmap
            };

            // println!("{}", searching_from_zero);
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
                )?;
            };

            Ok(Allocated {
                module: None,
                allocated: true,
                size: target,
                link: link,
            })
        } else {
            Err("error, unable to allocate memory".to_string())
        }
    }

    pub fn update_free_node(
        &mut self,
        index: Option<usize>,
        size: Option<u64>,
        back: Option<Option<usize>>,
        front: Option<Option<usize>>,
        fl: u64,
        sl: u64,
        link_idx: usize,
    ) -> Result<(), String> {
        let first_level = self.fl_list.get_mut(fl as usize).ok_or(format!(
            "Error, The first level with index {} does not exist",
            fl
        ))?;

        let second_level = first_level.sl_list.get_mut(sl as usize).ok_or(format!(
            "Error, The second level with index {} in the first level with index {} does not exist",
            sl, fl
        ))?;

        let free_node_idx = second_level.link.get(link_idx).ok_or(format!(
            "Error, link index {} mengarahkan ke pada index yang tidak exist pada first level {}, second level {}", link_idx, fl, sl,
        ))?.ok_or(format!("Error, link index {} mengarahkan ke pada index yang bernilai Nonde pada first level {}, second level {}", link_idx, fl, sl))?;

        let free_node = self
            .linked_list
            .get_mut(free_node_idx)
            .ok_or(format!(
                "Error, link index {} mengarah ke index yang tidak exist pada linked_list",
                link_idx
            ))?
            .as_mut()
            .ok_or(format!(
                "Error, link index {} mengarah ke index yang bernilai None linked_list",
                link_idx
            ))?;

        if let Some(new_idx) = index {
            free_node.index = new_idx;
        }

        if let Some(back_link) = back {
            free_node.back = back_link;
        }

        if let Some(front_link) = front {
            free_node.front = front_link;
        }

        if let Some(new_size) = size {
            // update second_level
            second_level.link[link_idx] = None;
            second_level.free_link_idx.push(link_idx);
            second_level.count -= 1;
            if second_level.count == 0 {
                first_level.bitmap &= !(1 << sl);
            }

            // first level
            first_level.count -= 1;
            if first_level.count == 0 {
                self.bitmap &= !(1 << fl);
            }

            // new location
            let (n_fl, n_sl) = get_fl_sl(new_size, 0, 0, self.minimum_size_raw);
            let first_level = self.fl_list.get_mut(n_fl as usize).ok_or(format!(
                "Error, The first level with index {} does not exist",
                fl
            ))?;

            let second_level = first_level.sl_list.get_mut(n_sl as usize).ok_or(format!(
                "Error, The second level with index {} in the first level with index {} does not exist",
                sl, fl
            ))?;

            // //update level
            first_level.count += 1;
            self.bitmap |= 1 << n_fl;

            second_level.count += 1;
            first_level.bitmap |= 1 << n_sl;

            // // alocation free node
            let sl_idx = if let Some(idx) = second_level.free_link_idx.pop() {
                second_level.link[idx] = Some(free_node.index);
                idx
            } else {
                let idx = second_level.link.len();
                second_level.link.push(Some(free_node.index));
                idx
            };

            free_node.status = NodeStatus::Free(n_fl, n_sl, crate::SlIdx(sl_idx));
            free_node.size = new_size;
        }

        Ok(())
    }
}
