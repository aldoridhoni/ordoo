//! Representations of various client errors

use std::error::Error as ErrorTrait;
use std::io::Error as IoError;
use std::fmt;
use hyper::Error as HttpError;
use hyper::status::StatusCode;
use rustc_serialize::json::{DecoderError, EncoderError, ParserError};

#[derive(Debug)]
pub enum Error {
    Decoding(DecoderError),
    Encoding(EncoderError),
    Parse(ParserError),
    Http(HttpError),
    IO(IoError),
    Fault { code: StatusCode, message: String },
}

impl From<ParserError> for Error {
    fn from(error: ParserError) -> Error {
        Error::Parse(error)
    }
}

impl From<DecoderError> for Error {
    fn from(error: DecoderError) -> Error {
        Error::Decoding(error)
    }
}

impl From<EncoderError> for Error {
    fn from(error: EncoderError) -> Error {
        Error::Encoding(error)
    }
}

impl From<HttpError> for Error {
    fn from(error: HttpError) -> Error {
        Error::Http(error)
    }
}

impl From<IoError> for Error {
    fn from(error: IoError) -> Error {
        Error::IO(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Docker Error: "));
        match self {
            &Error::Decoding(ref err) => return err.fmt(f),
            &Error::Encoding(ref err) => return err.fmt(f),
            &Error::Parse(ref err) => return err.fmt(f),
            &Error::Http(ref err) => return err.fmt(f),
            &Error::IO(ref err) => return err.fmt(f),
            &Error::Fault { code, .. } => return write!(f, "{}", code),
        };
    }
}

impl ErrorTrait for Error {
    fn description(&self) -> &str {
        "Ordoo Error"
    }

    fn cause(&self) -> Option<&ErrorTrait> {
        match self {
            &Error::Decoding(ref err) => Some(err),
            &Error::Encoding(ref err) => Some(err),
            &Error::Parse(ref err) => Some(err),
            &Error::Http(ref err) => Some(err),
            &Error::IO(ref err) => Some(err),
            _ => None,
        }
    }
}
