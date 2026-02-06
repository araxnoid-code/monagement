## version/0.0.3
- added `allocate_unchecked` method to allocate memory quickly but unsafely, requiring unsafe block.
- added `free_unchecked` method to free memory quickly but unsafely, requires unsafe block.
- To use `free_unchecked`, you must use the method, for example: `a.free_unchecked()`. For `drop(a)`, it will automatically use the `free()` method.
- Now `Allocated` has 2 methods to show the range:
  - `allocated.get_range()`, equal to start..start + size.
  - `allocated.get_node_range()`, displays the original range of nodes, sometimes start..end is not the same as start..start + size, because monagement will take all free nodes if the remaining free nodes are smaller than the start property in MonagementInit, see changelog `version/0.0.2`.

## version/0.0.2
3-Feb-2026
- add MonagementInit in Monagement initialization, MonagementInit has properties including:
  - start, becomes the start of the category and sets the smallest category range. size is based on 2^start. default `2`.
    - example: 
      - `start:2`, then 2^2 then fl_0 will have range 4, 5, 6, 7.
      - `start:3`, then 2^3 then fl_0 will have range 8, 9, 10, 11, 12, 13, 14, 15.
  - maximum, Determines the maximum capacity and the earliest free node (space) in the category. default `1024`.
  - selector_opt, determines how monagement decides between 2 or more free nodes that have the same category. default `SelectorOpt::DIRECT`
    - SelectorOpt::DIRECT, take the earliest node.
    - SelectorOpt::SCANNING, perform a scan to find matching nodes in the same category.
- The total number of categories on the second level is now based on the start property on MonagementInit, for example: `start:3`, then 2^3 = 8, then there will be 8 categories.
- refactoring the `allocate` method and the `free` method.
- To free up memory that has been allocated, now only 2 methods apply:
  - using the free method, `a.free()`
  - using drop, `drop(a)`
- Updated the bitmap method for finding free node coordinates. Initially, a linear search is now performed, but now it jumps directly to the active bit without having to navigate through the dead bits. This change applies to bitmaps that map the first and second levels.
- overcome double free error.
- At the second level, links are used to point to free nodes in a linked list. The link algorithm now uses a linked list instead of a regular array, and is supported by the head_link and end_link properties to directly point to the first and last links.
- added a range property to Allocated, showing the start and end.
  - `ATTENTION!`, the start..end range will sometimes not match start..start + size, because monagement will take the full node size if the remaining memory is smaller than the size of fl_0.
- using NonZeroU64 for the allocate method input.
- add test code, see [testing](./src/monagement/test/mod.rs)

## version/0.0.1
27-jan-2026
- allocate method.
- free method.
- minimum size is 4.
- the division on the second level is 4 and cannot be changed.
