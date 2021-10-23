// Yoinked from serde and modified: https://github.com/serde-rs/serde

use core::fmt;
use std::{fmt::Display, marker::PhantomData, str::FromStr};
use serde::Deserializer;
use serde::de::{Visitor, Error, Unexpected};

/// Deserialize an `Option<T>` from a string or bool using `FromStr`
pub fn deserialize<'de, D, S>(deserializer: D) -> Result<Option<S>, D::Error>
where
    D: Deserializer<'de>,
    S: FromStr,
    S::Err: Display,
{
    struct OptionStringFalseNone<S>(PhantomData<S>);
    impl<'de, S> Visitor<'de> for OptionStringFalseNone<S>
    where
        S: FromStr,
        S::Err: Display,
    {
        type Value = Option<S>;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("string or false")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            match value {
                v => S::from_str(v).map(Some).map_err(Error::custom),
            }
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: Error,
        {
            match &*value {
                v => S::from_str(v).map(Some).map_err(Error::custom),
            }
        }

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
        where
            E: Error,
        {
            match value {
                false => Ok(None),
                true => Err(Error::invalid_value(Unexpected::Bool(true), &"string or false"))
            }
        }

        // handles the `null` case
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(None)
        }
    }

    deserializer.deserialize_any(OptionStringFalseNone(PhantomData))
}
