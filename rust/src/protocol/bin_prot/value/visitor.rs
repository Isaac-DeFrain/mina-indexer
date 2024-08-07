// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "loose_deserialization")]
use crate::protocol::bin_prot::loose_deserializer::EnumData;
use crate::protocol::bin_prot::value::Value;
#[cfg(feature = "loose_deserialization")]
use serde::de::{EnumAccess, VariantAccess};
use serde::{
    de::{MapAccess, SeqAccess, Visitor},
    Deserialize,
};

pub struct ValueVisitor;

impl<'de> Visitor<'de> for ValueVisitor {
    type Value = Value;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("any valid OCaml value")
    }

    #[inline]
    fn visit_bool<E>(self, value: bool) -> Result<Value, E> {
        Ok(Value::Bool(value))
    }

    #[inline]
    fn visit_i64<E>(self, value: i64) -> Result<Value, E> {
        Ok(Value::Int(value))
    }

    #[inline]
    fn visit_f64<E>(self, value: f64) -> Result<Value, E> {
        Ok(Value::Float(value))
    }

    #[inline]
    fn visit_char<E>(self, value: char) -> Result<Value, E> {
        Ok(Value::Char(value as u8))
    }

    #[inline]
    fn visit_str<E>(self, value: &str) -> Result<Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_string(String::from(value))
    }

    #[inline]
    fn visit_bytes<E>(self, value: &[u8]) -> Result<Value, E> {
        // Represent bytes as a list of chars
        // Chars are always 1 byte in BinProt so this fits
        let bytes = value.iter().map(|x| Value::Char(*x)).collect();
        Ok(Value::List(bytes))
    }

    #[inline]
    fn visit_none<E>(self) -> Result<Value, E> {
        Ok(Value::Option(None))
    }

    #[inline]
    fn visit_some<D>(self, deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Value::Option(Some(Box::new(Deserialize::deserialize(
            deserializer,
        )?))))
    }

    #[inline]
    fn visit_unit<E>(self) -> Result<Value, E> {
        Ok(Value::Unit)
    }

    #[inline]
    fn visit_seq<V>(self, mut visitor: V) -> Result<Value, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let mut vec = Vec::new();
        while let Some(elem) = visitor.next_element()? {
            vec.push(elem);
        }

        if visitor.size_hint().is_some() {
            Ok(Value::List(vec))
        } else {
            Ok(Value::Tuple(vec))
        }
    }

    fn visit_map<V>(self, mut visitor: V) -> Result<Value, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut values = Vec::new();
        while let Some((k, v)) = visitor.next_entry()? {
            values.push((k, v));
        }
        Ok(Value::Record(values))
    }

    #[cfg(feature = "loose_deserialization")]
    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: EnumAccess<'de>,
    {
        let (payload, variant_access) = data.variant::<EnumData>()?;

        // payload must encode the index and name in a deserializer
        // the variant access can be used to retrieve the correct content based on this

        match payload {
            EnumData::Sum { index, name, len } => {
                let body = variant_access.tuple_variant(len, self)?;
                Ok(Value::Sum {
                    name,
                    index,
                    value: Box::new(body),
                })
            }
            EnumData::Polyvar {
                index: _,
                tag,
                name,
                len,
            } => {
                let body = variant_access.tuple_variant(len, self)?;
                Ok(Value::Polyvar {
                    name,
                    tag,
                    value: Box::new(body),
                })
            }
        }
    }
}
