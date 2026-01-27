use crate::monagement::{
    MonagementCore,
    allocated::Allocated,
    get_fl_sl,
    node_core::{NodeStatus, Sl_Idx},
};

impl MonagementCore {
    pub fn free(&mut self, allocated: &Allocated) -> Result<(), &str> {
        let link = allocated.link;
        let mut free_size = 0;
        let mut update_back_link = None;
        let mut update_front_link = None;

        {
            let node = self
                .linked_list
                .get_mut(link)
                .ok_or("Allocated Link Not Found")?
                .as_mut()
                .ok_or("The Link points To An Empty Location")?;
            let node_back = node.back;
            let node_front = node.front;

            // back
            if let Some(back_link_idx) = node_back {
                if let Some(back_link) = self.linked_list.get(back_link_idx) {
                    if let Some(back_node) = back_link {
                        if let NodeStatus::Free(fl, sl, sl_idx) = &back_node.status {
                            free_size += back_node.size;
                            update_back_link = Some(back_node.back);

                            let first_level = self.fl_list.get_mut(*fl as usize).unwrap();
                            first_level.count -= 1;
                            if first_level.count == 0 {
                                self.bitmap &= !(1 << fl);
                            }

                            let second_level = first_level.sl_list.get_mut(*sl as usize).unwrap();
                            second_level.link[sl_idx.0] = None;
                            second_level.free_link_idx.push(sl_idx.0);
                            second_level.count -= 1;
                            if second_level.count == 0 {
                                first_level.bitmap &= !(1 << sl);
                            }

                            self.linked_list[back_link_idx] = None;
                            self.free_linked_list_index.push(back_link_idx);
                        }
                    }
                }
            }

            // front
            if let Some(front_link_idx) = node_front {
                if let Some(front_link) = self.linked_list.get(front_link_idx) {
                    if let Some(front_node) = front_link {
                        if let NodeStatus::Free(fl, sl, sl_idx) = &front_node.status {
                            free_size += front_node.size;
                            update_front_link = Some(front_node.front);

                            let first_level = self.fl_list.get_mut(*fl as usize).unwrap();
                            first_level.count -= 1;
                            if first_level.count == 0 {
                                self.bitmap &= !(1 << fl);
                            }

                            let second_level = first_level.sl_list.get_mut(*sl as usize).unwrap();
                            second_level.link[sl_idx.0] = None;
                            second_level.free_link_idx.push(sl_idx.0);
                            second_level.count -= 1;
                            if second_level.count == 0 {
                                first_level.bitmap &= !(1 << sl);
                            }

                            self.linked_list[front_link_idx] = None;
                            self.free_linked_list_index.push(front_link_idx);
                        }
                    }
                }
            }
        }

        let (fl, sl) = {
            let node = self
                .linked_list
                .get_mut(link)
                .ok_or("Allocated Link Not Found")?
                .as_mut()
                .ok_or("The Link points To An Empty Location")?;
            node.size += free_size;
            let (fl, sl) = get_fl_sl(node.size, self.minimum_size, self.second_level_count);

            if let Some(back) = update_back_link {
                node.back = back;
            }

            if let Some(front) = update_front_link {
                node.front = front;
            }

            (fl, sl)
        };

        if let Some(is_back) = update_back_link {
            if let Some(back) = is_back {
                self.linked_list
                    .get_mut(back)
                    .as_mut()
                    .unwrap()
                    .as_mut()
                    .unwrap()
                    .front = Some(link);
            }
        };

        if let Some(is_front) = update_front_link {
            if let Some(front) = is_front {
                self.linked_list
                    .get_mut(front)
                    .as_mut()
                    .unwrap()
                    .as_mut()
                    .unwrap()
                    .back = Some(link);
            }
        };

        self.bitmap |= 1 << fl;

        let first_level = self.fl_list.get_mut(fl as usize).unwrap();
        first_level.count += 1;
        first_level.bitmap |= 1 << sl;

        let second_level = first_level.sl_list.get_mut(sl as usize).unwrap();
        second_level.count += 1;
        let sl_idx = if let Some(idx) = second_level.free_link_idx.pop() {
            second_level.link[idx] = Some(link);
            idx
        } else {
            let index = second_level.link.len();
            second_level.link.push(Some(link));
            index
        };

        self.linked_list
            .get_mut(link)
            .ok_or("Allocated Link Not Found")?
            .as_mut()
            .ok_or("The Link points To An Empty Location")?
            .status = NodeStatus::Free(fl, sl, Sl_Idx(sl_idx));

        Ok(())
    }
}
