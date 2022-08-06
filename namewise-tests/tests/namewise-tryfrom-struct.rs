use namewise::NamewiseError;

#[derive(Clone)]
pub struct SourceA {
    a: String,
    text: &'static str,
    y: i32,
    numeric: Option<i16>,
}

#[derive(namewise::TryFrom, Clone)]
#[namewise_try_from(try_from_type = "crate::SourceA")]
pub struct DestinationB {
    a: String,
    text: String,
    #[namewise_try_from(optional, from_name = "numeric")]
    number: i64,
    #[namewise_try_from(mapper = "y_mapper")]
    y: String,
}

fn y_mapper(y: i32) -> String {
    y.to_string()
}

#[test]
fn test_derive_try_from_struct() {
    let source = SourceA {
        a: "A".to_string(),
        text: "arb-text",
        y: 23,
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
        destination.clone().unwrap().number
    );
    assert_eq!(cloned_source.y.to_string(), destination.clone().unwrap().y);
}
