pub fn get_fl_sl(size: u64, minimum_size: u64, second_level_count: u64) -> (u64, u64) {
    let size = if size < minimum_size {
        minimum_size as f32
    } else {
        size as f32
    };

    let fl = size.log2() as u32;
    let fl_indexing = fl - 2;
    let sl_indexing = (size as i32 - 2i32.pow(fl)) / (2i32.pow(fl) / second_level_count as i32);
    (fl_indexing as u64, sl_indexing as u64)
}
