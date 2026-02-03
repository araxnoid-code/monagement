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
