use std::num::NonZeroU64;

use monagement::{Monagement, MonagementInit};
use rand::random;

fn main() {
    let allocator = Monagement::init(MonagementInit {
        start: 3,
        maximum: 100,
    })
    .unwrap();

    // unsafe {
    //     allocator
    //         .borrow_core()
    //         ._allocate(NonZeroU64::new_unchecked(32))
    //         .unwrap();
    // };

    let mut map: u64 = 0b01100100101;
    loop {
        let idx = map.trailing_zeros();
        if idx == 64 {
            break;
        }
        println!("{}", idx);
        map &= map - 1;
    }
}
