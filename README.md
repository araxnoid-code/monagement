<div align="center">
    <h1>MONAGEMENT</h1>
    <b><p>TLSF MEMORY ALLOCATOR</p></b>
    <p>⚙️ under development ⚙️</p>
    <b>
        <p>Version / 0.0.1</p>
    </b>
</div>

## About
`Monagement`, is a memory allocator project written in rust that is based on the `TLFS` (Two-Level Segregated Fit) concept.

## Main Architecture
### Two-Level Segregated Fit
uses a 2-level bitmap hierarchy in searching for empty blocks, thus reducing the need for linear scanning.
### Coalescing
any adjacent free blocks will be automatically merged to reduce fragmentation.

## Announcement
- In this version, the division on the second level is 4 and cannot be changed.
- Minimum space for allocator is 4

## Start
### Intallation
```toml
monagement = { git = "" }
```
### Start
```rust
use monagement::Monagement;

fn main() {
    let allocator = Monagement::init(256).expect("Monagement Init Error");

    // allocate memory
    let allocate_a = allocator.allocate(12).expect("Memory Allocation A Error");
    let allocate_b = allocator.allocate(20).expect("Memory Allocation B Error");
    let allocate_c = allocator.allocate(32).expect("Memory Allocation C Error");

    // free up memory
    allocator.free(allocate_a).expect("Freeing Memory A Error");
    // or
    allocate_b.free().expect("Freeing Memory B Error");
    // or
    drop(allocate_c);

    // get data link memory
    println!("{:#?}", allocator.borrow_core().get_linked_list());
}

```
