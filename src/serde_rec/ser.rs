use serde::{ser, Serialize};

use super::error::{Error, Result};

#[derive(Debug)]
pub struct Serializer<'a> {
    field: Option<&'static str>,
    output: &'a mut String,
}

pub fn to_string<T>(value: &T) -> Result<String>
where
    T: Serialize,
{
    let mut output = String::new();
    value.serialize(&mut Serializer::new(&mut output))?;
    Ok(output)
}

impl<'a> Serializer<'a> {
    fn new(output: &'a mut String) -> Self {
        Self {
            field: None,
            output,
        }
    }

    fn field_prefix_len(&self) -> usize {
        match self.field {
            Some(field) => format!("\n{}: ", field).len(),
            None => 0,
        }
    }
}

impl<'a, 'b> ser::Serializer for &'b mut Serializer<'a> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = SerializerSeq<'a, 'b>;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = SerializerStruct<'a, 'b>;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        *self.output += if v { "true" } else { "false" };
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        *self.output += &v.to_string();
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        *self.output += &v.to_string();
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        *self.output += &v.to_string();
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        *self.output += v;
        Ok(())
    }

    fn serialize_bytes(self, _: &[u8]) -> Result<()> {
        unimplemented!()
    }

    fn serialize_none(self) -> Result<()> {
        self.output
            .truncate(self.output.len() - self.field_prefix_len());
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        unimplemented!()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        unimplemented!()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        unimplemented!()
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, _v: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(match self.field {
            Some(_) => Self::SerializeSeq {
                serializer: self,
                length: len.unwrap(),
                mode: SerializerSeqMode::Fields,
                progress: 0,
            },
            None => Self::SerializeSeq {
                serializer: self,
                length: len.unwrap(),
                mode: SerializerSeqMode::Records,
                progress: 0,
            },
        })
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        unimplemented!()
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        unimplemented!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        unimplemented!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        unimplemented!()
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(Self::SerializeStruct { serializer: self })
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        unimplemented!()
    }
}

#[derive(Debug)]
enum SerializerSeqMode {
    Fields,
    Records,
}

#[derive(Debug)]
pub struct SerializerSeq<'a, 'b> {
    serializer: &'b mut Serializer<'a>,
    mode: SerializerSeqMode,
    length: usize,
    progress: usize,
}

impl<'a, 'b> ser::SerializeSeq for SerializerSeq<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        *self.serializer.output += match (&self.mode, self.progress) {
            (SerializerSeqMode::Fields, 0) => String::new(),
            (SerializerSeqMode::Fields, _) => format!("\n{}: ", self.serializer.field.unwrap()),
            _ => String::new(),
        }
        .as_ref();

        value.serialize(&mut Serializer {
            output: &mut *self.serializer.output,
            field: None,
        })?;

        self.progress += 1;
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, _key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn serialize_value<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct SerializerStruct<'a, 'b> {
    serializer: &'b mut Serializer<'a>,
}

impl<'a, 'b> ser::SerializeStruct for SerializerStruct<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, field: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !field.is_empty() {
            *self.serializer.output += format!("{}: ", field).as_ref();
        }

        value.serialize(&mut Serializer {
            output: &mut *self.serializer.output,
            field: Some(field),
        })?;

        *self.serializer.output += "\n";

        Ok(())
    }

    fn end(self) -> Result<()> {
        *self.serializer.output += "\n";
        Ok(())
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _field: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::to_string;
    use serde::Serialize;

    #[test]
    fn test_basic() {
        #[derive(Serialize)]
        struct Test {
            foo: i8,
            bar: String,
        }

        let test = Test {
            foo: 1,
            bar: String::from("hunter2"),
        };

        let expected = "foo: 1\nbar: hunter2\n\n";
        assert_eq!(to_string(&test).unwrap(), expected);
    }

    #[test]
    fn test_vec() {
        #[derive(Serialize)]
        struct Test {
            foobar: Vec<String>,
        }

        let test = Test {
            foobar: vec!["foo", "bar"].iter().map(|s| s.to_string()).collect(),
        };

        let expected = "foobar: foo\nfoobar: bar\n\n";
        assert_eq!(to_string(&test).unwrap(), expected);
    }
}
