#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Method {
    Delete,
    Get,
    Patch,
    Post,
    Put,
    Uninitialised,
}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s.trim() {
            "DELETE" => Self::Delete,
            "GET" => Self::Get,
            "PATCH" => Self::Patch,
            "POST" => Self::Post,
            "PUT" => Self::Put,
            _ => Self::Uninitialised,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialised,
}

impl From<&str> for Version {
    fn from(s: &str) -> Self {
        match s.trim() {
            "HTTP/1.1" => Self::V1_1,
            "HTTP/2.0" => Self::V2_0,
            _ => Self::Uninitialised,
        }
    }
}

use std::{error::Error, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RequestLineParseError;

impl fmt::Display for RequestLineParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "request line must include method, resource, and version")
    }
}

impl Error for RequestLineParseError {}

#[derive(Debug, Clone)]
pub struct RequestLine {
    method: Method,
    resource: Resource,
    version: Version,
}

impl RequestLine {
    #[must_use]
    pub const fn method(&self) -> &Method {
        &self.method
    }

    #[must_use]
    pub const fn resource(&self) -> &Resource {
        &self.resource
    }

    #[must_use]
    pub const fn version(&self) -> &Version {
        &self.version
    }
}

impl TryFrom<&str> for RequestLine {
    type Error = RequestLineParseError;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let mut words = line.split_whitespace();

        let method = words.next().ok_or(RequestLineParseError)?;

        let resource = words.next().ok_or(RequestLineParseError)?;

        let version = words.next().ok_or(RequestLineParseError)?;

        Ok(Self {
            method: method.into(),
            resource: Resource::Path(resource.into()),
            version: version.into(),
        })
    }
}
