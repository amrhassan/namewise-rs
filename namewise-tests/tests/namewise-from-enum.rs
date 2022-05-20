#[derive(Clone, Copy)]
pub enum SourceA {
    First,
    Second,
    Third,
    Fourth,
}

#[derive(namewise::From, PartialEq, Eq, Debug)]
#[namewise(from = "SourceA")]
pub enum DestinationB {
    First,
    Second,
    Third,
    Fourth,
}

#[test]
fn test_namewise_from_enum() {
    assert_eq!(DestinationB::from(SourceA::First), DestinationB::First);
    assert_eq!(DestinationB::from(SourceA::Second), DestinationB::Second);
    assert_eq!(DestinationB::from(SourceA::Third), DestinationB::Third);
    assert_eq!(DestinationB::from(SourceA::Fourth), DestinationB::Fourth);
}
