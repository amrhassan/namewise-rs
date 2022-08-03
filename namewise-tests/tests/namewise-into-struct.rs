#[derive(Clone, namewise::Into)]
#[namewise_into(into_type = "crate::DestinationB")]
pub struct SourceA {
    a: String,
    text: &'static str,
    number: i16,
}

pub struct DestinationB {
    a: String,
    text: String,
    number: i64,
}

#[test]
fn test_derive_into_struct() {
    let source = SourceA {
        a: "A".to_string(),
        text: "arb-text",
        number: 42,
    };
    let cloned_source = source.clone();

    let destination: DestinationB = source.into();

    assert_eq!(cloned_source.a, destination.a);
    assert_eq!(cloned_source.text, destination.text.as_str());
    assert_eq!(cloned_source.number as i64, destination.number);
}
