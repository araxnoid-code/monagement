use std::{num::NonZeroU64, panic};

use monagement::{Monagement, MonagementInit, NodeStatus};
use rand::random;

fn main() {
    let allocator = Monagement::init(MonagementInit {
        start: 2,
        maximum: 32,
    })
    .unwrap();

    println!("{:#?}", allocator.borrow_core());

    let allocated_a = allocator.allocate(NonZeroU64::new(15).unwrap()).unwrap();

    println!("{:#?}", allocator.borrow_core());

    allocator.borrow_mut_core()._free(&allocated_a).unwrap();

    println!("{:#?}", allocator.borrow_core());
}
