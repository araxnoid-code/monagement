use std::{num::NonZeroU64, time::SystemTime};

use monagement::{Allocated, Monagement, MonagementInit, SelectorOpt};
use rand::{random, random_bool, random_range};

fn main() {
    let maximum = 16777216;
    let allocator = Monagement::init(MonagementInit {
        start: 5,
        maximum,
        selector_opt: SelectorOpt::SCANNING,
    })
    .unwrap();

    let count = 500000;
    let tick = std::time::SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    for _ in 0..count {
        let allocated = allocator.allocate(NonZeroU64::new(1000).unwrap()).unwrap();
        allocated.free();
    }

    let tock = std::time::SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    println!("{}", tock - tick);
    println!("mean :{}", (tock as f64 - tick as f64) / count as f64)
}
