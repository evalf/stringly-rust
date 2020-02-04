use serde::ser::{self, Serialize};

use crate::error::{Error, Result};
use crate::util;

pub struct Serializer;
pub struct SerializeSequence {
    n: usize,
    output: String,
}
pub struct SerializeVariantSequence {
    variant: &'static str,
    n: usize,
    output: String,
}

/// Serializes an object to Stringly.
pub fn to_string<T>(value: &T) -> Result<String>
where
    T: Serialize,
{
    value.serialize(Serializer)
}

impl ser::Serializer for Serializer {
    type Ok = String;
    type Error = Error;

    type SerializeSeq = SerializeSequence;
    type SerializeTuple = SerializeSequence;
    type SerializeTupleStruct = SerializeSequence;
    type SerializeTupleVariant = SerializeVariantSequence;
    type SerializeMap = SerializeSequence;
    type SerializeStruct = SerializeSequence;
    type SerializeStructVariant = SerializeVariantSequence;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        Ok((if v { "True" } else { "False" }).to_string())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        Ok(v.to_string())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        Ok(v.to_string())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        Ok(v.to_string())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        Ok(v.to_string())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        Ok(v.to_string())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        Ok(v.to_string())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        Ok(v.to_string())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        Ok(v.to_string())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        Ok(v.to_string())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        Ok(v.to_string())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        Ok(v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        Ok(v.to_string())
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok> {
        unimplemented! {}
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Ok("".to_string())
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        let s = value.serialize(self)?;
        if s.starts_with('{') && s.ends_with('}') || s.is_empty() {
            Ok(util::protect_unconditionally(&s))
        } else {
            Ok(s)
        }
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Ok("".to_string())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        Ok("".to_string())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        Ok(variant.to_string())
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        // TODO: check this
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        // TODO: assert '{' and '}' not in `variant`
        let value = value.serialize(self)?;
        if value.is_empty() {
            Ok(variant.to_string())
        } else {
            Ok([variant.to_string(), util::protect_unconditionally(&value)].concat())
        }
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(SerializeSequence {
            n: 0,
            output: String::new(),
        })
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Ok(SerializeSequence {
            n: 0,
            output: String::new(),
        })
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Ok(SerializeSequence {
            n: 0,
            output: String::new(),
        })
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        // TODO: assert '{' and '}' not in `variant`
        Ok(SerializeVariantSequence {
            variant,
            n: 0,
            output: String::new(),
        })
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(SerializeSequence {
            n: 0,
            output: String::new(),
        })
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(SerializeSequence {
            n: 0,
            output: String::new(),
        })
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        // TODO: assert '{' and '}' not in `variant`
        Ok(SerializeVariantSequence {
            variant,
            n: 0,
            output: String::new(),
        })
    }
}

fn protect_comma_or_empty(s: &str) -> String {
    if s.is_empty() {
        "{}".to_string()
    } else {
        util::protect(s, ',')
    }
}

impl ser::SerializeSeq for SerializeSequence {
    type Ok = String;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if self.n != 0 {
            self.output += ",";
        }
        self.n += 1;
        self.output += &protect_comma_or_empty(&value.serialize(Serializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(self.output)
    }
}

impl ser::SerializeTuple for SerializeSequence {
    type Ok = String;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if self.n != 0 {
            self.output += ",";
        }
        self.n += 1;
        self.output += &protect_comma_or_empty(&value.serialize(Serializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(self.output)
    }
}

impl ser::SerializeTupleStruct for SerializeSequence {
    type Ok = String;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if self.n != 0 {
            self.output += ",";
        }
        self.n += 1;
        self.output += &protect_comma_or_empty(&value.serialize(Serializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(self.output)
    }
}

impl ser::SerializeTupleVariant for SerializeVariantSequence {
    type Ok = String;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if self.n != 0 {
            self.output += ",";
        }
        self.n += 1;
        self.output += &protect_comma_or_empty(&value.serialize(Serializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok([
            self.variant.to_string(),
            util::protect_unconditionally(&self.output),
        ]
        .concat())
    }
}

impl ser::SerializeMap for SerializeSequence {
    type Ok = String;
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if self.n != 0 {
            self.output += ",";
        }
        self.n += 1;
        self.output += &util::protect(&key.serialize(Serializer)?, [',', '=']);
        self.output += "=";
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.output += &util::protect(&value.serialize(Serializer)?, ',');
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(self.output)
    }
}

impl ser::SerializeStruct for SerializeSequence {
    type Ok = String;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if self.n != 0 {
            self.output += ",";
        }
        self.n += 1;
        self.output += &util::protect(&key.serialize(Serializer)?, [',', '=']);
        self.output += "=";
        self.output += &util::protect(&value.serialize(Serializer)?, ',');
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(self.output)
    }
}

impl ser::SerializeStructVariant for SerializeVariantSequence {
    type Ok = String;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if self.n != 0 {
            self.output += ",";
        }
        self.n += 1;
        self.output += &util::protect(&key.serialize(Serializer)?, [',', '=']);
        self.output += "=";
        self.output += &util::protect(&value.serialize(Serializer)?, ',');
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok([
            self.variant.to_string(),
            util::protect_unconditionally(&self.output),
        ]
        .concat())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[test]
    fn test_struct() {
        #[derive(Serialize)]
        struct Test {
            int: u32,
            seq: Vec<&'static str>,
        }

        let test = Test {
            int: 1,
            seq: vec!["a", "b"],
        };
        let expected = r#"int=1,seq={a,b}"#;
        assert_eq!(to_string(&test).unwrap(), expected);
    }

    #[test]
    fn test_enum() {
        #[derive(Serialize)]
        enum E {
            Unit,
            Newtype(u32),
            Tuple(u32, u32),
            Struct { a: u32 },
        }

        let u = E::Unit;
        let expected = r#"Unit"#;
        assert_eq!(to_string(&u).unwrap(), expected);

        let n = E::Newtype(1);
        let expected = r#"Newtype{1}"#;
        assert_eq!(to_string(&n).unwrap(), expected);

        let t = E::Tuple(1, 2);
        let expected = r#"Tuple{1,2}"#;
        assert_eq!(to_string(&t).unwrap(), expected);

        let s = E::Struct { a: 1 };
        let expected = r#"Struct{a=1}"#;
        assert_eq!(to_string(&s).unwrap(), expected);
    }
}
