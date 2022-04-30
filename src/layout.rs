use crate::rect::Rect;
use crate::wm::{Adapter, Client};

pub trait Layout {
    fn arrange(&mut self, adapter: &mut Adapter, scope: &Rect, clients: &mut [Client]);
}

#[derive(Debug)]
pub struct LeftMaster {
}

impl Layout for LeftMaster {
    fn arrange(&mut self, adapter: &mut Adapter, scope: &Rect, clients: &mut [Client]) {
        let count = clients.len();

        for (index, client) in clients.iter_mut().enumerate() {
            let rect = if index == 0 {
                if count == 1 {
                    Rect::new(0, 0, scope.w, scope.h)
                } else {
                    Rect::new(0, 0, scope.center_x(), scope.h)
                }
            } else {
                /* height of one box */
                let boxh = scope.h / (count - 1);
                /* pos of one box */
                let posh = boxh * (index - 1);

                Rect::new(scope.center_x(), posh, scope.w, posh + boxh)
            };

            client.resize(adapter, rect);
        }
    }
}

impl LeftMaster { }
