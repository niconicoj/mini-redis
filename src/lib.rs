use std::{fmt::Display, net::TcpStream};

use serde::{Deserialize, Serialize};

pub const BIND_ADDRESS: &str = "127.0.0.1:8080";

#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    Write(String, String),
    Read(String),
    Delete(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    Success(Option<String>),
    Failure(Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Error {
    NotFound,
    DecodeError(String),
    EncodeError(String),
}

pub fn send_request(request: Request, stream: &mut TcpStream) -> Result<Response, Error> {
    rmp_serde::encode::write(stream, &request)?;
    let response = rmp_serde::decode::from_read::<_, Response>(stream)?;
    Ok(response)
}

pub fn send_response(response: Response, stream: &mut TcpStream) -> Result<(), Error> {
    rmp_serde::encode::write(stream, &response)?;
    Ok(())
}

impl From<rmp_serde::decode::Error> for Error {
    fn from(value: rmp_serde::decode::Error) -> Self {
        Error::DecodeError(value.to_string())
    }
}

impl From<rmp_serde::encode::Error> for Error {
    fn from(value: rmp_serde::encode::Error) -> Self {
        Error::EncodeError(value.to_string())
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Response::Success(v) => write!(f, "{}", v.as_ref().unwrap_or(&"".to_string())),
            Response::Failure(err) => match err {
                Error::NotFound => write!(f, "not found"),
                Error::DecodeError(err) => write!(f, "{}", err),
                Error::EncodeError(err) => write!(f, "{}", err),
            },
        }
    }
}
