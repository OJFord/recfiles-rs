extern crate serde;

mod descriptor;
mod records;
mod serde_rec;

pub use descriptor::Descriptor;
pub use records::{Record, RecordSet};
pub use serde_rec::{to_string, Serializer};
