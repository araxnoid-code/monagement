use crate::monagement::{
    allocated::Allocated,
    get_fl_sl,
    monagement_core::MonagementCore,
    node_core::{Node, NodeStatus},
};

impl MonagementCore {
    pub fn allocate(&mut self, size: u64) -> Result<Allocated, String> {
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
            for sl_idx in sl_start..4 {
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
                        let (new_fl, new_sl) = get_fl_sl(rest, minimum_size, second_level_count);

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
                size,
                link: link,
            })
        } else {
            Err("error, unable to allocate memory".to_string())
        }
    }
}
