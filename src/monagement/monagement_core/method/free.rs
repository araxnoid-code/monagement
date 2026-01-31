use crate::monagement::{
    MonagementCore,
    allocated::Allocated,
    node_core::{NodeStatus, SlIdx},
};

impl MonagementCore {
    pub(crate) fn free(&mut self, allocated: &Allocated) -> Result<(), String> {
        let link = allocated.link;
        // handler
        let allocated_node_index;
        let new_size;
        let mut new_front_link = None;
        let mut new_back_link = None;
        let new_status;
        let mut new_start = None;
        let mut new_end = None;
        {
            // allocated node
            let allocated_node = self
            .linked_list
            .get_mut(link)
            .ok_or("Free Error | Allocated Link Error. index link points to an invalid index")?
            .as_mut()
            .ok_or("Free Error | Allocated Link Error. index link points to a node with a value of None")?;
            // allocated data
            allocated_node_index = allocated_node.index;

            // Coalescing
            let allocated_node_back = allocated_node.back;
            let allocated_node_front = allocated_node.front;
            // // front node
            let mut size = allocated.size;
            if let Some(front_node_link) = allocated_node_front {
                let front_node = self
                .linked_list
                .get(front_node_link)
                .ok_or(
                    "Free Error | Coalescing Error, index front link points to an invalid index",
                )?
                .as_ref()
                .ok_or("Free Error | Coalescing Error, index front link points to a node with a value of None")?;

                if let NodeStatus::Free(fl, sl, sl_idx) = &front_node.status {
                    // update handler
                    new_front_link = Some(front_node.front);
                    new_start = Some(front_node.start);

                    // update size
                    let index = front_node.index;
                    size += front_node.size;

                    // clean node
                    self.clean_free_node_in_fl_sl(*fl as usize, *sl as usize, sl_idx.0)?;
                    self.clean_node_in_linked_list_unchecked(index);
                }
            }

            if let Some(back_node_link) = allocated_node_back {
                let back_node = self
                .linked_list
                .get(back_node_link)
                .ok_or(
                    "Free Error | Coalescing Error, index back link points to an invalid index in linked list",
                )?
                .as_ref()
                .ok_or("Free Error | Coalescing Error, index back link points to a node with a value of None in linked list")?;

                if let NodeStatus::Free(fl, sl, sl_idx) = &back_node.status {
                    // update handler
                    new_back_link = Some(back_node.back);
                    new_end = Some(back_node.end);

                    // update size
                    let index = back_node.index;
                    size += back_node.size;

                    // clean node
                    self.clean_free_node_in_fl_sl(*fl as usize, *sl as usize, sl_idx.0)?;
                    self.clean_node_in_linked_list_unchecked(index);
                }
            }

            // update level
            // // get new fl sl
            let (fl, sl) = self.get_fl_sl(size);

            // // first level
            let first_level = self.fl_list.get_mut(fl as usize).ok_or(
                "Free Error | Update First Level. index points to an invalid index in fl_list",
            )?;

            first_level.count += 1;
            self.bitmap |= 1 << fl;

            let second_level = first_level.sl_list.get_mut(sl as usize).ok_or(
                "Free Error | Update Second Level. index points to an invalid index in sl_list",
            )?;

            second_level.count += 1;
            first_level.bitmap |= 1 << sl;

            let sl_idx = if let Some(idx) = second_level.free_link_idx.pop() {
                second_level.link[idx] = Some(allocated_node_index);
                idx
            } else {
                let idx = second_level.link.len();
                second_level.link.push(Some(allocated_node_index));
                idx
            };

            // update handler
            new_size = size;
            new_status = NodeStatus::Free(fl, sl, SlIdx(sl_idx));
        }

        // update node
        // // update front node
        if let Some(front_link) = new_front_link {
            if let Some(idx) = front_link {
                self.linked_list
                    .get_mut(idx)
                    .ok_or("Free Error | update front node. front link points to an invalid index")?
                    .as_mut()
                    .ok_or(
                        "Free Error | update front node. front link points to a node with a value of None",
                    )?.back = Some(allocated_node_index);
            }
        }

        if let Some(back_link) = new_back_link {
            if let Some(idx) = back_link {
                self.linked_list
                    .get_mut(idx)
                    .ok_or("Free Error | Update Back Node Error. back link points to an invalid index")?
                    .as_mut()
                    .ok_or(
                        "Free Error | Update Back Node Error. back link points to a node with a value of None",
                    )?.front = Some(allocated_node_index);
            }
        }

        // // allocated node
        let allocated_node = self
        .linked_list
        .get_mut(link)
        .ok_or("Free Error | Update Allocated Node Error. index link points to an invalid index")?
        .as_mut()
        .ok_or("Free Error | Update Allocated Node Error. index link points to a node with a value of None")?;
        allocated_node.size = new_size;
        allocated_node.status = new_status;
        if let Some(front_link) = new_front_link {
            allocated_node.front = front_link;
        }

        if let Some(back_link) = new_back_link {
            allocated_node.back = back_link;
        }

        if let Some(n_start) = new_start {
            allocated_node.start = n_start;
        }

        if let Some(n_end) = new_end {
            allocated_node.end = n_end;
        }

        Ok(())
    }

    pub fn clean_node_in_linked_list_unchecked(&mut self, idx: usize) {
        // clean
        self.linked_list[idx] = None;
        self.free_linked_list_index.push(idx);
    }

    pub fn clean_node_in_linked_list(&mut self, idx: usize) -> Result<(), String> {
        // check node is exist
        self.linked_list
            .get(idx)
            .ok_or("Free Node From Linked List Error. index pointes to an invalid index in linked list")?
            .as_ref()
            .ok_or(
                "Free Node From Linked List Error. index points to a node with a value of None in linked list",
            )?;

        // clean
        self.linked_list[idx] = None;
        self.free_linked_list_index.push(idx);

        Ok(())
    }

    pub fn clean_free_node_in_fl_sl(
        &mut self,
        fl: usize,
        sl: usize,
        sl_idx: usize,
    ) -> Result<(), String> {
        // get first level and second level
        let first_level = self.fl_list.get_mut(fl).ok_or(
            "Clean Free Node Error. first level index points to an invalid index in fl_list",
        )?;

        let second_level = first_level.sl_list.get_mut(sl).ok_or(
            "Clean Free Node Error. second level index points to an invalid index in sl_list",
        )?;

        // update
        // // second level
        second_level.link[sl_idx] = None;
        second_level.free_link_idx.push(sl_idx);
        second_level.count -= 1;
        if second_level.count == 0 {
            first_level.bitmap &= !(1 << sl);
        }

        // // first level
        first_level.count -= 1;
        if first_level.count == 0 {
            self.bitmap &= !(1 << fl);
        }

        Ok(())
    }
}
