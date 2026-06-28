use asis::{
    Error,
    request::{Method, Request, RequestLine, Resource, Version},
};
use rstest::rstest;

#[rstest]
#[case("GET", Method::Get)]
#[case("DELETE", Method::Delete)]
#[case("POST", Method::Post)]
#[case("PATCH", Method::Patch)]
#[case("PUT", Method::Put)]
#[case("UNKOWN", Method::Uninitialised)]
#[test]
fn can_convert_str_to_method(#[case] value: &str, #[case] expected: Method) {
    let actual = Method::from(value);

    assert_eq!(actual, expected);
}

#[rstest]
#[case("HTTP/1.1", Version::V1_1)]
#[case("HTTP/2.0", Version::V2_0)]
#[case("UNKOWN", Version::Uninitialised)]
#[test]
fn can_convert_str_to_version(#[case] value: &str, #[case] expected: Version) {
    let actual = Version::from(value);

    assert_eq!(actual, expected);
}

#[rstest]
#[case::get_products("GET /products HTTP/1.1", Method::Get, "/products", Version::V1_1)]
#[case::delete_root("DELETE / HTTP/1.1", Method::Delete, "/", Version::V1_1)]
#[case::post_nested_resource(
    "POST /products/123/reviews HTTP/2.0",
    Method::Post,
    "/products/123/reviews",
    Version::V2_0
)]
#[case::patch_with_query_string(
    "PATCH /products?category=books&page=2 HTTP/1.1",
    Method::Patch,
    "/products?category=books&page=2",
    Version::V1_1
)]
#[case::put_with_repeated_whitespace(
    "  PUT   /products/123    HTTP/2.0  ",
    Method::Put,
    "/products/123",
    Version::V2_0
)]
#[case::unknown_method("TRACE /jobs HTTP/1.1", Method::Uninitialised, "/jobs", Version::V1_1)]
#[case::unknown_version(
    "GET /products HTTP/3.0",
    Method::Get,
    "/products",
    Version::Uninitialised
)]
#[test]
fn can_convert_str_to_request_line(
    #[case] line: &str,
    #[case] expected_method: Method,
    #[case] expected_resource: &str,
    #[case] expected_version: Version,
) {
    let request_line = RequestLine::try_from(line).expect("request line should parse");

    assert_eq!(request_line.version(), &expected_version);
    assert_eq!(request_line.method(), &expected_method);
    assert_eq!(
        request_line.resource(),
        &Resource::Path(expected_resource.to_string())
    );
}

#[rstest]
#[case::empty("")]
#[case::missing_resource("GET")]
#[case::missing_version("GET /products")]
#[test]
fn returns_error_when_request_line_has_fewer_than_three_tokens(#[case] line: &str) {
    assert!(RequestLine::try_from(line).is_err());
}

#[rstest]
#[case::crlf_with_headers_and_body(
    "POST /products HTTP/1.1\r\nHost: example.com\r\nContent-Type: application/json\r\nContent-Length: 15\r\n\r\n{\"name\":\"book\"}",
    Method::Post,
    "/products",
    Version::V1_1,
    vec![
        ("Host", "example.com"),
        ("Content-Type", "application/json"),
        ("Content-Length", "15"),
    ],
    "{\"name\":\"book\"}"
)]
#[case::lf_with_query_and_body(
    "PATCH /products/123?notify=true HTTP/2.0\nAuthorization: Bearer token\nAccept: application/json\n\n{\"price\":42}",
    Method::Patch,
    "/products/123?notify=true",
    Version::V2_0,
    vec![
        ("Authorization", "Bearer token"),
        ("Accept", "application/json"),
    ],
    "{\"price\":42}"
)]
#[case::request_without_headers_or_body(
    "GET /health HTTP/1.1",
    Method::Get,
    "/health",
    Version::V1_1,
    vec![],
    ""
)]
#[case::request_with_headers_and_no_body(
    "DELETE /products/123 HTTP/1.1\r\nHost: api.example.com\r\n\r\n",
    Method::Delete,
    "/products/123",
    Version::V1_1,
    vec![("Host", "api.example.com")],
    ""
)]
#[test]
fn can_parse_request(
    #[case] raw_request: &str,
    #[case] expected_method: Method,
    #[case] expected_resource: &str,
    #[case] expected_version: Version,
    #[case] expected_headers: Vec<(&str, &str)>,
    #[case] expected_body: &str,
) {
    let request = Request::try_from(raw_request.to_string()).expect("request should parse");

    assert_eq!(request.method(), &expected_method);
    assert_eq!(
        request.resource(),
        &Resource::Path(expected_resource.to_string())
    );
    assert_eq!(request.version(), &expected_version);
    assert_eq!(request.headers().len(), expected_headers.len());

    for (key, value) in expected_headers {
        assert_eq!(request.headers().get(key).map(String::as_str), Some(value));
    }

    assert_eq!(request.body(), expected_body);
}

#[test]
fn trims_header_keys_and_values() {
    let raw_request = "GET /products HTTP/1.1\r\n  Host  :  example.com  \r\n\r\n";

    let request = Request::try_from(raw_request.to_string()).expect("request should parse");

    assert_eq!(
        request.headers().get("Host").map(String::as_str),
        Some("example.com")
    );
}

#[test]
fn keeps_colons_after_the_first_header_separator_in_the_value() {
    let raw_request = "GET /products HTTP/1.1\r\nReferer: http://example.com/products:123\r\n\r\n";

    let request = Request::try_from(raw_request.to_string()).expect("request should parse");

    assert_eq!(
        request.headers().get("Referer").map(String::as_str),
        Some("http://example.com/products:123")
    );
}

#[test]
fn ignores_header_lines_without_colons() {
    let raw_request = "GET /products HTTP/1.1\r\nHost: example.com\r\ninvalid header\r\n\r\n";

    let request = Request::try_from(raw_request.to_string()).expect("request should parse");

    assert_eq!(request.headers().len(), 1);
    assert_eq!(
        request.headers().get("Host").map(String::as_str),
        Some("example.com")
    );
}

#[test]
fn later_duplicate_header_replaces_previous_value() {
    let raw_request =
        "GET /products HTTP/1.1\r\nHost: first.example.com\r\nHost: second.example.com\r\n\r\n";

    let request = Request::try_from(raw_request.to_string()).expect("request should parse");

    assert_eq!(request.headers().len(), 1);
    assert_eq!(
        request.headers().get("Host").map(String::as_str),
        Some("second.example.com")
    );
}

#[rstest]
#[case::empty("")]
#[case::only_blank_separator("\r\n\r\n")]
#[test]
fn returns_empty_request_error_when_request_has_no_request_line(#[case] raw_request: &str) {
    let err = Request::try_from(raw_request.to_string()).expect_err("request should fail");

    assert!(matches!(err, Error::EmptyRequest));
    assert_eq!(err.to_string(), "Request line is empty");
}

#[rstest]
#[case::only_line_separator("\r\n")]
#[case::missing_resource("GET")]
#[case::missing_version("GET /products")]
#[case::missing_version_with_headers("GET /products\r\nHost: example.com\r\n\r\n")]
#[test]
fn returns_malformed_request_line_error_when_request_line_is_incomplete(#[case] raw_request: &str) {
    let err = Request::try_from(raw_request.to_string()).expect_err("request should fail");

    assert!(matches!(err, Error::MalformedRequestLine(_)));
    assert_eq!(
        err.to_string(),
        raw_request.lines().next().unwrap_or_default()
    );
}

#[rstest]
#[case::empty("")]
#[case::missing_resource("GET")]
#[case::missing_version("GET /products")]
#[test]
fn parse_request_line_returns_malformed_request_line_error(#[case] line: &str) {
    let err = Request::parse_request_line(line).expect_err("request line should fail");

    assert!(matches!(err, Error::MalformedRequestLine(_)));
    assert_eq!(err.to_string(), line);
}
