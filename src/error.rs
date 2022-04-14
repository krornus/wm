use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("a window manager is already running")]
    AlreadyRunning,
    #[error("screen not found")]
    MissingScreen,
    #[error("XKB version unsupported")]
    XKBUnsupported,
    #[error("Unknown keyboard device")]
    UnknownKeyboard,
    #[error("failed to register signal handler")]
    SignalError(std::io::Error),
    #[error("failed to connect to X11 server")]
    ConnectionError(#[from] xcb::ConnError),
    #[error("io error")]
    IoError(#[from] std::io::Error),
    #[error("xcb error")]
    XCBError(#[from] xcb::Error),
    #[error("protocol error")]
    ProtocolError(#[from] xcb::ProtocolError),
}
