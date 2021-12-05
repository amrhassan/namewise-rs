#[derive(Clone)]
pub struct SourceA {
    a: String,
}

#[derive(fieldwise::From)]
#[fieldwise(from(SourceA))]
pub struct DestinationB {
    a: String,
}

#[test]
fn test_fieldwise_from() {
    let source = SourceA { a: "A".to_string() };
    let cloned_source = source.clone();

    let destination: DestinationB = source.into();

    assert_eq!(cloned_source.a, destination.a);
}
