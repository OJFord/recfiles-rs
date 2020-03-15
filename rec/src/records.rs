use crate::descriptor::Descriptor;
use serde::ser::SerializeSeq;
use serde::Serialize;
use serde::Serializer;

pub trait Record: Serialize {}

pub struct RecordSet<T: Record> {
    descriptor: Option<Descriptor>,
    records: Vec<T>,
}

impl<T: Record> Serialize for RecordSet<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let len = self.records.len() + self.descriptor.as_ref().map_or(0, |_| 1);
        let mut seq_state = serializer.serialize_seq(Some(len))?;

        if let Some(descriptor) = &self.descriptor {
            seq_state.serialize_element(descriptor)?;
        }

        for record in &self.records {
            seq_state.serialize_element(&record)?;
        }

        seq_state.end()
    }
}

#[cfg(test)]
mod tests {
    extern crate serde_rec;

    use super::*;

    #[test]
    fn test_serialise_custom_record() {
        #[derive(Serialize)]
        #[serde(rename_all = "PascalCase")]
        struct Book {
            author: String,
            title: String,
        }

        impl Record for Book {};

        let odyssey = Book {
            author: String::from("Homer"),
            title: String::from("The Odyssey"),
        };

        assert_eq!(
            serde_rec::to_string(&odyssey).unwrap(),
            "Author: Homer\nTitle: The Odyssey\n\n"
        );
    }

    #[test]
    fn test_serialise_custom_record_set() {
        let book_descriptor = Descriptor {
            name: String::from("Book"),
            mandatory: Some(
                vec!["Author", "Title"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
            ),
            key: Some(vec!["Title"].iter().map(|s| s.to_string()).collect()),
            ..Default::default()
        };

        #[derive(Serialize)]
        #[serde(rename_all = "PascalCase")]
        struct Book {
            author: String,
            title: String,
        }

        impl Record for Book {};

        let odyssey = Book {
            author: String::from("Homer"),
            title: String::from("The Odyssey"),
        };

        let brideshead = Book {
            author: String::from("Waugh"),
            title: String::from("Brideshead Revisited"),
        };

        let set = RecordSet {
            descriptor: Some(book_descriptor),
            records: vec![brideshead, odyssey],
        };

        assert_eq!(
            serde_rec::to_string(&set).unwrap(),
            "%rec: Book\n%mandatory: Author\n%mandatory: Title\n%key: Title\n\nAuthor: Waugh\nTitle: Brideshead Revisited\n\nAuthor: Homer\nTitle: The Odyssey\n\n"
        );
    }
}
