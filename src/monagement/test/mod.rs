use crate::{Monagement, NodeStatus};

#[test]
// 0.0.1
fn testing_allocating() {
    let allocator = Monagement::init(3);
    if let Ok(_) = allocator {
        panic!("in version 0.0.1, the minimum size is 4.")
    }

    let allocator = Monagement::init(256).expect("Init Error");

    let _a = allocator.allocate(50).expect("allocate a error");
    let _b = allocator.allocate(10).expect("allocate b error");
    let _c = allocator.allocate(20).expect("allocate c error");

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
