use monagement::{Monagement, MonagementInit};
use std::num::NonZeroU64;

fn main() {
    let allocator = Monagement::init(MonagementInit::default()).expect("Monagement Init Error");

    // allocate memory
    let allocate_a = allocator
        .allocate(NonZeroU64::new(12).unwrap())
        .expect("Memory Allocation A Error");
    let allocate_b = allocator
        .allocate(NonZeroU64::new(20).unwrap())
        .expect("Memory Allocation B Error");

    // free up memory
    allocate_a.free();
    // or
    drop(allocate_b);

    // get data link memory
    println!("{:#?}", allocator.borrow_core().get_linked_list());
}
