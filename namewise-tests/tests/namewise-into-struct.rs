use std::collections::HashSet;

#[derive(Clone, namewise::Into)]
#[namewise_into(into_type = "crate::DestinationB")]
pub struct SourceA {
    a: String,
    text: &'static str,
    #[namewise_into(into_name = "number")]
    numeric: i16,
    #[namewise_into(mapper = "truth_mapper")]
    truth: bool,
    #[namewise_into(collect, mapper = "truth_mapper")]
    truths: Vec<bool>,
}

pub struct DestinationB {
    a: String,
    text: String,
    number: i64,
    truth: String,
    truths: HashSet<String>,
}

fn truth_mapper(t: bool) -> String {
    t.to_string()
}

#[test]
fn test_derive_into_struct() {
    let source = SourceA {
        a: "A".to_string(),
        text: "arb-text",
        numeric: 42,
        truth: false,
        truths: vec![false, true, true, false, false],
    };
    let cloned_source = source.clone();

    let destination: DestinationB = source.into();

    assert_eq!(cloned_source.a, destination.a);
    assert_eq!(cloned_source.text, destination.text.as_str());
    assert_eq!(cloned_source.numeric as i64, destination.number);
    assert_eq!(cloned_source.truth.to_string(), destination.truth);
    assert_eq!(destination.truths.len(), 2);
}
