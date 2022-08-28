use crate::wm::Connection;
use crate::error::Error;
use crate::rect::Rect;

use xcb::x;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pixel: u32,
}

pub struct Painter {
    gc: x::Gcontext,
    drawable: x::Drawable,
    colormap: x::Colormap,
    black: Color,
    white: Color,
    foreground: Color,
    background: Color,
}

impl Painter {
    pub fn new<T>(conn: &mut Connection<T>, window: x::Window) -> Result<Self, Error> {
        let gc: x::Gcontext = conn.generate_id();
        let drawable = x::Drawable::Window(window);

        let setup = conn.raw().get_setup();
        let screen = setup
            .roots()
            .nth(conn.screen())
            .unwrap();

        let white = Color { pixel: screen.white_pixel() };
        let black = Color { pixel: screen.black_pixel() };
        let colormap = screen.default_colormap();

        conn.send_and_check_request(&x::CreateGc {
            cid: gc,
            drawable: drawable,
            value_list: &[],
        })?;

        Ok(Painter {
            gc: gc,
            drawable: drawable,
            colormap: colormap,
            white: white,
            black: black,
            foreground: white,
            background: black,
        })
    }

    #[inline]
    pub fn white(&self) -> Color {
        self.white
    }

    #[inline]
    pub fn black(&self) -> Color {
        self.black
    }

    pub fn color<T>(&mut self, conn: &mut Connection<T>, r: u16, g: u16, b: u16) -> Result<Color, Error> {
        let cookie = conn.send_request(&x::AllocColor {
            cmap: self.colormap,
            red: r * 256,
            green: g * 256,
            blue: b * 256,
        });

        let reply = conn.wait_for_reply(cookie)?;

        Ok(Color { pixel: dbg!(reply.pixel()) })
    }

    pub fn brush<T>(&mut self, conn: &mut Connection<T>, foreground: Color, background: Color) -> Result<(), Error> {
        if self.foreground != foreground && self.background != background {
            conn.send_and_check_request(&x::ChangeGc {
                gc: self.gc,
                value_list: &[
                    x::Gc::Foreground(foreground.pixel),
                    x::Gc::Background(background.pixel),
                ],
            })?;

            self.foreground = dbg!(foreground);
            self.background = dbg!(background);

        } else if self.foreground != foreground {
            conn.send_and_check_request(&x::ChangeGc {
                gc: self.gc,
                value_list: &[
                    x::Gc::Foreground(foreground.pixel),
                ],
            })?;

            self.foreground = foreground;

        } else if self.background != background {
            conn.send_and_check_request(&x::ChangeGc {
                gc: self.gc,
                value_list: &[
                    x::Gc::Background(background.pixel),
                ],
            })?;

            self.background = background;
        };

        Ok(())
    }

    pub fn rect<T>(&mut self, conn: &mut Connection<T>, rect: &Rect) -> Result<(), Error> {
        let area = x::Rectangle::from(rect);

        conn.send_and_check_request(&x::PolyFillRectangle {
            drawable: self.drawable,
            gc: self.gc,
            rectangles: &[area],
        })?;

        Ok(())
    }
}
