use std::num::NonZeroU64;

use monagement::{Monagement, MonagementInit, NodeStatus};
use rand::random;

fn main() {
    let allocator = Monagement::init(MonagementInit {
        start: 5,
        maximum: 16384,
    })
    .expect("Init Error");

    unsafe {
        allocator.allocate(NonZeroU64::new_unchecked(500)).unwrap();
        allocator.allocate(NonZeroU64::new_unchecked(200)).unwrap();
        allocator.allocate(NonZeroU64::new_unchecked(1000)).unwrap();
        allocator.allocate(NonZeroU64::new_unchecked(800)).unwrap();
        allocator.allocate(NonZeroU64::new_unchecked(300)).unwrap();
        allocator.allocate(NonZeroU64::new_unchecked(1500)).unwrap();
        allocator.allocate(NonZeroU64::new_unchecked(1750)).unwrap();
        allocator.allocate(NonZeroU64::new_unchecked(1200)).unwrap();
        allocator.allocate(NonZeroU64::new_unchecked(300)).unwrap();
        allocator.allocate(NonZeroU64::new_unchecked(3025)).unwrap();
        allocator.allocate(NonZeroU64::new_unchecked(100)).unwrap();
        allocator.allocate(NonZeroU64::new_unchecked(200)).unwrap();
        allocator.allocate(NonZeroU64::new_unchecked(2500)).unwrap();
        allocator.allocate(NonZeroU64::new_unchecked(1500)).unwrap();

        println!("{:#?}", allocator.borrow_core());
        println!(
            "{:#?}",
            allocator
                .borrow_core()
                .get_linked_list()
                .iter()
                .map(|node| {
                    if let Some(node) = node {
                        node.get_size()
                    } else {
                        0
                    }
                })
                .sum::<u64>()
        );
    }
}
