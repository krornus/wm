use std::collections::VecDeque;

use xcb::randr;
use xcb::x::{self, Keycode};

use crate::error::Error;
use crate::comm::Event;
use crate::monitor::Monitors;
use crate::keyboard::{Bindings, Bind};

pub struct Connection {
    raw: xcb::Connection,
    screen: usize,
    root: x::Window,
    events: VecDeque<Event>,
}

impl Connection {
    pub fn connect(name: Option<&str>) -> Result<Self, Error> {
        let (conn, main) = xcb::Connection::connect_with_extensions(
            name,
            &[xcb::Extension::RandR],
            &[])?;

        let setup = conn.get_setup();
        let screen = setup
            .roots()
            .nth(main as usize)
            .ok_or(Error::MissingScreen)?;

        let root = screen.root();

        Ok(Connection {
            raw: conn,
            screen: main as usize,
            root: root,
            events: VecDeque::new(),
        })
    }

    #[inline]
    pub fn raw(&self) -> &xcb::Connection {
        &self.raw
    }

    #[inline]
    pub fn raw_mut(&mut self) -> &mut xcb::Connection {
        &mut self.raw
    }

    #[inline]
    pub fn screen(&self) -> usize {
        self.screen
    }

    #[inline]
    pub fn root(&self) -> x::Window {
        self.root
    }

    #[inline]
    pub fn get_setup(&self) -> &x::Setup {
        self.raw.get_setup()
    }

    #[inline]
    pub fn generate_id<I: xcb::XidNew>(&self) -> I {
        self.raw.generate_id()
    }

    #[inline]
    #[must_use]
    pub fn send_request<R>(&self, req: &R) -> R::Cookie
    where
        R: xcb::Request,
    {
        self.raw.send_request(req)
    }

    #[inline]
    pub fn wait_for_reply<C>(&self, cookie: C) -> Result<C::Reply, Error>
    where
        C: xcb::CookieWithReplyChecked,
    {
        Ok(self.raw.wait_for_reply(cookie)?)
    }

    #[inline]
    #[must_use]
    pub fn send_request_checked<R>(&mut self, request: &R) -> xcb::VoidCookieChecked
    where
        R: xcb::RequestWithoutReply,
    {
        self.raw.send_request_checked(request)
    }

    pub fn send_and_check_request<R>(&self, req: &R) -> xcb::ProtocolResult<()>
    where
        R: xcb::RequestWithoutReply,
    {
        self.raw.send_and_check_request(req)
    }

    #[inline]
    pub fn check_request(&self, cookie: xcb::VoidCookieChecked) -> xcb::ProtocolResult<()> {
        self.raw.check_request(cookie)
    }


    #[inline]
    pub fn push(&mut self, e: Event) {
        self.events.push_front(e);
    }

    #[inline]
    pub fn pop(&mut self) -> Option<Event> {
        self.events.pop_back()
    }
}

pub struct WindowManager {
    monitors: Monitors,
    bindings: Bindings,
}

impl WindowManager {
    pub fn new(conn: &mut Connection) -> Result<Self, Error> {
        let setup = conn.raw.get_setup();
        let roots: Vec<_> = setup.roots().map(|x| x.root()).collect();

        for root in roots {
            conn.raw.send_and_check_request(&x::ChangeWindowAttributes {
                window: root,
                value_list: &[xcb::x::Cw::EventMask(
                    x::EventMask::STRUCTURE_NOTIFY
                        | x::EventMask::PROPERTY_CHANGE
                        | x::EventMask::SUBSTRUCTURE_NOTIFY
                        | x::EventMask::SUBSTRUCTURE_REDIRECT,
                )],
            }).map_err(|_| Error::AlreadyRunning)?;

            conn.raw.send_and_check_request(&randr::SelectInput {
                window: root,
                enable: randr::NotifyMask::SCREEN_CHANGE
                    | randr::NotifyMask::OUTPUT_CHANGE
                    | randr::NotifyMask::CRTC_CHANGE
                    | randr::NotifyMask::OUTPUT_PROPERTY,
            })?;

        }

        let mut mgr = WindowManager {
            monitors: Monitors::new(),
            bindings: Bindings::new(conn)?,
        };

        mgr.screen_change(conn)?;

        Ok(mgr)
    }

    /// Consume an event from the connection
    /// This may or may not generate new events in the output queue
    pub fn consume(&mut self, conn: &mut Connection) -> Result<(), Error> {
        let event = conn.raw.wait_for_event()?;

        match event {
            xcb::Event::RandR(xcb::randr::Event::ScreenChangeNotify(_)) => {
                self.screen_change(conn)
            },
            xcb::Event::X(xcb::x::Event::KeyPress(ref e)) => {
                self.key_press(conn, e)
            },
            xcb::Event::X(xcb::x::Event::KeyRelease(ref e)) => {
                self.key_release(conn, e)
            },
            _ => {
                Ok(())
            },
        };

        Ok(())
    }

    fn screen_change(&mut self, conn: &mut Connection) -> Result<(), Error> {
        let setup = conn.raw.get_setup();
        let roots: Vec<_> = setup.roots().map(|x| x.root()).collect();

        for root in roots {
            self.monitors.update(conn, root)?;
        }

        Ok(())
    }

    fn key_press(&mut self, conn: &mut Connection, event: &xcb::x::KeyPressEvent) -> Result<(), Error> {
        self.bindings.get(event.root(), event.state(), event.detail() as Keycode, true)
            .map(|bind| conn.push(Event::Key {
                root: event.root(),
                keysym: bind.keysym,
                mask: bind.mask,
                press: true,
            }));

        Ok(())
    }

    fn key_release(&mut self, conn: &mut Connection, event: &xcb::x::KeyReleaseEvent) -> Result<(), Error> {
        self.bindings.get(event.root(), event.state(), event.detail() as Keycode, false)
            .map(|bind| conn.push(Event::Key {
                root: event.root(),
                keysym: bind.keysym,
                mask: bind.mask,
                press: false,
            }));

        Ok(())
    }
}
