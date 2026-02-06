## version/0.0.3
3-Feb-2026
- added `allocate_unchecked` method to allocate memory quickly but unsafely, requiring unsafe block.
- added `free_unchecked` method to free memory quickly but unsafely, requires unsafe block.
- To use `free_unchecked`, you must use the method, for example: `a.free_unchecked()`. For `drop(a)`, it will automatically use the `free()` method.
- Now `Allocated` has 2 methods to show the range:
  - `allocated.get_range()`, equal to start..start + size.
  - `allocated.get_node_range()`, displays the original range of nodes, sometimes start..end is not the same as start..start + size, because monagement will take all free nodes if the remaining free nodes are smaller than the start property in MonagementInit, see changelog `version/0.0.2`.
