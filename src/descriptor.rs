use serde::Serialize;

type Field = String;

#[derive(Default, Serialize)]
pub struct Descriptor {
    #[serde(rename = "%rec")]
    name: String,
    #[serde(rename = "%mandatory")]
    mandatory: Option<Vec<Field>>,
    #[serde(rename = "%allowed")]
    allowed: Option<Vec<Field>>,
    #[serde(rename = "%prohibit")]
    prohibited: Option<Vec<Field>>,
    #[serde(rename = "%unique")]
    unique: Option<Vec<Field>>,
    #[serde(rename = "%key")]
    key: Option<Vec<Field>>,
    #[serde(rename = "%doc")]
    doc: Option<String>,
}

#[cfg(test)]
mod tests {
    use crate::Descriptor;

    fn strings_from_strs(vec: Vec<&str>) -> Vec<String> {
        vec.iter().map(|s| s.to_string()).collect()
    }

    mod ser {
        use super::*;
        use crate::to_string;

        #[test]
        fn test_default() {
            let test = Descriptor {
                name: String::from("Test"),
                ..Default::default()
            };

            assert_eq!(to_string(&test).unwrap(), "%rec: Test\n\n")
        }

        #[test]
        fn test_field_lists() {
            let test = Descriptor {
                name: String::from("Test"),
                mandatory: Some(strings_from_strs(vec!["Foo", "Bar"])),
                unique: Some(strings_from_strs(vec!["Bar"])),
                ..Default::default()
            };

            assert_eq!(
                to_string(&test).unwrap(),
                "%rec: Test\n%mandatory: Foo\n%mandatory: Bar\n%unique: Bar\n\n"
            )
        }
    }
}
