use std::{num::NonZeroU64, time::SystemTime};

use rand::random;

use crate::{Monagement, MonagementInit, NodeStatus, SelectorOpt};

#[test]
fn testing_allocating_low() {
    let allocator = Monagement::init(MonagementInit {
        start: 2,
        maximum: 256,
        selector_opt: crate::monagement::monagement_core::SelectorOpt::SCANNING,
    })
    .expect("Init Error");

    let _a = allocator
        .allocate(NonZeroU64::new(50).unwrap())
        .expect("allocate a error");
    let _b = allocator
        .allocate(NonZeroU64::new(10).unwrap())
        .expect("allocate b error");
    let _c = allocator
        .allocate(NonZeroU64::new(20).unwrap())
        .expect("allocate c error");

    let allocator_core = allocator.borrow_core();
    // block 0
    let block = allocator_core
        .get_linked_list()
        .get(0)
        .expect("block with index 0 not found")
        .as_ref()
        .expect("the block accessed is invalid, in this case it is a free block");

    assert_eq!(block.get_index(), 0);
    if let NodeStatus::Free(fl, sl, sl_idx) = block.get_status() {
        assert_eq!(*fl, 5);
        assert_eq!(*sl, 1);
        assert_eq!(sl_idx.0, 0);
    }
    assert_eq!(block.get_size(), 176);
    if let Some(_) = block.get_back_link_id() {
        panic!("error, back must be None")
    }

    if let Some(id) = block.get_front_link_id() {
        assert_eq!(id, 3);
    } else {
        panic!("error, back must be 3")
    }
    // block 0
    //
    // block 1
    let block = allocator_core
        .get_linked_list()
        .get(1)
        .expect("block with index 1 not found")
        .as_ref()
        .expect("the block accessed is invalid, in this case it is a free block");

    assert_eq!(block.get_index(), 1);
    if let NodeStatus::Free(_, _, _) = block.get_status() {
        panic!("error, Status Must Be Used")
    }
    assert_eq!(block.get_size(), 50);
    if let Some(id) = block.get_back_link_id() {
        assert_eq!(id, 2);
    }

    if let Some(_) = block.get_front_link_id() {
        panic!("error, back must be None")
    }
    // block 1
    //
    // block 2
    let block = allocator_core
        .get_linked_list()
        .get(2)
        .expect("block with index 2 not found")
        .as_ref()
        .expect("the block accessed is invalid, in this case it is a free block");

    assert_eq!(block.get_index(), 2);
    if let NodeStatus::Free(_, _, _) = block.get_status() {
        panic!("error, Status Must Be Used")
    }
    assert_eq!(block.get_size(), 10);
    if let Some(id) = block.get_back_link_id() {
        assert_eq!(id, 3);
    }

    if let Some(id) = block.get_front_link_id() {
        assert_eq!(id, 1);
    }
    // block 2
    //
    // block 3
    let block = allocator_core
        .get_linked_list()
        .get(3)
        .expect("block with index 3 not found")
        .as_ref()
        .expect("the block accessed is invalid, in this case it is a free block");

    assert_eq!(block.get_index(), 3);
    if let NodeStatus::Free(_, _, _) = block.get_status() {
        panic!("error, Status Must Be Used")
    }
    assert_eq!(block.get_size(), 20);
    if let Some(id) = block.get_back_link_id() {
        assert_eq!(id, 0);
    }

    if let Some(id) = block.get_front_link_id() {
        assert_eq!(id, 2);
    }
    // block 3
}

#[test]
fn testing_allocating_medium() {
    let allocator = Monagement::init(MonagementInit {
        start: 5,
        maximum: 16384,
        selector_opt: crate::monagement::monagement_core::SelectorOpt::SCANNING,
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

        let allocator_borrow = allocator.borrow_core();
        let check = allocator_borrow.get_linked_list().iter().find(|node| {
            let node = node.as_ref().unwrap();
            let range = node.get_range();
            let distance = range.1 - range.0;

            !distance == node.get_size()
        });
        if let Some(_) = check {
            panic!("range and size should be related")
        }

        let linked_list = allocator_borrow.get_linked_list();
        let free_node = linked_list
            .get(0)
            .as_ref()
            .expect("error in accessing index 0")
            .as_ref()
            .expect(
                "error in accessing index 0, because it has a value of None in the linked_list",
            );
        assert_eq!(free_node.get_index(), 0);
        if let NodeStatus::Free(fl, sl, sl_idx) = free_node.get_status() {
            assert_eq!(*fl, 5);
            assert_eq!(*sl, 15);
            assert_eq!(sl_idx.0, 0);
        }
        assert_eq!(free_node.get_size(), 1509);
        if let Some(_) = free_node.get_back_link_id() {
            panic!("error, free node should have back link value 'None'")
        }
        let front_link = free_node
            .get_front_link_id()
            .expect("error, free node should have front link value pointing to node index 14");
        assert_eq!(front_link, 14);

        let node_idx_10 = linked_list
            .get(10)
            .as_ref()
            .expect("error in accessing index 10")
            .as_ref()
            .expect(
                "error in accessing index 0, because it has a value of None in the linked_list",
            );
        assert_eq!(node_idx_10.get_index(), 10);
        if let NodeStatus::Free(_, _, _) = node_idx_10.get_status() {
            panic!("error, node with index 10 should have status Used");
        }
        assert_eq!(node_idx_10.get_size(), 3025,);
        let back_link = node_idx_10.get_back_link_id().expect(
            "error, node with idx 10 should have back link value pointing to node index 10",
        );
        assert_eq!(back_link, 11);

        let front_link = node_idx_10.get_front_link_id().expect(
            "error, node with idx 10 should have front link value pointing to node index 9",
        );
        assert_eq!(front_link, 9);

        let node_idx_14 = linked_list
            .get(13)
            .as_ref()
            .expect("error in accessing index 14")
            .as_ref()
            .expect(
                "error in accessing index 0, because it has a value of None in the linked_list",
            );
        assert_eq!(node_idx_14.get_index(), 13);
        if let NodeStatus::Free(_, _, _) = node_idx_14.get_status() {
            panic!("error, node with index 13 should have status Used");
        }
        assert_eq!(node_idx_14.get_size(), 2500);
        let back_link = node_idx_14.get_back_link_id().expect(
            "error, node with idx 14 should have back link value pointing to node index 10",
        );
        assert_eq!(back_link, 14);

        let front_link = node_idx_14.get_front_link_id().expect(
            "error, node with idx 12 should have front link value pointing to node index 9",
        );
        assert_eq!(front_link, 12);

        // total
        let sum = allocator
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
            .sum::<u64>();
        assert_eq!(sum, allocator.borrow_core().get_max_size());
    }
}

#[test]
fn allocating_free_stress() {
    let maximum = 16777216;
    let monagement = Monagement::init(MonagementInit {
        start: 3,
        maximum,
        selector_opt: SelectorOpt::SCANNING,
    })
    .unwrap();

    for i in 0..1000 {
        let size = random::<u16>() as u64;
        let a = if size > 0 {
            let drop_stat = rand::random_bool(0.5);
            let a = monagement.allocate(NonZeroU64::new(size).unwrap()).unwrap();
            if drop_stat {
                a.free();
                None
            } else {
                Some(a)
            }
        } else {
            None
        };

        if let Some(allocated) = &a {
            let allocator = monagement.borrow_core();
            let node = allocator
                .get_linked_list()
                .get(allocated.get_link())
                .expect("Error, index points to an invalid index")
                .as_ref()
                .expect("Error, Node is None");

            let range = node.get_range();
            if range.1 - range.0 != size {
                panic!("Error, size and range are not related")
            }
        }

        let size = random::<u16>() as u64;
        let a = if size > 0 {
            let drop_stat = rand::random_bool(0.5);
            let a = monagement.allocate(NonZeroU64::new(size).unwrap()).unwrap();
            if drop_stat {
                a.free();
                None
            } else {
                Some(a)
            }
        } else {
            None
        };

        if let Some(allocated) = &a {
            let allocator = monagement.borrow_core();
            let node = allocator
                .get_linked_list()
                .get(allocated.get_link())
                .expect("Error, index points to an invalid index")
                .as_ref()
                .expect("Error, Node is None");

            let range = node.get_range();
            if range.1 - range.0 != size {
                panic!("Error, size and range are not related")
            }
        }

        let size = random::<u16>() as u64;
        let a = if size > 0 {
            let drop_stat = rand::random_bool(0.5);
            let a = monagement.allocate(NonZeroU64::new(size).unwrap()).unwrap();
            if drop_stat {
                a.free();
                None
            } else {
                Some(a)
            }
        } else {
            None
        };

        if let Some(allocated) = &a {
            let allocator = monagement.borrow_core();
            let node = allocator
                .get_linked_list()
                .get(allocated.get_link())
                .expect("Error, index points to an invalid index")
                .as_ref()
                .expect("Error, Node is None");

            let range = node.get_range();
            if range.1 - range.0 != size {
                panic!("Error, size and range are not related")
            }
        }

        let size = random::<u16>() as u64;
        let a = if size > 0 {
            let drop_stat = rand::random_bool(0.5);
            let a = monagement.allocate(NonZeroU64::new(size).unwrap()).unwrap();
            if drop_stat {
                a.free();
                None
            } else {
                Some(a)
            }
        } else {
            None
        };

        if let Some(allocated) = &a {
            let allocator = monagement.borrow_core();
            let node = allocator
                .get_linked_list()
                .get(allocated.get_link())
                .expect("Error, index points to an invalid index")
                .as_ref()
                .expect("Error, Node is None");

            let range = node.get_range();
            if range.1 - range.0 != size {
                panic!("Error, size and range are not related")
            }
        }

        let size = random::<u16>() as u64;
        let a = if size > 0 {
            let drop_stat = rand::random_bool(0.5);
            let a = monagement.allocate(NonZeroU64::new(size).unwrap()).unwrap();
            if drop_stat {
                a.free();
                None
            } else {
                Some(a)
            }
        } else {
            None
        };

        if let Some(allocated) = &a {
            let allocator = monagement.borrow_core();
            let node = allocator
                .get_linked_list()
                .get(allocated.get_link())
                .expect("Error, index points to an invalid index")
                .as_ref()
                .expect("Error, Node is None");

            let range = node.get_range();
            if range.1 - range.0 != size {
                panic!("Error, size and range are not related")
            }
        }
    }

    let size = monagement
        .borrow_core()
        .get_linked_list()
        .iter()
        .find(|node| if let Some(_) = node { true } else { false })
        .expect("free node not found")
        .as_ref()
        .unwrap()
        .get_size();

    assert_eq!(maximum, size);
}

#[test]
fn second_level_linked_list_small_testing() {
    let allocator = Monagement::init(MonagementInit {
        start: 2,
        maximum: 1024,
        selector_opt: SelectorOpt::SCANNING,
    })
    .expect("Init Error");

    let _a = allocator
        .allocate(NonZeroU64::new(28).unwrap())
        .expect("allocate a error");

    let _b = allocator
        .allocate(NonZeroU64::new(31).unwrap())
        .expect("allocate b error");

    let _c = allocator
        .allocate(NonZeroU64::new(31).unwrap())
        .expect("allocate c error");

    let _d = allocator
        .allocate(NonZeroU64::new(31).unwrap())
        .expect("allocate d error");

    let _e = allocator
        .allocate(NonZeroU64::new(29).unwrap())
        .expect("allocate e error");

    let _f = allocator
        .allocate(NonZeroU64::new(31).unwrap())
        .expect("allocate f error");

    let _g = allocator
        .allocate(NonZeroU64::new(31).unwrap())
        .expect("allocate g error");

    drop(_a);
    drop(_c);
    drop(_e);

    {
        let core = allocator.borrow_core();
        let first_level = core.get_fl_list().get(2).expect("msg");
        let second_level = first_level.sl_list.get(3).expect("msg");
        assert_eq!(3, second_level.count);

        let head_link = second_level
            .head_link
            .expect("head idx points to an invalid index");
        assert_eq!(head_link, 0);

        let end_link = second_level
            .end_link
            .expect("end idx points to an invalid index");
        assert_eq!(end_link, 2);

        let link = second_level
            .link_list
            .get(0)
            .expect("leads to an invalid index")
            .as_ref()
            .expect("leads to a link that has the value None");
        assert_eq!(0, link.index);
        assert_eq!(1, link.node_link);
        let front_link = link.front.expect("front link not initialized");
        assert_eq!(1, front_link);
        if let Some(_) = link.back {
            panic!("this is the first link, there should be no back links")
        }

        let link = second_level
            .link_list
            .get(1)
            .expect("leads to an invalid index")
            .as_ref()
            .expect("leads to a link that has the value None");
        assert_eq!(1, link.index);
        assert_eq!(3, link.node_link);
        let front_link = link.front.expect("front link not initialized");
        assert_eq!(2, front_link);
        let back_link = link.back.expect("back link not initialized");
        assert_eq!(0, back_link);

        let link = second_level
            .link_list
            .get(2)
            .expect("leads to an invalid index")
            .as_ref()
            .expect("leads to a link that has the value None");
        assert_eq!(2, link.index);
        assert_eq!(5, link.node_link);
        if let Some(_) = link.front {
            panic!("this is the last link, there should be no more front links")
        }
        let back_link = link.back.expect("back link not initialized");
        assert_eq!(1, back_link);
    }

    let _h = allocator
        .allocate(NonZeroU64::new(30).unwrap())
        .expect("allocate h error");
    let _i = allocator
        .allocate(NonZeroU64::new(29).unwrap())
        .expect("allocate i error");
}

#[test]
fn test_peforma() {
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

    println!("\ntested monagement with an allocation of 500,000 times and immediately cleared");
    println!("total\t: {} ms", tock - tick);
    println!("mean\t: {} ms", (tock as f64 - tick as f64) / count as f64)
}
