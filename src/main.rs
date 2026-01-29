use std::num::NonZeroU64;

use monagement::{Monagement, MonagementInit};
use rand::random;

fn main() {
    let allocator = Monagement::init(MonagementInit {
        start: 2,
        maximum: 256,
    })
    .expect("Init Error");

    let _a = allocator
        .allocate(NonZeroU64::new(50).unwrap())
        .expect("allocate a error");
    let _b = allocator
        .allocate(NonZeroU64::new(10).unwrap())
        .expect("allocate b error");
    // let _c = allocator
    //     .allocate(NonZeroU64::new(20).unwrap())
    //     .expect("allocate c error");

    println!("{:#?}", allocator.borrow_core());
}
