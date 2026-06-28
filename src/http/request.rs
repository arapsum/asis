use std::collections::HashMap;

use crate::{Error, Result};

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Resource {
    Path(String),
}

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
    type Error = Error;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let mut words = line.split_whitespace();

        let method = words
            .next()
            .ok_or_else(|| Error::MalformedRequestLine(line.to_string()))?;

        let resource = words
            .next()
            .ok_or_else(|| Error::MalformedRequestLine(line.to_string()))?;

        let version = words
            .next()
            .ok_or_else(|| Error::MalformedRequestLine(line.to_string()))?;

        Ok(Self {
            method: method.into(),
            resource: Resource::Path(resource.into()),
            version: version.into(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct RequestHeader(String, String);

#[derive(Debug, Clone)]
pub struct Request {
    method: Method,
    resource: Resource,
    version: Version,
    headers: HashMap<String, String>,
    body: String,
}

impl Request {
    /// Parses the request line from a raw HTTP request.
    ///
    /// # Errors
    ///
    /// Returns [`Error::MalformedRequestLine`] when the line does not include a
    /// method, resource, and version.
    pub fn parse_request_line(s: &str) -> Result<RequestLine> {
        let req_line = RequestLine::try_from(s)?;

        Ok(req_line)
    }

    #[must_use]
    pub fn parse_headers(s: &str) -> Option<RequestHeader> {
        let (key, value) = s.split_once(':')?;

        Some(RequestHeader(
            key.trim().to_string(),
            value.trim().to_string(),
        ))
    }

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

    #[must_use]
    pub const fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    #[must_use]
    pub fn body(&self) -> &str {
        &self.body
    }
}

impl TryFrom<String> for Request {
    type Error = Error;

    fn try_from(req: String) -> std::result::Result<Self, Self::Error> {
        let (head, body) = match req.split_once("\r\n\r\n") {
            Some((h, b)) => (h, b),
            None => req.split_once("\n\n").unwrap_or((req.as_str(), "")),
        };

        let mut lines = head.lines();

        let req_line_str = lines.next().ok_or(Error::EmptyRequest)?;
        let req_line = Self::parse_request_line(req_line_str)?;

        let mut headers = HashMap::new();

        for line in lines {
            if let Some(RequestHeader(key, value)) = Self::parse_headers(line) {
                headers.insert(key, value);
            }
        }

        Ok(Self {
            method: req_line.method,
            version: req_line.version,
            resource: req_line.resource,
            headers,
            body: body.to_string(),
        })
    }
}
