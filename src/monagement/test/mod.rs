mod testing_allocation;

use std::num::NonZeroU64;

use rand::random;

use crate::{Monagement, NodeStatus, monagement::monagement_core::MonagementInit};

// fn testing() {
//     let monagement = Monagement::init(MonagementInit {
//         start: 2,
//         maximum: 262144,
//     })
//     .unwrap();

//     for i in 0..100000 {
//         let size = random::<u16>() as u64;
//         let a = if size > 0 {
//             let drop_stat = rand::random_bool(0.5);
//             let a = monagement.allocate(NonZeroU64::new(size).unwrap()).unwrap();
//             if drop_stat {
//                 a.free().unwrap();
//                 None
//             } else {
//                 Some(a)
//             }
//         } else {
//             None
//         };

//         let size = random::<u16>() as u64;
//         let a = if size > 0 {
//             let drop_stat = rand::random_bool(0.5);
//             let a = monagement.allocate(NonZeroU64::new(size).unwrap()).unwrap();
//             if drop_stat {
//                 a.free().unwrap();
//                 None
//             } else {
//                 Some(a)
//             }
//         } else {
//             None
//         };

//         let size = random::<u16>() as u64;
//         let a = if size > 0 {
//             let drop_stat = rand::random_bool(0.5);
//             let a = monagement.allocate(NonZeroU64::new(size).unwrap()).unwrap();
//             if drop_stat {
//                 a.free().unwrap();
//                 None
//             } else {
//                 Some(a)
//             }
//         } else {
//             None
//         };

//         let size = random::<u16>() as u64;
//         let a = if size > 0 {
//             let drop_stat = rand::random_bool(0.5);
//             let a = monagement.allocate(NonZeroU64::new(size).unwrap()).unwrap();
//             if drop_stat {
//                 a.free().unwrap();
//                 None
//             } else {
//                 Some(a)
//             }
//         } else {
//             None
//         };

//         let size = random::<u16>() as u64;
//         let a = if size > 0 {
//             let drop_stat = rand::random_bool(0.5);
//             let a = monagement.allocate(NonZeroU64::new(size).unwrap()).unwrap();
//             if drop_stat {
//                 a.free().unwrap();
//                 None
//             } else {
//                 Some(a)
//             }
//         } else {
//             None
//         };

//         println!("iter {} done", i);
//     }
// }
