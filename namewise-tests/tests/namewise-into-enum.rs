#[derive(Clone, Copy, namewise::Into)]
#[namewise(into = "crate::DestinationB")]
pub enum SourceA {
    First,
    Second,
    Third,
    Fourth,
}

#[derive(PartialEq, Eq, Debug)]
pub enum DestinationB {
    First,
    Second,
    Third,
    Fourth,
}

#[test]
fn test_derive_into_enum() {
    assert_eq!(into_b(SourceA::First), DestinationB::First);
    assert_eq!(into_b(SourceA::Second), DestinationB::Second);
    assert_eq!(into_b(SourceA::Third), DestinationB::Third);
    assert_eq!(into_b(SourceA::Fourth), DestinationB::Fourth);
}

fn into_b(a: impl Into<DestinationB>) -> DestinationB {
    a.into()
}
