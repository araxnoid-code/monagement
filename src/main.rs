use std::{num::NonZeroU64, time::SystemTime};

use monagement::{Allocated, Monagement, MonagementInit, SelectorOpt};
use rand::{random, random_bool, random_range};

fn main() {
    let maximum = 16777216;
    let allocator = Monagement::init(MonagementInit {
        start: 5,
        maximum,
        selector_opt: SelectorOpt::DIRECT,
    })
    .unwrap();

    let mut save = vec![];

    let tick = std::time::SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    for _ in 0..10000 {
        for _ in 0..random_range(0..50) {
            let size = random::<u16>() as u64;
            if size <= 0 {
                continue;
            }
            save.push(allocator.allocate(NonZeroU64::new(size).unwrap()));
        }

        let mut len = save.len();
        for _ in 0..len / 2 {
            let random = random_range(0..len);
            let allocated = save.swap_remove(random);
            if let Ok(allocated) = allocated {
                allocated.free();
            }

            len -= 1;
        }

        for _ in 0..random_range(0..50) {
            let size = random::<u16>() as u64;
            if size <= 0 {
                continue;
            }
            save.push(allocator.allocate(NonZeroU64::new(size).unwrap()));
        }

        let mut len = save.len();
        for _ in 0..len / 2 {
            let random = random_range(0..len);
            let allocated = save.swap_remove(random);
            if let Ok(allocated) = allocated {
                allocated.free();
            }

            len -= 1;
        }
    }

    let tock = std::time::SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    println!("{}", tock - tick);
}
