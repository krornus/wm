pub enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

pub trait Contains<T> {
    fn contains(&self, item: &T) -> bool;
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Point {
    x: i16,
    y: i16,
}

/* we can derive clone for this -- its a u64 */
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Rect {
    x: i16,
    y: i16,
    w: u16,
    h: u16,
}

impl Contains<Point> for Rect {
    fn contains(&self, point: &Point) -> bool {
        point.x < self.w as i16 && point.y < self.h as i16 &&
        point.x >= self.x && point.y >= self.y
    }
}

impl Contains<Rect> for Rect {
    fn contains(&self, other: &Rect) -> bool {
        self.contains(&other.corner(Corner::TopLeft)) &&
        self.contains(&other.corner(Corner::BottomRight))
    }
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

    pub fn corner(&self, at: Corner) -> Point {
        match at {
            Corner::TopLeft => {
                Point {
                    x: self.x,
                    y: self.y,
                }
            }
            Corner::TopRight => {
                Point {
                    x: self.x + self.w as i16,
                    y: self.y,
                }
            }
            Corner::BottomLeft => {
                Point {
                    x: self.x,
                    y: self.y + self.h as i16,
                }
            }
            Corner::BottomRight => {
                Point {
                    x: self.x + self.w as i16,
                    y: self.y + self.h as i16,
                }
            }
        }
    }
}
