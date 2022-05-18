//! Error and Result module.

use hyper::{header::InvalidHeaderValue, StatusCode};
use std::{
    error::Error as StdError,
    fmt::{self, Display},
    result::Result,
    str::Utf8Error,
};

/// A simple type alias so as to DRY.
pub type ConnectResult<T> = Result<T, Error>;

pub type Cause = Box<dyn StdError + Send + Sync>;

pub struct Error {
    inner: Box<ErrorImpl>,
}

struct ErrorImpl {
    kind: Kind,
    cause: Option<Cause>,
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut f = f.debug_tuple("hyper::Error");
        f.field(&self.inner.kind);
        if let Some(ref cause) = self.inner.cause {
            f.field(cause);
        }
        f.finish()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref cause) = self.inner.cause {
            write!(f, "{}: {}", self.description(), cause)
        } else {
            f.write_str(self.description())
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.inner
            .cause
            .as_ref()
            .map(|cause| &**cause as &(dyn StdError + 'static))
    }
}

impl Error {
    pub(super) fn new(kind: Kind) -> Error {
        Error {
            inner: Box::new(ErrorImpl { kind, cause: None }),
        }
    }

    pub(super) fn with<C: Into<Cause>>(mut self, cause: C) -> Error {
        self.inner.cause = Some(cause.into());
        self
    }

    pub(crate) fn find_source<E: StdError + 'static>(&self) -> Option<&E> {
        let mut cause = self.source();
        while let Some(err) = cause {
            if let Some(ref typed) = err.downcast_ref() {
                return Some(typed);
            }
            cause = err.source();
        }

        // else
        None
    }

    pub(super) fn new_network_error<E: Into<Cause>>(cause: E) -> Self {
        Error::new(Kind::NetworkError).with(cause)
    }

    pub(super) fn new_parsing_error<E: Into<Cause>>(cause: E) -> Self {
        Error::new(Kind::ParsingError).with(cause)
    }

    pub(super) fn new_retry_error<E: Into<Cause>>(cause: E) -> Self {
        Error::new(Kind::RetryError).with(cause)
    }

    pub(super) fn new_internal_error() -> Self {
        Error::new(Kind::InternalError)
    }

    /// The error's standalone message, without the message from the source.
    pub fn message(&self) -> impl fmt::Display + '_ {
        self.description()
    }

    fn description(&self) -> &str {
        match self.inner.kind {
            Kind::HyperError(_) => "this is a Hyper related error!",
            Kind::HyperHttpError(_) => "this is a Hyper HTTP related error!",
            Kind::InternalError => "internal error",
            Kind::InvalidHeaderValue => "invalid header value",
            Kind::NetworkError => "network error",
            Kind::NotImplementedError => "not implemented error",
            Kind::ParsingError => "parsing error",
            Kind::RetryError => "retry error",
            Kind::RequestNotSuccessful(_) => "client returned an unsuccessful HTTP status code",
            Kind::SerdeJsonError(_) => "serde deserialization error",
            Kind::Utf8Error => "parsing bytes experienced a UTF8 error",
        }
    }
}

/// Wrapper type which contains a failed request's status code and body.
#[derive(Debug)]
pub struct RequestNotSuccessful {
    /// Status code returned by the HTTP call to the Melissa API.
    pub status: StatusCode,
    /// Body returned by the HTTP call to the Melissa API.
    pub body: String,
}

impl RequestNotSuccessful {
    /// Create a new unsuccessful request error.
    pub fn new(status: StatusCode, body: String) -> Self {
        Self { status, body }
    }
}

impl StdError for RequestNotSuccessful {}

impl Display for RequestNotSuccessful {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StatusCode: {}, Body: {}", self.status, self.body)
    }
}

#[derive(Debug)]
pub(super) enum Kind {
    /// The failure was due to a Hyper error
    HyperError(hyper::Error),

    /// The failure was due to a Hyper error
    HyperHttpError(hyper::http::Error),

    InternalError,

    InvalidHeaderValue,

    /// The failure was due to the network client not working properly.
    NetworkError,

    NotImplementedError,

    ParsingError,

    RetryError,

    RequestNotSuccessful(RequestNotSuccessful),

    SerdeJsonError(serde_json::Error),

    Utf8Error,
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &Self::HyperError(_) => {
                write!(f, "HyperError")
            }
            &Self::HyperHttpError(_) => {
                write!(f, "HyperHttpError")
            }
            Self::InternalError => {
                write!(f, "InternalError")
            }
            Self::InvalidHeaderValue => {
                write!(f, "InvalidHeaderValue")
            }
            Self::NetworkError => {
                write!(f, "NetworkError")
            }
            Self::NotImplementedError => {
                write!(f, "NotImplementedError")
            }
            Self::ParsingError => {
                write!(f, "ParsingError")
            }
            Self::RetryError => {
                write!(f, "RetryError")
            }
            &Self::RequestNotSuccessful(_) => {
                write!(f, "RequestNotSuccessful")
            }
            &Self::SerdeJsonError(_) => {
                write!(f, "SerdeJsonError")
            }
            Self::Utf8Error => {
                write!(f, "Utf8Error")
            }
        }
    }
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Self {
        Error::new(Kind::HyperError(err))
    }
}

impl From<hyper::http::Error> for Error {
    fn from(err: hyper::http::Error) -> Self {
        Error::new(Kind::HyperHttpError(err))
    }
}

impl From<InvalidHeaderValue> for Error {
    fn from(_err: InvalidHeaderValue) -> Self {
        Error::new(Kind::InvalidHeaderValue)
    }
}

impl From<RequestNotSuccessful> for Error {
    fn from(err: RequestNotSuccessful) -> Self {
        Error::new(Kind::RequestNotSuccessful(err))
    }
}

impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Self {
        Error::new(Kind::Utf8Error).with(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::new(Kind::SerdeJsonError(err))
    }
}
