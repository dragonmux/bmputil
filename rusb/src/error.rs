use std::{fmt, result, backtrace::Backtrace};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use libusb1_sys::constants::*;

/// A result of a function that may return a `Error`.
pub type Result<T> = result::Result<T, Error>;

/// Kinds of errors for [Error].
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ErrorKind {
    /// Input/output error.
    Io,

    /// Invalid parameter.
    InvalidParam,

    /// Access denied (insufficient permissions).
    Access,

    /// No such device (it may have been disconnected).
    NoDevice,

    /// Entity not found.
    NotFound,

    /// Resource busy.
    Busy,

    /// Operation timed out.
    Timeout,

    /// Overflow.
    Overflow,

    /// Pipe error.
    Pipe,

    /// System call interrupted (perhaps due to signal).
    Interrupted,

    /// Insufficient memory.
    NoMem,

    /// Operation not supported or unimplemented on this platform.
    NotSupported,

    /// The device returned a malformed descriptor.
    BadDescriptor,

    /// Other error.
    Other,
}

impl ErrorKind {
    fn error(self) -> Error {
        Error::new(self, Backtrace::capture())
    }

    fn error_from(self, backtrace: Backtrace) -> Error {
        Error::new(self, backtrace)
    }
}

impl From<ErrorKind> for Error {
    #[inline(always)]
    fn from(kind: ErrorKind) -> Self {
        kind.error()
    }
}

/// Errors returned by the `libusb` library.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Error {
    pub kind: ErrorKind,
    pub backtrace: Box<Backtrace>
}

impl Error {
    #[inline(always)]
    pub fn new(kind: ErrorKind, backtrace: Backtrace) -> Self {
        Self {
            kind,
            backtrace: backtrace.into(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        fmt.write_str(match self.kind {
            ErrorKind::Io => "Input/Output Error",
            ErrorKind::InvalidParam => "Invalid parameter",
            ErrorKind::Access => "Access denied (insufficient permissions)",
            ErrorKind::NoDevice => "No such device (it may have been disconnected)",
            ErrorKind::NotFound => "Entity not found",
            ErrorKind::Busy => "Resource busy",
            ErrorKind::Timeout => "Operation timed out",
            ErrorKind::Overflow => "Overflow",
            ErrorKind::Pipe => "Pipe error",
            ErrorKind::Interrupted => "System call interrupted (perhaps due to signal)",
            ErrorKind::NoMem => "Insufficient memory",
            ErrorKind::NotSupported => "Operation not supported or unimplemented on this platform",
            ErrorKind::BadDescriptor => "Malformed descriptor",
            ErrorKind::Other => "Other error",
        })
    }
}

impl std::error::Error for Error {}

#[doc(hidden)]
pub(crate) fn from_libusb(err: i32) -> Error {
    match err {
        LIBUSB_ERROR_IO => ErrorKind::Io,
        LIBUSB_ERROR_INVALID_PARAM => ErrorKind::InvalidParam,
        LIBUSB_ERROR_ACCESS => ErrorKind::Access,
        LIBUSB_ERROR_NO_DEVICE => ErrorKind::NoDevice,
        LIBUSB_ERROR_NOT_FOUND => ErrorKind::NotFound,
        LIBUSB_ERROR_BUSY => ErrorKind::Busy,
        LIBUSB_ERROR_TIMEOUT => ErrorKind::Timeout,
        LIBUSB_ERROR_OVERFLOW => ErrorKind::Overflow,
        LIBUSB_ERROR_PIPE => ErrorKind::Pipe,
        LIBUSB_ERROR_INTERRUPTED => ErrorKind::Interrupted,
        LIBUSB_ERROR_NO_MEM => ErrorKind::NoMem,
        LIBUSB_ERROR_NOT_SUPPORTED => ErrorKind::NotSupported,
        LIBUSB_ERROR_OTHER | _ => ErrorKind::Other,
    }.error_from(Backtrace::capture())
}

#[doc(hidden)]
macro_rules! try_unsafe {
    ($x:expr) => {
        match unsafe { $x } {
            0 => (),
            err => return Err($crate::error::from_libusb(err)),
        }
    };
}
