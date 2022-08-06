use namewise::NamewiseError;

#[derive(Clone)]
pub struct SourceA {
    a: String,
    text: &'static str,
    _y: i32,
    numeric: Option<i16>,
}

#[derive(namewise::TryFrom, Clone)]
#[namewise_try_from(try_from_type = "crate::SourceA")]
pub struct DestinationB {
    a: String,
    text: String,
    #[namewise_try_from(optional, from_name = "numeric")]
    number: i64,
}

#[test]
fn test_derive_try_from_struct() {
    let source = SourceA {
        a: "A".to_string(),
        text: "arb-text",
        _y: 23,
        numeric: Some(12),
    };
    let cloned_source = source.clone();

    let destination: Result<DestinationB, NamewiseError> = source.try_into();

    assert_eq!(cloned_source.a, destination.clone().unwrap().a);
    assert_eq!(
        cloned_source.text,
        destination.clone().unwrap().text.as_str()
    );
    assert_eq!(
        cloned_source.numeric.unwrap() as i64,
        destination.unwrap().number
    );
}
