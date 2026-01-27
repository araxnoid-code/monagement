pub fn get_fl_sl(size: u32, minimum_size: u32, second_level_count: u32) -> (u32, u32) {
    let size = if (size as u32) < minimum_size {
        minimum_size as f32
    } else {
        size as f32
    };

    let fl = size.log2() as u32;
    let fl_indexing = fl - 2;
    let sl_indexing = (size as i32 - 2i32.pow(fl)) / (2i32.pow(fl) / second_level_count as i32);
    (fl_indexing, sl_indexing as u32)
}
