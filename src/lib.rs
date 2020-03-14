extern crate serde;

mod descriptor;
mod serde_rec;

pub use descriptor::Descriptor;
pub use serde_rec::{to_string, Serializer};
