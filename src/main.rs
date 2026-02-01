use std::num::NonZeroU64;

use monagement::{Monagement, MonagementInit};

fn main() {
    let allocator = Monagement::init(MonagementInit {
        start: 2,
        maximum: 256,
        selector_opt: monagement::SelectorOpt::SCANNING,
    })
    .expect("Init Error");

    let _a = allocator
        .allocate(NonZeroU64::new(50).unwrap())
        .expect("allocate a error");
    println!("{:#?}", allocator.borrow_core());

    // let _b = allocator
    //     .allocate(NonZeroU64::new(10).unwrap())
    //     .expect("allocate b error");
    // let _c = allocator
    //     .allocate(NonZeroU64::new(20).unwrap())
    //     .expect("allocate c error");
}
