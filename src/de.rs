use serde::de::{
    self, Deserialize, DeserializeSeed, EnumAccess, MapAccess, SeqAccess, VariantAccess, Visitor,
};

use crate::error::{Error, Result};
use crate::util;

pub struct Deserializer<T> {
    input: T,
}

struct DeserializeSequence<'a, 'b> {
    iter: &'b mut util::SafesplitIter<'a>,
}
struct DeserializeMap<'a, 'b> {
    iter: &'b mut util::SafesplitIter<'a>,
    value: Option<&'a str>,
}
struct DeserializeEnum<'a> {
    variant: &'a str,
    value: &'a str,
}

#[allow(clippy::should_implement_trait)]
impl<'de> Deserializer<&'de str> {
    pub fn from_str(input: &'de str) -> Self {
        Deserializer { input }
    }
}

/// Deserializes an object from Stringly.
pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: Deserialize<'a>,
{
    T::deserialize(Deserializer::from_str(s))
}

impl<'de> de::Deserializer<'de> for Deserializer<&'de str> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match &self.input.to_ascii_lowercase() as &str {
            "true" | "yes" => visitor.visit_bool(true),
            "false" | "no" => visitor.visit_bool(false),
            _ => Err(Error::NotABoolean),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.input.parse() {
            Ok(v) => visitor.visit_i8(v),
            Err(_) => Err(Error::NotAnInteger),
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.input.parse() {
            Ok(v) => visitor.visit_i16(v),
            Err(_) => Err(Error::NotAnInteger),
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.input.parse() {
            Ok(v) => visitor.visit_i32(v),
            Err(_) => Err(Error::NotAnInteger),
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.input.parse() {
            Ok(v) => visitor.visit_i64(v),
            Err(_) => Err(Error::NotAnInteger),
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.input.parse() {
            Ok(v) => visitor.visit_u8(v),
            Err(_) => Err(Error::NotAnUnsignedInteger),
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.input.parse() {
            Ok(v) => visitor.visit_u16(v),
            Err(_) => Err(Error::NotAnUnsignedInteger),
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.input.parse() {
            Ok(v) => visitor.visit_u32(v),
            Err(_) => Err(Error::NotAnUnsignedInteger),
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.input.parse() {
            Ok(v) => visitor.visit_u64(v),
            Err(_) => Err(Error::NotAnUnsignedInteger),
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.input.parse() {
            Ok(v) => visitor.visit_f32(v),
            Err(_) => Err(Error::NotAFloatingPointNumber),
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.input.parse() {
            Ok(v) => visitor.visit_f64(v),
            Err(_) => Err(Error::NotAFloatingPointNumber),
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut chars = self.input.chars();
        match (chars.next(), chars.next()) {
            (Some(ch), None) => visitor.visit_char(ch),
            _ => Err(Error::NotASingleCharacter),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_str(self.input)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_str(self.input)
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.input.len() {
            0 => visitor.visit_none(),
            _ => visitor.visit_some(Deserializer::from_str(util::unprotect(self.input))),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.input.len() {
            0 => visitor.visit_unit(),
            _ => Err(Error::UnexpectedValueForUnit),
        }
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.input.len() {
            0 => visitor.visit_unit(),
            _ => Err(Error::UnexpectedValueForUnit),
        }
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut iter = util::safesplit(self.input, ',');
        let v = visitor.visit_seq(DeserializeSequence { iter: &mut iter });
        match iter.next() {
            None => v,
            Some(_) => Err(Error::TooManyElements),
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut iter = util::safesplit(self.input, ',');
        let v = visitor.visit_seq(DeserializeSequence { iter: &mut iter });
        match iter.next() {
            None => v,
            Some(_) => Err(Error::TooManyElements),
        }
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut iter = util::safesplit(self.input, ',');
        let v = visitor.visit_seq(DeserializeSequence { iter: &mut iter });
        match iter.next() {
            None => v,
            Some(_) => Err(Error::TooManyElements),
        }
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut iter = util::safesplit(self.input, ',');
        let v = visitor.visit_map(DeserializeMap {
            iter: &mut iter,
            value: None,
        });
        match iter.next() {
            None => v,
            Some(_) => Err(Error::TooManyElements),
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut iter = util::safesplit(self.input, ',');
        let v = visitor.visit_map(DeserializeMap {
            iter: &mut iter,
            value: None,
        });
        match iter.next() {
            None => v,
            Some(_) => Err(Error::TooManyElements),
        }
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (variant, value) = util::splitarg(self.input)?;
        visitor.visit_enum(DeserializeEnum { variant, value })
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_str(self.input)
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
}

impl<'de, 'b> SeqAccess<'de> for DeserializeSequence<'de, 'b> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some(s) => seed
                .deserialize(Deserializer::from_str(util::unprotect(s)))
                .map(Some),
            None => Ok(None),
        }
    }
}

impl<'de, 'b> MapAccess<'de> for DeserializeMap<'de, 'b> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some(s) => match util::safesplit_once(s, '=') {
                Ok((key, value)) => {
                    self.value = Some(value);
                    seed.deserialize(Deserializer::from_str(util::unprotect(key)))
                        .map(Some)
                }
                Err(_) => Err(Error::NotAKeyValuePair),
            },
            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        match self.value {
            Some(s) => {
                self.value = None;
                seed.deserialize(Deserializer::from_str(util::unprotect(s)))
            }
            None => {
                panic! {"next_key_seed not called before next_value_seed"}
            }
        }
    }
}

impl<'de> EnumAccess<'de> for DeserializeEnum<'de> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: DeserializeSeed<'de>,
    {
        Ok((
            seed.deserialize(Deserializer::from_str(self.variant))?,
            self,
        ))
    }
}

impl<'de> VariantAccess<'de> for DeserializeEnum<'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        match self.value.len() {
            0 => Ok(()),
            _ => Err(Error::UnexpectedValueForUnit),
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: DeserializeSeed<'de>,
    {
        seed.deserialize(Deserializer::from_str(self.value))
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        de::Deserializer::deserialize_seq(Deserializer::from_str(self.value), visitor)
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        de::Deserializer::deserialize_map(Deserializer::from_str(self.value), visitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[test]
    fn test_struct() {
        #[derive(Deserialize, PartialEq, Debug)]
        struct Test {
            int: u32,
            seq: Vec<String>,
        }

        let j = r#"int=1,seq={a,b}"#;
        let expected = Test {
            int: 1,
            seq: vec!["a".to_owned(), "b".to_owned()],
        };
        assert_eq!(expected, from_str(j).unwrap());
    }

    #[test]
    fn test_enum() {
        #[derive(Deserialize, PartialEq, Debug)]
        enum E {
            Unit,
            Newtype(u32),
            Tuple(u32, f32),
            Struct { a: u32 },
        }

        let j = r#"Unit"#;
        let expected = E::Unit;
        assert_eq!(expected, from_str(j).unwrap());

        let j = r#"Newtype{1}"#;
        let expected = E::Newtype(1);
        assert_eq!(expected, from_str(j).unwrap());

        let j = r#"Tuple{1,2}"#;
        let expected = E::Tuple(1, 2f32);
        assert_eq!(expected, from_str(j).unwrap());

        let j = r#"Struct{a=1}"#;
        let expected = E::Struct { a: 1 };
        assert_eq!(expected, from_str(j).unwrap());
    }
}
