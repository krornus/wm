use crate::wm::Adapter;
use crate::rect::{Rect, Cut, Split};
use crate::client::Client;

pub trait Layout {
    fn arrange(&mut self, adapter: &mut Adapter, scope: &Rect, clients: &mut [Client]);
}

#[derive(Debug, Clone)]
pub struct Monacle { }

impl Monacle {
    pub fn new() -> Self {
        Monacle { }
    }
}

impl Layout for Monacle {
    fn arrange(&mut self, adapter: &mut Adapter, scope: &Rect, clients: &mut [Client]) {
        for client in clients.iter_mut() {
            if client.focused() {
                client.resize(adapter, scope);
                client.show(adapter, true);
            } else {
                client.show(adapter, false);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct LeftMaster {
    factor: f32,
    rows: usize,
    columns: usize,
}

impl LeftMaster {
    pub fn new() -> Self {
        LeftMaster {
            factor: 0.5,
            rows: 2,
            columns: 1,
        }
    }
}

impl Layout for LeftMaster {
    fn arrange(&mut self, adapter: &mut Adapter, scope: &Rect, clients: &mut [Client]) {
        let mut count = clients.len();
        let mut index = 0;

        if count == 1 {
            /* one client -- full screen */
            clients[0].resize(adapter, scope);
        } else if count <= self.rows {
            /* only enough windows for the masters */
            for master in scope.split(Split::Horizontal(count)) {
                clients[index].resize(adapter, &master);
                index += 1;
            }
        } else {
            /* we have masters and a right grid */
            let master = scope.w as f32 * self.factor;
            let (left, right) = scope.cut(Cut::Vertical(master.round() as u16));

            /* resize master(s) */
            for master in left.split(Split::Horizontal(self.rows)) {
                clients[index].resize(adapter, &master);
                index += 1;
            }

            count -= self.rows;

            /* calculate the number of columns in the stack */
            let columns = if count < self.columns {
                count
            } else {
                self.columns
            };

            /* calculate the number of rows per column. we round up,
             * leaving the last column to possibly contain less rows */
            let mut rows = 1 + ((count - 1) / columns);

            /* now iterate columns, resizing each one */
            for column in right.split(Split::Vertical(dbg!(columns))) {
                /* this is for the final column */
                if rows > count {
                    rows = count;
                }

                count -= rows;

                for window in column.split(Split::Horizontal(dbg!(rows))) {
                    clients[index].resize(adapter, &dbg!(window));
                    index += 1;
                }

            }
        }

    }
}
