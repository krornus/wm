// use crate::wm::Adapter;
// use crate::rect::{Cut, Split};
// use crate::container::{Container, ContainerId};

// pub trait Layout {
//     fn arrange(&mut self, adapter: &mut Adapter, container: &mut Container, id: &ContainerId);
// }

// #[derive(Debug)]
// pub struct LeftMaster {
//     factor: f32,
//     rows: usize,
//     columns: usize,
// }

// impl Layout for LeftMaster {
//     fn arrange(&mut self, adapter: &mut Adapter, container: &mut Container, id: &ContainerId) {
//         let scope = container.get_mut(id)
//             .expect(&format!("scope not found: {}", id));

//         let mut count = scope.clients.len();
//         let mut index = 0;

//         if count == 1 {
//             /* one client -- full screen */
//             scope.clients[0].resize(adapter, scope.rect);
//         } else if count <= self.rows {
//             /* only enough windows for the masters */
//             for master in scope.rect.split(Split::Horizontal(count)) {
//                 scope.clients[index].resize(adapter, master);
//                 index += 1;
//             }
//         } else {
//             /* we have masters and a right grid */
//             let master = scope.rect.w as f32 * self.factor;
//             let (left, right) = scope.rect.cut(Cut::Vertical(master.round() as u16));

//             /* calculate the number of rows in the master grid */
//             let mut rows = if self.rows < count {
//                 count
//             } else {
//                 self.rows
//             };

//             /* resize master(s) */
//             for master in left.split(Split::Horizontal(rows)) {
//                 scope.clients[index].resize(adapter, master);
//                 index += 1;
//             }

//             count -= rows;

//             /* calculate the number of columns in the stack */
//             let columns = if self.columns < count {
//                 count
//             } else {
//                 self.columns
//             };

//             /* calculate the number of rows per column. we round up,
//              * leaving the last column to possibly contain less rows */
//             rows = if count > 0 {
//                 1 + ((count - 1) / columns)
//             } else {
//                 0
//             };

//             /* now iterate columns, resizing each one */
//             for column in right.split(Split::Vertical(columns)) {
//                 /* this is for the final column */
//                 if rows > count {
//                     rows = count;
//                 }

//                 count -= rows;

//                 for window in column.split(Split::Horizontal(rows)) {
//                     scope.clients[index].resize(adapter, window);
//                     index += 1;
//                 }

//             }
//         }

//     }
// }
