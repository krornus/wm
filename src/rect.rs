use std::fmt;

use xcb::x;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

/* we can derive copy for this -- its a u64 */
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Rect {
    pub x: i16,
    pub y: i16,
    pub w: u16,
    pub h: u16,
}

pub enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

pub enum Cut {
    Horizontal(u16),
    Vertical(u16),
}

pub trait Contains<T> {
    fn contains(&self, item: &T) -> bool;
}

pub enum Split {
    Horizontal(usize),
    Vertical(usize),
}

pub struct HorizontalSplit<'a> {
    rect: &'a Rect,
    count: usize,
    index: usize,
    bottom: i16,
    height: u16,
}

pub struct VerticalSplit<'a> {
    rect: &'a Rect,
    count: usize,
    index: usize,
    right: i16,
    width: u16,
}

pub enum SplitIterator<'a> {
    Horizontal(HorizontalSplit<'a>),
    Vertical(VerticalSplit<'a>),
}

impl Rect {
    pub fn new(x: i16, y: i16, w: u16, h: u16) -> Self {
        if w > (i16::MAX as u16) || h > (i16::MAX as u16) {
            panic!("Rect: integer overflow: {}x{}+{}x{}", x, y, w, y)
        }

        Rect {
            x: x,
            y: y,
            w: w,
            h: h,
        }
    }

    #[inline]
    pub fn top(&self) -> i16 {
        self.y
    }

    #[inline]
    pub fn bottom(&self) -> i16 {
        self.y + self.h as i16
    }

    #[inline]
    pub fn left(&self) -> i16 {
        self.x
    }

    #[inline]
    pub fn right(&self) -> i16 {
        self.x + self.w as i16
    }

    #[inline]
    pub fn center(&self) -> Point {
        Point {
            x: self.x + self.w as i16 / 2,
            y: self.y + self.h as i16 / 2,
        }
    }

    pub fn corner(&self, at: Corner) -> Point {
        match at {
            Corner::TopLeft => Point {
                x: self.x,
                y: self.y,
            },
            Corner::TopRight => Point {
                x: self.x + self.w as i16,
                y: self.y,
            },
            Corner::BottomLeft => Point {
                x: self.x,
                y: self.y + self.h as i16,
            },
            Corner::BottomRight => Point {
                x: self.x + self.w as i16,
                y: self.y + self.h as i16,
            },
        }
    }

    pub fn cut(&self, at: Cut) -> (Rect, Rect) {
        match at {
            Cut::Horizontal(mut n) => {
                if n > self.h || n > i16::MAX as u16 {
                    n = self.h;
                }

                (
                    Rect::new(self.x, self.y, self.w, n),
                    Rect::new(self.x, self.y + n as i16, self.w, self.h - n),
                )
            }
            Cut::Vertical(mut n) => {
                if n > self.w || n > i16::MAX as u16 {
                    n = self.w;
                }

                (
                    Rect::new(self.x, self.y, n, self.h),
                    Rect::new(self.x + n as i16, self.y, self.w - n, self.h),
                )
            }
        }
    }

    pub fn split<'a>(&'a self, at: Split) -> SplitIterator<'a> {
        match at {
            Split::Horizontal(n) => SplitIterator::Horizontal(HorizontalSplit::new(self, n)),
            Split::Vertical(n) => SplitIterator::Vertical(VerticalSplit::new(self, n)),
        }
    }
}

impl Contains<Point> for Rect {
    fn contains(&self, point: &Point) -> bool {
        point.x < self.w as i16 && point.y < self.h as i16 && point.x >= self.x && point.y >= self.y
    }
}

impl Contains<Rect> for Rect {
    fn contains(&self, other: &Rect) -> bool {
        self.contains(&other.corner(Corner::TopLeft))
            && self.contains(&other.corner(Corner::BottomRight))
    }
}

impl From<Rect> for x::Rectangle {
    fn from(rect: Rect) -> Self {
        x::Rectangle {
            x: rect.x,
            y: rect.y,
            width: rect.w,
            height: rect.h,
        }
    }
}

impl From<&Rect> for x::Rectangle {
    fn from(rect: &Rect) -> Self {
        x::Rectangle {
            x: rect.x,
            y: rect.y,
            width: rect.w,
            height: rect.h,
        }
    }
}

impl<'a> HorizontalSplit<'a> {
    fn new(rect: &'a Rect, count: usize) -> Self {
        if count > i16::MAX as usize {
            panic!("Rect::split: integer out of bounds");
        }

        let height = if count > 0 {
            1 + ((rect.h - 1) / count as u16)
        } else {
            0
        };

        HorizontalSplit {
            rect: rect,
            count: count,
            index: 0,
            bottom: rect.y,
            height: height,
        }
    }
}

impl<'a> VerticalSplit<'a> {
    fn new(rect: &'a Rect, count: usize) -> Self {
        if count > i16::MAX as usize {
            panic!("Rect::split: integer out of bounds");
        }

        let width = if count > 0 {
            1 + ((rect.w - 1) / count as u16)
        } else {
            0
        };

        VerticalSplit {
            rect: rect,
            count: count,
            index: 0,
            right: rect.x,
            width: width,
        }
    }
}

impl<'a> Iterator for HorizontalSplit<'a> {
    type Item = Rect;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            None
        } else if self.index < self.count - 1 {
            let mut y = self.rect.y + (self.index * self.height as usize) as i16;
            let mut height = self.height;

            /* for splits where the number of splits is greater than the available size */
            if y >= self.rect.bottom() {
                y = self.rect.bottom();
                height = 0;
            }

            self.bottom = y + height as i16;
            self.index += 1;

            Some(Rect {
                x: self.rect.x,
                y: y,
                w: self.rect.w,
                h: height,
            })
        } else if self.index == self.count - 1 {
            self.index += 1;

            Some(Rect {
                x: self.rect.x,
                y: self.bottom,
                w: self.rect.w,
                h: self.rect.h - (self.bottom - self.rect.y) as u16,
            })
        } else {
            None
        }
    }
}

impl<'a> Iterator for VerticalSplit<'a> {
    type Item = Rect;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            None
        } else if self.index < self.count - 1 {
            let mut x = self.rect.x + (self.index * self.width as usize) as i16;
            let mut width = self.width;

            /* for splits where the number of splits is greater than the available size */
            if x >= self.rect.right() {
                x = self.rect.right();
                width = 0;
            }

            self.right = x + width as i16;
            self.index += 1;

            Some(Rect {
                x: x,
                y: self.rect.y,
                w: width,
                h: self.rect.h,
            })
        } else if self.index == self.count - 1 {
            self.index += 1;

            Some(Rect {
                x: self.right,
                y: self.rect.y,
                w: self.rect.w - (self.right - self.rect.x) as u16,
                h: self.rect.h,
            })
        } else {
            None
        }
    }
}

impl<'a> Iterator for SplitIterator<'a> {
    type Item = Rect;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            SplitIterator::Vertical(v) => v.next(),
            SplitIterator::Horizontal(h) => h.next(),
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}x{}", self.x, self.y)
    }
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}x{}+{}+{}", self.w, self.h, self.x, self.y)
    }
}

#[cfg(test)]
mod split_tests {
    use super::*;

    #[test]
    fn test_vertical_split() {
        let rect = Rect::new(0, 0, 7, 5);
        let mut col = rect.split(Split::Vertical(3));
        assert_eq!(col.next().unwrap(), Rect::new(0, 0, 3, 5));
        assert_eq!(col.next().unwrap(), Rect::new(3, 0, 3, 5));
        assert_eq!(col.next().unwrap(), Rect::new(6, 0, 1, 5));
        assert!(col.next().is_none());
    }

    #[test]
    fn test_horizontal_split() {
        let rect = Rect::new(0, 0, 5, 7);
        let mut row = rect.split(Split::Horizontal(3));
        assert_eq!(row.next().unwrap(), Rect::new(0, 0, 5, 3));
        assert_eq!(row.next().unwrap(), Rect::new(0, 3, 5, 3));
        assert_eq!(row.next().unwrap(), Rect::new(0, 6, 5, 1));
        assert!(row.next().is_none());
    }

    #[test]
    fn test_empty_split() {
        let rect = Rect::new(0, 0, 5, 7);

        let mut col = rect.split(Split::Vertical(0));
        assert!(col.next().is_none());

        let mut row = rect.split(Split::Horizontal(0));
        assert!(row.next().is_none());
    }

    #[test]
    fn test_single_split() {
        let rect = Rect::new(0, 0, 5, 7);

        let mut col = rect.split(Split::Vertical(1));
        assert_eq!(col.next().unwrap(), rect);

        let mut row = rect.split(Split::Horizontal(1));
        assert_eq!(row.next().unwrap(), rect);
    }

    #[test]
    #[should_panic]
    fn test_split_panic() {
        let rect = Rect::new(0, 0, 5, 7);

        rect.split(Split::Vertical(i16::MAX as usize + 1));
        rect.split(Split::Horizontal(i16::MAX as usize + 1));
    }

    #[test]
    fn test_horizontal_split_max() {
        let rect = Rect::new(0, 0, 5, 7);

        let mut col = rect.split(Split::Horizontal(i16::MAX as usize));

        for i in 0i16..7i16 {
            assert_eq!(col.next().unwrap(), Rect::new(0, i, 5, 1));
        }

        for i in 7i16..i16::MAX {
            assert_eq!(col.next().unwrap(), Rect::new(0, 7, 5, 0));
        }

        assert!(col.next().is_none());
    }

    #[test]
    fn test_vertical_split_max() {
        let rect = Rect::new(0, 0, 7, 5);

        let mut col = rect.split(Split::Vertical(i16::MAX as usize));

        for i in 0i16..7i16 {
            assert_eq!(col.next().unwrap(), Rect::new(i, 0, 1, 5));
        }

        for i in 7i16..i16::MAX {
            assert_eq!(col.next().unwrap(), Rect::new(7, 0, 0, 5));
        }

        assert!(col.next().is_none());
    }
}
