#[derive(Clone)]
pub struct Source {
    a: String,
}

#[derive(fieldwise::From)]
// #[fieldwise(from=Source)]
pub struct Destination {
    a: String,
}

#[test]
fn test_fieldwise_from() {
    let source = Source { a: "A".to_string() };
    let cloned_source = source.clone();

    let destination: Destination = source.into();

    assert_eq!(cloned_source.a, destination.a);
}
