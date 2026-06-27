use asis::request::{Method, RequestLine, Resource, Version};
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
    let request_line = RequestLine::from(line);

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
#[should_panic]
#[test]
fn panics_when_request_line_has_fewer_than_three_tokens(#[case] line: &str) {
    let _ = RequestLine::from(line);
}
