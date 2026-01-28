use crate::monagement::monagement_core::MonagementCore;

impl MonagementCore {
    pub(crate) fn get_fl_sl(&self, size: u64) -> (u64, u64) {
        let size = if (size as u64) < self.minimum_size {
            self.minimum_size as f32
        } else {
            size as f32
        };

        let fl = size.log2() as u32;
        let fl_indexing = fl - 2;
        let sl_indexing =
            (size as i32 - 2i32.pow(fl)) / (2i32.pow(fl) / self.second_level_count as i32);
        (fl_indexing as u64, sl_indexing as u64)
    }
}
