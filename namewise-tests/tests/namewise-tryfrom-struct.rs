use namewise::NamewiseError;

#[derive(Clone)]
pub struct SourceA {
    a: String,
    text: &'static str,
    y: i32,
    numeric: Option<i16>,
    z: SourceField,
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
    z: TargetField,
}

fn y_mapper(y: i32) -> String {
    y.to_string()
}

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SourceField {
    A,
    B,
}

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TargetField {
    B,
    C,
}

#[derive(Clone, Debug, strum::Display, thiserror::Error)]
enum FieldError {
    Catastrophic(String),
}

impl TryFrom<SourceField> for TargetField {
    type Error = FieldError;
    fn try_from(value: SourceField) -> Result<Self, Self::Error> {
        if value == SourceField::B {
            Ok(TargetField::B)
        } else {
            Err(FieldError::Catastrophic(format!("Unmatched source variant: {value:?}")))
        }
    }
}

#[test]
fn test_derive_try_from_struct() {
    let source = SourceA {
        a: "A".to_string(),
        text: "arb-text",
        y: 23,
        numeric: Some(12),
        z: SourceField::B,
    };
    let cloned_source = source.clone();

    let destination_res: Result<DestinationB, NamewiseError> = source.try_into();
    let destination = destination_res.unwrap();

    assert_eq!(cloned_source.a, destination.clone().a);
    assert_eq!(cloned_source.text, destination.clone().text.as_str());
    assert_eq!(
        cloned_source.numeric.unwrap() as i64,
        destination.clone().number
    );
    assert_eq!(cloned_source.y.to_string(), destination.clone().y);
    assert_eq!(TargetField::B, destination.clone().z);
}
