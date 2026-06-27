#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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
