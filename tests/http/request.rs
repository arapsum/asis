use asis::request::{Method, Version};
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
