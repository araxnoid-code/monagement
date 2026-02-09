<div align="center">
    <h1>MONAGEMENT</h1>
    <b><p>TLSF MEMORY ALLOCATOR</p></b>
    <p>⚙️ under development ⚙️</p>
    <b>
        <p>Version / 0.0.3</p>
    </b>
</div>

## About
`Monagement`, is a memory allocator project written in rust that is based on the `TLSF` (Two-Level Segregated Fit) concept.

## Main Architecture
### Two-Level Segregated Fit
uses a 2-level bitmap hierarchy in searching for empty blocks, thus reducing the need for linear scanning.
### Bitmap
use of bitmaps for fast search
### Coalescing
any adjacent free blocks will be automatically merged to reduce fragmentation.

## What's New?
see what's new in version 0.0.3: [version/0.0.3](./version.md)

## Changelog
[changelog.md](./changelog.md)

## Start
### Installation
Run the following Cargo command in your project directory:
```toml
cargo add monagement
```
Or add the following line to your Cargo.toml:
```toml
monagement = "0.0.3"
```

### Code
```rust
use monagement::{Monagement, MonagementInit};
use std::num::NonZeroU64;

fn main() {
    let allocator = Monagement::init(MonagementInit::default()).expect("Monagement Init Error");

    // allocate memory
    let allocate_a = allocator
        .allocate(NonZeroU64::new(12).unwrap())
        .expect("Memory Allocation A Error");
    let allocate_b = allocator
        .allocate(NonZeroU64::new(20).unwrap())
        .expect("Memory Allocation B Error");

    // free up memory
    allocate_a.free();
    // or
    drop(allocate_b);

    // get data link memory
    println!("{:#?}", allocator.borrow_core().get_linked_list());
}
```

### Monagement initialization
```rust
use monagement::{Monagement, SelectorOpt};

fn main() {
    let allocator = Monagement::init(monagement::MonagementInit {
        start: 3, // 2^3
        maximum: 2048,
        selector_opt: SelectorOpt::DIRECT, 
        // SelectorOpt::DIRECT, only takes the first free node from a category (default), 
        //                      if the free node does not match it will go straight to the next category.
        // SelectorOpt::SCANNING, perform scanning on a category if there are different 
        //                        free nodes that have the same category
    });
}
```
