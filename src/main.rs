use std::num::NonZeroU64;

use monagement::{Monagement, MonagementInit};
use rand::random;

fn main() {
    let allocator = Monagement::init(MonagementInit {
        start: 5,
        maximum: 16384,
    })
    .expect("Init Error");

    unsafe {
        let a = allocator
            .allocate(NonZeroU64::new_unchecked(500))
            .expect("allocate a error");

        let b = allocator
            .allocate(NonZeroU64::new_unchecked(200))
            .expect("allocate b error");

        let c = allocator
            .allocate(NonZeroU64::new_unchecked(1000))
            .expect("allocate c error");

        let d = allocator
            .allocate(NonZeroU64::new_unchecked(800))
            .expect("allocate d error");

        let e = allocator
            .allocate(NonZeroU64::new_unchecked(300))
            .expect("allocate e error");

        let f = allocator
            .allocate(NonZeroU64::new_unchecked(1500))
            .expect("allocate f error");

        let g = allocator
            .allocate(NonZeroU64::new_unchecked(1750))
            .expect("allocate g error");

        let h = allocator
            .allocate(NonZeroU64::new_unchecked(1200))
            .expect("allocate h error");

        let i = allocator
            .allocate(NonZeroU64::new_unchecked(300))
            .expect("allocate i error");

        let j = allocator
            .allocate(NonZeroU64::new_unchecked(3025))
            .expect("allocate j error");

        let k = allocator
            .allocate(NonZeroU64::new_unchecked(100))
            .expect("allocate k error");

        let l = allocator
            .allocate(NonZeroU64::new_unchecked(200))
            .expect("allocate l error");

        let m = allocator
            .allocate(NonZeroU64::new_unchecked(2500))
            .expect("allocate m error");

        let n = allocator
            .allocate(NonZeroU64::new_unchecked(1500))
            .expect("allocate n error");
    }
}
