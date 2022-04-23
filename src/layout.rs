use crate::rect::Rect;

pub trait Layout {
    fn count(&self) -> usize;
    fn arrange_one(&mut self, scope: &Rect, count: usize, index: usize) -> Rect;

    fn arrange_all(&mut self, scope: &Rect, count: usize) -> Vec<Rect> {
        (0..count)
            .map(|i| self.arrange_one(scope, count, i))
            .collect()
    }

    fn arrange(&mut self, scope: &Rect) -> Rect {
        let n = self.count();
        self.arrange_one(scope, n + 1, n)
    }
}

pub struct LeftMaster {
    count: usize,
}

impl Layout for LeftMaster {
    fn count(&self) -> usize {
        self.count
    }

    fn arrange_one(&mut self, scope: &Rect, count: usize, index: usize) -> Rect {
        self.count = self.count + count;

        if index == 0 {
            Rect::new(0, 0, scope.center_x(), scope.h)
        } else {
            /* height of one box */
            let boxh = scope.h / (count - 1);
            /* pos of one box */
            let posh = boxh * (index - 1);

            Rect::new(scope.center_x(), posh, scope.w, posh + boxh)
        }
    }
}

impl LeftMaster {
    pub fn new() -> Self {
        LeftMaster { count: 0 }
    }
}
