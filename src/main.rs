use monagement::Monagement;

fn main() {
    let allocator = Monagement::init(3).expect("Monagement Init Error");

    // mengalokasikan memory
    let allocate_a = allocator.allocate(12).expect("Memory Allocation A Error");
    let allocate_b = allocator.allocate(20).expect("Memory Allocation B Error");
    let allocate_c = allocator.allocate(32).expect("Memory Allocation C Error");

    // membebaskan memory
    allocator.free(allocate_a).expect("Freeing Memory A Error");
    // or
    allocate_b.free().expect("Freeing Memory B Error");
    // or
    drop(allocate_c);

    // mendapatkan data link memory
    println!("{:#?}", allocator.borrow_core().get_linked_list());
}
