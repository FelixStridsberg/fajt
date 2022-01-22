use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Formatter;

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn empty() -> Self {
        Self::new(0, 0)
    }

    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn translate(&self, offset: isize) -> Self {
        Span {
            start: (self.start as isize + offset) as usize,
            end: (self.end as isize + offset) as usize,
        }
    }
}

impl Serialize for Span {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}:{}", self.start, self.end))
    }
}

impl<'de> Deserialize<'de> for Span {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SpanVisitor;

        impl<'de> Visitor<'de> for SpanVisitor {
            type Value = Span;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("struct Span")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let mut parts = v.split(':');
                if let (Some(start), Some(end)) = (parts.next(), parts.next()) {
                    Ok(Span {
                        start: start.parse().unwrap(),
                        end: end.parse().unwrap(),
                    })
                } else {
                    Err(serde::de::Error::custom(format!(
                        "{} is not a valid span.",
                        v
                    )))
                }
            }
        }

        deserializer.deserialize_str(SpanVisitor)
    }
}

impl From<(usize, usize)> for Span {
    fn from((start, end): (usize, usize)) -> Self {
        Span { start, end }
    }
}
