#[derive(Clone)]
pub struct SourceA {
    a: String,
    text: &'static str,
    _y: i32,
    number: i16,
}

#[derive(namewise::From)]
#[namewise(from = "SourceA")]
pub struct DestinationB {
    a: String,
    text: String,
    number: i64,
}

#[test]
fn test_namewise_from() {
    let source = SourceA {
        a: "A".to_string(),
        text: "arb-text",
        _y: 23,
        number: 42,
    };
    let cloned_source = source.clone();

    let destination: DestinationB = source.into();

    assert_eq!(cloned_source.a, destination.a);
    assert_eq!(cloned_source.text, destination.text.as_str());
    assert_eq!(cloned_source.number as i64, destination.number);
}
