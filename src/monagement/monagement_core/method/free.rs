use crate::monagement::{
    MonagementCore,
    allocated::Allocated,
    level_core::SecondLevelLink,
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
            .expect("Free Error | Allocated Link Error. index link points to an invalid index")
            .as_mut()
            .expect("Free Error | Allocated Link Error. index link points to a node with a value of None");
            // allocated data
            allocated_node_index = allocated_node.index;

            // Coalescing
            let allocated_node_back = allocated_node.back;
            let allocated_node_front = allocated_node.front;
            // // front node
            let mut size = allocated_node.size;
            if let Some(front_node_link) = allocated_node_front {
                let front_node = self
                .linked_list
                .get(front_node_link)
                .expect("Free Error, Coalescing Error, index front link points to an invalid index")
                .as_ref()
                .expect("Free Error, Coalescing Error, index front link points to a node with a value of None");

                if let NodeStatus::Free(fl, sl, sl_idx) = &front_node.status {
                    // update handler
                    new_front_link = Some(front_node.front);
                    new_start = Some(front_node.start);

                    // update size
                    let index = front_node.index;
                    size += front_node.size;

                    // clean node
                    self.clean_free_node_in_fl_sl(*fl as usize, *sl as usize, sl_idx.0)
                        .unwrap();
                    self.clean_node_in_linked_list_unchecked(index);
                }
            }

            if let Some(back_node_link) = allocated_node_back {
                let back_node = self
                .linked_list
                .get(back_node_link)
                .expect(
                    "Free Error | Coalescing Error, index back link points to an invalid index in linked list",
                )
                .as_ref()
                .expect("Free Error | Coalescing Error, index back link points to a node with a value of None in linked list");

                if let NodeStatus::Free(fl, sl, sl_idx) = &back_node.status {
                    // update handler
                    new_back_link = Some(back_node.back);
                    new_end = Some(back_node.end);

                    // update size
                    let index = back_node.index;
                    size += back_node.size;

                    // clean node
                    self.clean_free_node_in_fl_sl(*fl as usize, *sl as usize, sl_idx.0)
                        .unwrap();
                    self.clean_node_in_linked_list_unchecked(index);
                }
            }

            // update level
            // // get new fl sl
            let (fl, sl) = self.get_fl_sl(size);

            // // first level
            let first_level = self.fl_list.get_mut(fl as usize).expect(
                "Free Error, Update First Level Error. index points to an invalid index in fl_list",
            );

            first_level.count += 1;
            self.bitmap |= 1 << fl;

            let second_level = first_level.sl_list.get_mut(sl as usize).expect(
                "Free Error, Update Second Level Error. index points to an invalid index in sl_list",
            );

            second_level.count += 1;
            first_level.bitmap |= 1 << sl;

            // let sl_idx = if let Some(idx) = second_level.free_link_idx.pop() {
            //     second_level.link[idx] = Some(allocated_node_index);
            //     idx
            // } else {
            //     let idx = second_level.link.len();
            //     second_level.link.push(Some(allocated_node_index));
            //     idx
            // };
            // update second_layer linked list
            let (sl_idx, push_handler) = if let Some(idx) = second_level.free_link_list.pop() {
                (idx, false)
            } else {
                (second_level.link_list.len(), true)
            };
            if let None = second_level.head_link {
                second_level.head_link = Some(sl_idx);
                second_level.end_link = Some(sl_idx);

                let second_level_link = SecondLevelLink {
                    index: sl_idx,
                    node_link: allocated_node_index,
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
                    .expect("Free Error, Update Second Level Linked List Error. index points to an invalid index in second level link list")
                    .as_mut()
                    .expect("Free Error, Update Second Level Linked List Error. index back link points to a node with a value of None in second level linked list")
                    .front = Some(sl_idx);

                let second_level_link = SecondLevelLink {
                    index: sl_idx,
                    node_link: allocated_node_index,
                    back: Some(end_idx),
                    front: None,
                };

                if push_handler {
                    second_level.link_list.push(Some(second_level_link));
                } else {
                    second_level.link_list[sl_idx] = Some(second_level_link);
                }
            }

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
                    .expect("Free Error | update front node. front link points to an invalid index")
                    .as_mut()
                    .expect(
                        "Free Error | update front node. front link points to a node with a value of None",
                    ).back = Some(allocated_node_index);
            }
        }

        if let Some(back_link) = new_back_link {
            if let Some(idx) = back_link {
                self.linked_list
                    .get_mut(idx)
                    .expect("Free Error | Update Back Node Error. back link points to an invalid index")
                    .as_mut()
                    .expect(
                        "Free Error | Update Back Node Error. back link points to a node with a value of None",
                    ).front = Some(allocated_node_index);
            }
        }

        // // allocated node
        let allocated_node = self
        .linked_list
        .get_mut(link)
        .expect("Free Error | Update Allocated Node Error. index link points to an invalid index")
        .as_mut()
        .expect("Free Error | Update Allocated Node Error. index link points to a node with a value of None");
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
            .expect("Free Node From Linked List Error. index pointes to an invalid index in linked list")
            .as_ref()
            .expect(
                "Free Node From Linked List Error. index points to a node with a value of None in linked list",
            );

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
        let first_level = self.fl_list.get_mut(fl).expect(
            "Clean Free Node Error. first level index points to an invalid index in fl_list",
        );

        let second_level = first_level.sl_list.get_mut(sl).expect(
            "Clean Free Node Error. second level index points to an invalid index in sl_list",
        );

        // update
        // // second level
        // // // link list
        let second_level_link = second_level.link_list
            .get(sl_idx)
            .expect("Clean Free Node Error. index points to an invalid index on the Second Level Link",)
            .as_ref()
            .expect("Clean Free Node Error. index points to an index that has a value of None on the Second Level Link");

        let (head_idx, end_idx) = if let (Some(head_idx), Some(end_idx)) =
            (second_level.head_link, second_level.end_link)
        {
            Some((head_idx, end_idx))
        } else {
            None
        }
        .expect("Clean Free Node Error, Head idx and End idx not found to clean Node");

        if head_idx == sl_idx && end_idx == sl_idx {
            second_level.head_link = None;
            second_level.end_link = None;
        } else if head_idx == sl_idx {
            second_level.head_link = second_level_link.front;
            if let Some(front_link) = second_level_link.front {
                second_level
                    .link_list
                    .get_mut(front_link)
                    .expect("Clean Free Node Error, update front link error. front index points to an index that has a value of None on the Second Level Link")
                    .as_mut()
                    .expect("Clean Free Node Error, update front link error. front index points to an index that has a value of None on the Second Level Link")
                    .back = None;
            }
        } else if end_idx == sl_idx {
            second_level.end_link = second_level_link.back;
            if let Some(back_link) = second_level_link.back {
                second_level
                    .link_list
                    .get_mut(back_link)
                    .expect("Clean Free Node Error, update back link error. back index points to an index that has a value of None on the Second Level Link")
                    .as_mut()
                    .expect("Clean Free Node Error, update back link error. back index points to an index that has a value of None on the Second Level Link")
                    .front = None;
            }
        } else {
            let second_level_link_front = second_level_link.front;
            let second_level_link_back = second_level_link.back;
            if let Some(front_link) = second_level_link_front {
                second_level
                    .link_list
                    .get_mut(front_link)
                    .expect("Clean Free Node Error, update front link error. front index points to an index that has a value of None on the Second Level Link")
                    .as_mut()
                    .expect("Clean Free Node Error, update front link error. front index points to an index that has a value of None on the Second Level Link")
                    .back = second_level_link_back;
            }

            if let Some(back_link) = second_level_link_back {
                second_level
                    .link_list
                    .get_mut(back_link)
                    .expect("Clean Free Node Error, update back link error. back index points to an index that has a value of None on the Second Level Link")
                    .as_mut()
                    .expect("Clean Free Node Error, update back link error. back index points to an index that has a value of None on the Second Level Link")
                    .front = second_level_link_front;
            }
        }

        second_level.link_list[sl_idx] = None;
        second_level.free_link_list.push(sl_idx);
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
