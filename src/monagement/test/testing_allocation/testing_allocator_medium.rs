#[test]
fn testing_allocator_medium() {
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

        let allocator_borrow = allocator.borrow_core();

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
