#[derive(Clone)]
pub struct SourceA {
    a: String,
    text: &'static str,
    _y: i32,
    numeric: i16,
}

#[derive(namewise::From)]
#[namewise_from(from_type = "crate::SourceA")]
pub struct DestinationB {
    a: String,
    text: String,
    #[namewise_from(from_name = "numeric")]
    number: i64,
    #[namewise_from(from_name = "numeric", mapper = "numeric_mapper")]
    s_number: String,
}

fn numeric_mapper(n: i16) -> String {
    n.to_string()
}

#[test]
fn test_derive_from_struct() {
    let source = SourceA {
        a: "A".to_string(),
        text: "arb-text",
        _y: 23,
        numeric: 42,
    };
    let cloned_source = source.clone();

    let destination: DestinationB = source.into();

    assert_eq!(cloned_source.a, destination.a);
    assert_eq!(cloned_source.text, destination.text.as_str());
    assert_eq!(cloned_source.numeric as i64, destination.number);
    assert_eq!(cloned_source.numeric.to_string(), destination.s_number);
}
