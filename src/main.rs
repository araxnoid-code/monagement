use std::{cell::RefCell, rc::Rc, sync::Arc};

fn main() {
    let mut monagement = Monagement::init(31).unwrap();

    // println!("{:#?}", monagement.linked_list);
    println!("==================");
    // println!("fl map {:b}", monagement.bitmap);

    monagement.allocate(10);

    // println!("{:#?}", monagement.linked_list);
    println!("==================");

    monagement.allocate(5);

    // println!("{:#?}", monagement.linked_list);
    println!("==================");

    monagement.allocate(13);

    println!("{:#?}", monagement.linked_list);
    println!("==================");

    // println!("{:#?}", monagement.linked_list);
    // println!("fl map  {:b}", monagement.bitmap);
}

#[derive(Debug)]
struct Monagement {
    max_size: u32,
    bitmap: i32,
    minimum_size: u32,
    second_level_count: u32,
    fl_list: Vec<FirstLevel>,
    linked_list: Vec<Option<Node>>,
    free_linked_list_index: Vec<usize>,

    //
    update_counter: Option<(u32, u32, usize)>,
}

impl Monagement {
    pub fn allocate(&mut self, size: u32) {
        let minimum_size = self.minimum_size;
        let second_level_count = self.second_level_count;
        let (fl, sl) = self.get_fl_sl(size);

        let mut sl_start = sl;
        'main: for fl_idx in fl as usize..self.fl_list.len() {
            let first_map = (self.bitmap >> fl_idx) & 1;

            if first_map == 0 {
                sl_start = 0;
                continue;
            }

            let first_level = &mut self.fl_list[fl_idx];
            for sl_idx in sl_start..4 {
                let second_map = (first_level.bitmap >> sl_idx) & 1;
                if second_map == 0 {
                    continue;
                }

                let second_level = &mut first_level.sl_list[sl_idx as usize];
                for (i, node_idx) in second_level.link.iter().enumerate() {
                    let linked_list_len = self.linked_list.len();
                    let node_mut = self.linked_list[*node_idx].as_mut().unwrap();
                    let node_mut_idx = node_mut.index;
                    let node_mut_front = node_mut.front;
                    if size > node_mut.size {
                        continue;
                    }

                    let mut rest = node_mut.size - size;
                    if rest < self.minimum_size {
                        rest = 0;
                    }

                    if rest == 0 {
                        // node
                        node_mut.status = NodeStatus::Used;
                        second_level.link.remove(i);

                        // update sl
                        second_level.count -= 1;
                        if second_level.count == 0 {
                            let update_map = first_level.bitmap & !(1 << sl_idx);
                            first_level.bitmap = update_map;
                        }

                        // update fl
                        first_level.count -= 1;
                        if first_level.count == 0 {
                            let update_map = self.bitmap & !(1 << fl_idx);
                            self.bitmap = update_map;
                        }
                    } else {
                        // crate free node data
                        let free_node_index = node_mut.index;

                        let idx_used_node = if self.free_linked_list_index.is_empty() {
                            linked_list_len
                        } else {
                            let index = self.free_linked_list_index.pop().unwrap();
                            index
                        };

                        // update free node
                        node_mut.size = rest;
                        node_mut.front = Some(idx_used_node);

                        // update front_node
                        if let Some(front_node) = &node_mut_front {
                            let front_node = self.linked_list[*front_node].as_mut().unwrap();
                            front_node.back = Some(idx_used_node);
                        }

                        let used_node = Node {
                            index: idx_used_node,
                            size,
                            status: NodeStatus::Used,
                            back: Some(free_node_index),
                            front: match &node_mut_front {
                                None => None,
                                Some(node_idx) => Some(*node_idx),
                            },
                        };

                        if self.free_linked_list_index.is_empty() {
                            self.linked_list.push(Some(used_node));
                        } else {
                            self.linked_list[idx_used_node] = Some(used_node);
                        }

                        // get new fl sl
                        let (new_fl, new_sl) = get_fl_sl(rest, minimum_size, second_level_count);

                        // update map
                        if fl != new_fl {
                            // masking sl
                            second_level.count -= 1;
                            if second_level.count == 0 {
                                let mask = first_level.bitmap & !(1 << sl_idx);
                                first_level.bitmap = mask;
                            }

                            // masking fl
                            first_level.count -= 1;
                            if first_level.count == 0 {
                                let mask = self.bitmap & !(1 << fl_idx);
                                self.bitmap = mask;
                            }

                            second_level.link.remove(i);
                        } else if sl != new_sl {
                            // masking sl
                            second_level.count -= 1;
                            if second_level.count == 0 {
                                let mask = first_level.bitmap & !(1 << sl_idx);
                                first_level.bitmap = mask;
                            }

                            second_level.link.remove(i);
                        }
                        self.update_counter = Some((new_fl, new_sl, node_mut_idx));
                    }
                    break 'main;
                }
            }
            sl_start = 0;
        }
        self.update_via_fl_sl_counter();
    }

    pub fn update_via_fl_sl_counter(&mut self) {
        if let Some((fl, sl, free_node_idx)) = self.update_counter {
            let first_level_bitmap = 1 << fl;
            self.bitmap |= first_level_bitmap;

            let first_level = &mut self.fl_list[fl as usize];
            first_level.count += 1;

            let second_level_bitmap = 1 << sl;
            first_level.bitmap |= second_level_bitmap;

            let second_level = &mut first_level.sl_list[sl as usize];
            second_level.count += 1;
            second_level.link.push(free_node_idx);

            self.update_counter = None;
        }
    }

    pub fn init(max_size: u32) -> Result<Self, &'static str> {
        if max_size < 4 {
            return Err("Minimum Size Is 4");
        }

        let mut monagement = Self {
            max_size,
            bitmap: 0,
            minimum_size: 4,
            second_level_count: 4,
            fl_list: vec![],
            linked_list: Vec::with_capacity(1),
            free_linked_list_index: vec![],

            //
            update_counter: None,
        };

        let (fl_indexing, sl_indexing) = monagement.get_fl_sl(max_size);
        let second_level = SecondLevel {
            count: 0,
            // size: 0,
            link: vec![],
        };

        let first_level_len = fl_indexing + 1;
        let first_level = FirstLevel {
            count: 0,
            bitmap: 0,
            // size: 0,
            sl_list: vec![second_level; 4],
        };

        monagement.bitmap = 1 << fl_indexing;
        monagement.fl_list = vec![first_level; first_level_len as usize];

        // Node
        let node = Node {
            index: 0,
            size: max_size,
            status: NodeStatus::Free,
            back: None,
            front: None,
        };

        // let ref_node = Rc::new(RefCell::new(node));

        // first level
        let first_level = &mut monagement.fl_list[fl_indexing as usize];
        first_level.count += 1;
        // first_level.size = max_size;
        first_level.bitmap = 1 << sl_indexing;

        // second level
        let second_level = &mut first_level.sl_list[sl_indexing as usize];
        second_level.count += 1;
        // second_level.size = max_size;
        second_level.link = vec![0];

        monagement.linked_list.push(Some(node));

        Ok(monagement)
    }

    fn get_fl_sl(&self, size: u32) -> (u32, u32) {
        let size = if (size as u32) < self.minimum_size {
            self.minimum_size as f32
        } else {
            size as f32
        };

        let fl = size.log2() as u32;
        let fl_indexing = fl - 2;
        let sl_indexing =
            (size as i32 - 2i32.pow(fl)) / (2i32.pow(fl) / self.second_level_count as i32);
        (fl_indexing, sl_indexing as u32)
    }
}

fn get_fl_sl(size: u32, minimum_size: u32, second_level_count: u32) -> (u32, u32) {
    let size = if (size as u32) < minimum_size {
        minimum_size as f32
    } else {
        size as f32
    };

    let fl = size.log2() as u32;
    let fl_indexing = fl - 2;
    let sl_indexing = (size as i32 - 2i32.pow(fl)) / (2i32.pow(fl) / second_level_count as i32);
    (fl_indexing, sl_indexing as u32)
}

#[derive(Debug)]
enum NodeStatus {
    Free,
    Used,
}

#[derive(Debug)]
struct Node {
    index: usize,
    status: NodeStatus,
    size: u32,
    back: Option<usize>,
    front: Option<usize>,
}

#[derive(Clone, Debug)]
struct FirstLevel {
    count: u32,
    // size: u32,
    bitmap: i32,
    sl_list: Vec<SecondLevel>,
}

#[derive(Clone, Debug)]
struct SecondLevel {
    count: u32,
    // size: u32,
    link: Vec<usize>,
}
