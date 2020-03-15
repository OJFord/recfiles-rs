use serde::Serialize;

type Field = String;

#[derive(Default, Serialize)]
pub struct Descriptor {
    #[serde(rename = "%rec")]
    pub name: String,
    #[serde(rename = "%mandatory")]
    pub mandatory: Option<Vec<Field>>,
    #[serde(rename = "%allowed")]
    pub allowed: Option<Vec<Field>>,
    #[serde(rename = "%prohibit")]
    pub prohibited: Option<Vec<Field>>,
    #[serde(rename = "%unique")]
    pub unique: Option<Vec<Field>>,
    #[serde(rename = "%key")]
    pub key: Option<Vec<Field>>,
    #[serde(rename = "%doc")]
    pub description: Option<String>,
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
