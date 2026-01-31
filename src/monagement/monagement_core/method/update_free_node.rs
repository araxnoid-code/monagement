use crate::{NodeStatus, get_fl_sl, monagement::monagement_core::MonagementCore};

impl MonagementCore {
    pub(crate) fn update_free_node(
        &mut self,
        index: Option<usize>,
        size: Option<u64>,
        back: Option<Option<usize>>,
        front: Option<Option<usize>>,
        fl: u64,
        sl: u64,
        link_idx: usize,
        start: u64,
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

        free_node.start = start;
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
