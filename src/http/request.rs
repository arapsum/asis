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

#[derive(Debug, Clone, PartialEq)]
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
    pub fn method(&self) -> &Method {
        &self.method
    }

    #[must_use]
    pub fn resource(&self) -> &Resource {
        &self.resource
    }

    #[must_use]
    pub fn version(&self) -> &Version {
        &self.version
    }
}

impl From<&str> for RequestLine {
    fn from(line: &str) -> Self {
        let mut words = line.split_whitespace();

        let method = words.next().unwrap();

        let resource = words.next().unwrap();

        let version = words.next().unwrap();

        Self {
            method: method.into(),
            resource: Resource::Path(resource.into()),
            version: version.into(),
        }
    }
}
