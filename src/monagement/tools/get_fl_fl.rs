pub fn get_fl_sl(
    size: u64,
    minimum_size: u64,
    second_level_count: u64,
    minimum_size_raw: u64,
) -> (u64, u64) {
    let minimum = minimum_size_raw; // minimum is x from 2^x
    let fl = 63 - size.leading_zeros() as u64;

    let fl_idx = fl.saturating_sub(minimum);
    let shift = if fl >= minimum { fl - minimum } else { 0 };
    let sl_idx = (size ^ (1 << fl)) >> shift;

    (fl_idx, sl_idx)
}
// let size = if size < minimum_size {
//     minimum_size as f32
// } else {
//     size as f32
// };

// let fl = size.log2() as u32;
// let fl_indexing = fl - 2;
// let sl_indexing = (size as i32 - 2i32.pow(fl)) / (2i32.pow(fl) / second_level_count as i32);
// (fl_indexing as u64, sl_indexing as u64)
