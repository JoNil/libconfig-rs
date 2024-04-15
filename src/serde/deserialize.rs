use super::error::Error;
use crate::Value;
use serde::{
    de::{self, DeserializeSeed, EnumAccess, MapAccess, SeqAccess, VariantAccess, Visitor},
    Deserialize,
};
use std::{collections::VecDeque, marker::PhantomData, str::FromStr};

// Example
// https://serde.rs/data-format.html
// https://github.com/serde-rs/example-format

#[derive(Clone, Debug)]
enum Token {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Count(usize),
}

impl Token {
    fn into_bool(self) -> Result<bool, Token> {
        match self {
            Token::Bool(v) => Ok(v),
            _ => Err(self),
        }
    }

    fn into_int(self) -> Result<i64, Token> {
        match self {
            Token::Int(v) => Ok(v),
            _ => Err(self),
        }
    }

    fn into_float(self) -> Result<f64, Token> {
        match self {
            Token::Float(v) => Ok(v),
            _ => Err(self),
        }
    }

    fn into_string(self) -> Result<String, Token> {
        match self {
            Token::String(v) => Ok(v),
            _ => Err(self),
        }
    }

    fn into_count(self) -> Result<usize, Token> {
        match self {
            Token::Count(v) => Ok(v),
            _ => Err(self),
        }
    }
}

fn flatten(res: &mut VecDeque<Token>, value: Value) {
    match value {
        Value::Bool(b) => {
            res.push_back(Token::Bool(b));
        }
        Value::Int(i) => {
            res.push_back(Token::Int(i));
        }
        Value::Float(f) => {
            res.push_back(Token::Float(f));
        }
        Value::String(s) => {
            res.push_back(Token::String(s));
        }
        Value::Array(a, _) => {
            res.push_back(Token::Count(a.len()));
            for v in a {
                flatten(res, v)
            }
        }
        Value::Object(o) => {
            res.push_back(Token::Count(o.len()));
            for (k, v) in o {
                res.push_back(Token::String(k));
                flatten(res, v)
            }
        }
    }
}

pub struct Deserializer<'de> {
    tokens: VecDeque<Token>,
    phantom: PhantomData<&'de str>,
}

pub fn from_str<'a, T>(s: &'a str) -> Result<T, Error>
where
    T: Deserialize<'a>,
{
    let value = crate::Value::from_str(s).map_err(|e| Error::Message(format!("{e:?}")))?;

    let mut tokens = VecDeque::new();

    flatten(&mut tokens, value);

    let mut deserializer = Deserializer::<'a> {
        tokens,
        phantom: PhantomData,
    };

    T::deserialize(&mut deserializer)
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        panic!("Format needs type hints!");
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let token = self
            .tokens
            .pop_front()
            .ok_or_else(|| Error::Message("Reached end of input!".into()))?;

        visitor.visit_bool(
            token
                .into_bool()
                .map_err(|t| Error::Message(format!("{t:?} is not a bool")))?,
        )
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let token = self
            .tokens
            .pop_front()
            .ok_or_else(|| Error::Message("Reached end of input!".into()))?;

        visitor.visit_i8(
            token
                .into_int()
                .map_err(|t| Error::Message(format!("{t:?} is not a integer")))? as i8,
        )
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let token = self
            .tokens
            .pop_front()
            .ok_or_else(|| Error::Message("Reached end of input!".into()))?;

        visitor.visit_i16(
            token
                .into_int()
                .map_err(|t| Error::Message(format!("{t:?} is not a integer")))? as i16,
        )
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let token = self
            .tokens
            .pop_front()
            .ok_or_else(|| Error::Message("Reached end of input!".into()))?;

        visitor.visit_i32(
            token
                .into_int()
                .map_err(|t| Error::Message(format!("{t:?} is not a integer")))? as i32,
        )
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let token = self
            .tokens
            .pop_front()
            .ok_or_else(|| Error::Message("Reached end of input!".into()))?;

        visitor.visit_i64(
            token
                .into_int()
                .map_err(|t| Error::Message(format!("{t:?} is not a integer")))?,
        )
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let token = self
            .tokens
            .pop_front()
            .ok_or_else(|| Error::Message("Reached end of input!".into()))?;

        visitor.visit_u8(
            token
                .into_int()
                .map_err(|t| Error::Message(format!("{t:?} is not a integer")))? as u8,
        )
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let token = self
            .tokens
            .pop_front()
            .ok_or_else(|| Error::Message("Reached end of input!".into()))?;

        visitor.visit_u16(
            token
                .into_int()
                .map_err(|t| Error::Message(format!("{t:?} is not a integer")))? as u16,
        )
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let token = self
            .tokens
            .pop_front()
            .ok_or_else(|| Error::Message("Reached end of input!".into()))?;

        visitor.visit_u32(
            token
                .into_int()
                .map_err(|t| Error::Message(format!("{t:?} is not a integer")))? as u32,
        )
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let token = self
            .tokens
            .pop_front()
            .ok_or_else(|| Error::Message("Reached end of input!".into()))?;

        visitor.visit_u64(
            token
                .into_int()
                .map_err(|t| Error::Message(format!("{t:?} is not a integer")))? as u64,
        )
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let token = self
            .tokens
            .pop_front()
            .ok_or_else(|| Error::Message("Reached end of input!".into()))?;

        visitor.visit_f32(
            token
                .into_float()
                .map_err(|t| Error::Message(format!("{t:?} is not a float")))? as f32,
        )
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let token = self
            .tokens
            .pop_front()
            .ok_or_else(|| Error::Message("Reached end of input!".into()))?;

        visitor.visit_f64(
            token
                .into_float()
                .map_err(|t| Error::Message(format!("{t:?} is not a float")))?,
        )
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let token = self
            .tokens
            .pop_front()
            .ok_or_else(|| Error::Message("Reached end of input!".into()))?;

        visitor.visit_char(
            token
                .into_string()
                .map_err(|t| Error::Message(format!("{t:?} is not a char")))?
                .chars()
                .next()
                .ok_or_else(|| Error::Message("String is empty".into()))?,
        )
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let token = self
            .tokens
            .pop_front()
            .ok_or_else(|| Error::Message("Reached end of input!".into()))?;

        visitor.visit_str(
            token
                .into_string()
                .map_err(|t| Error::Message(format!("{t:?} is not a str")))?
                .as_str(),
        )
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let token = self
            .tokens
            .pop_front()
            .ok_or_else(|| Error::Message("Reached end of input!".into()))?;

        visitor.visit_string(
            token
                .into_string()
                .map_err(|t| Error::Message(format!("{t:?} is not a str")))?,
        )
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!("")
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!("")
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let len = self
            .tokens
            .pop_front()
            .ok_or_else(|| Error::Message("Reached end of input!".into()))?
            .into_count()
            .map_err(|t| Error::Message(format!("{t:?} is not a count")))?;

        if len == 0 {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let len = self
            .tokens
            .pop_front()
            .ok_or_else(|| Error::Message("Reached end of input!".into()))?
            .into_count()
            .map_err(|t| Error::Message(format!("{t:?} is not a count")))?;

        if len == 0 {
            visitor.visit_unit()
        } else {
            Err(Error::Message("Expected empty list".into()))
        }
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let len = self
            .tokens
            .pop_front()
            .ok_or_else(|| Error::Message("Reached end of input!".into()))?
            .into_count()
            .map_err(|t| Error::Message(format!("{t:?} is not a count")))?;

        if len != 1 {
            return Err(Error::Message(format!(
                "Expected 1 field in struct got {len}"
            )));
        }

        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let count = self
            .tokens
            .pop_front()
            .ok_or_else(|| Error::Message("Reached end of input!".into()))?
            .into_count()
            .map_err(|t| Error::Message(format!("Expected field count, got {t:?}")))?;

        visitor.visit_seq(SeqAccessor {
            de: self,
            remaining: count,
        })
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let count = self
            .tokens
            .pop_front()
            .ok_or_else(|| Error::Message("Reached end of input!".into()))?
            .into_count()
            .map_err(|t| Error::Message(format!("Expected field count, got {t:?}")))?;

        visitor.visit_map(MapAccessor {
            de: self,
            remaining: count,
        })
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let count = self
            .tokens
            .pop_front()
            .ok_or_else(|| Error::Message("Reached end of input!".into()))?
            .into_count()
            .map_err(|t| Error::Message(format!("Expected field count, got {t:?}")))?;

        visitor.visit_map(StructAccessor {
            de: self,
            remaining: count,
        })
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Some(Token::Count(_)) = self.tokens.front() {
            self.tokens.pop_front();
        };

        visitor.visit_enum(Enum::new(self))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let token = self
            .tokens
            .pop_front()
            .ok_or_else(|| Error::Message("Reached end of input!".into()))?
            .into_string()
            .map_err(|t| Error::Message(format!("{t:?} is not an identifier")))?;

        visitor.visit_str(token.as_str())
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

struct SeqAccessor<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    remaining: usize,
}

impl<'de, 'a> SeqAccess<'de> for SeqAccessor<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if self.remaining > 0 {
            self.remaining -= 1;
            seed.deserialize(&mut *self.de).map(Some)
        } else {
            Ok(None)
        }
    }
}

struct StructAccessor<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    remaining: usize,
}

impl<'de, 'a> MapAccess<'de> for StructAccessor<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        if self.remaining > 0 {
            self.remaining -= 1;
            seed.deserialize(&mut *self.de).map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de)
    }
}

struct MapAccessor<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    remaining: usize,
}

impl<'de, 'a> MapAccess<'de> for MapAccessor<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        if self.remaining > 0 {
            self.remaining -= 1;
            let count = self
                .de
                .tokens
                .pop_front()
                .ok_or_else(|| Error::Message("Reached end of input!".into()))?
                .into_count()
                .map_err(|t| Error::Message(format!("{t:?} is not a count")))?;

            if count != 2 {
                return Err(Error::Message(
                    "Map does not contain list of key value pairs".into(),
                ));
            }

            seed.deserialize(&mut *self.de).map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de)
    }
}

struct Enum<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> Enum<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        Enum { de }
    }
}

impl<'de, 'a> EnumAccess<'de> for Enum<'a, 'de> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        Ok((seed.deserialize(&mut *self.de)?, self))
    }
}

impl<'de, 'a> VariantAccess<'de> for Enum<'a, 'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        self.de.tokens.pop_front();
        seed.deserialize(self.de)
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        de::Deserializer::deserialize_seq(self.de, visitor)
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        de::Deserializer::deserialize_struct(self.de, "", &[], visitor)
    }
}
