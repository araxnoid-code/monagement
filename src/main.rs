use std::{cell::RefCell, rc::Rc, sync::Arc};

fn main() {
    let monagement = Monagement::init(32).unwrap();

    println!("{:#?}", monagement);
}

#[derive(Debug)]
struct Monagement {
    max_size: u32,
    bitmap: i32,
    minimum_size: u32,
    second_level_count: u32,
    fl_list: Vec<FirstLevel>,
    linked_list: Vec<Rc<RefCell<Node>>>,
}

#[derive(Debug)]
struct Node {
    free: bool,
    size: u32,
    back: Option<Rc<RefCell<Node>>>,
    front: Option<Rc<RefCell<Node>>>,
}

#[derive(Clone, Debug)]
struct FirstLevel {
    count: u32,
    size: u32,
    bitmap: i32,
    sl_list: Vec<SecondLevel>,
}

#[derive(Clone, Debug)]
struct SecondLevel {
    count: u32,
    size: u32,
    link: Option<Rc<RefCell<Node>>>,
}

impl Monagement {
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
        };

        let (fl_indexing, sl_indexing) = monagement.get_fl_sl(max_size);
        let second_level = SecondLevel {
            count: 0,
            size: 0,
            link: None,
        };

        let first_level_len = fl_indexing + 1;
        let first_level = FirstLevel {
            count: 0,
            bitmap: 0,
            size: 0,
            sl_list: vec![second_level; 4],
        };

        monagement.bitmap = 1 << fl_indexing;
        monagement.fl_list = vec![first_level; first_level_len as usize];

        // Node
        let node = Node {
            size: max_size,
            free: true,
            back: None,
            front: None,
        };

        let ref_node = Rc::new(RefCell::new(node));

        // first level
        let first_level = &mut monagement.fl_list[fl_indexing as usize];
        first_level.count += 1;
        first_level.size = max_size;
        first_level.bitmap = 1 << sl_indexing;

        // second level
        let second_level = &mut first_level.sl_list[sl_indexing as usize];
        second_level.count += 1;
        second_level.size = max_size;
        second_level.link = Some(ref_node.clone());

        monagement.linked_list.push(ref_node);

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
