use std::{
    fmt::{Display, Formatter as FmtFormatter, Result as FmtResult},
    str::FromStr,
};

use super::{Header, HeaderName, HeaderValue};
use crate::BoxError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MyHeader {
    Value1,
    Value2
}
impl Header for MyHeader {
    fn name() -> HeaderName {
        HeaderName::new_from_ascii_str("MY-HEADER")
    }

    fn parse(s: &str) -> Result<Self, BoxError> {
        Ok(s.parse()?)
    }

    fn display(&self) -> HeaderValue {
        let val = self.to_string();
        HeaderValue::dangerous_new_pre_encoded(Self::name(), val.clone(), val)
    }
}

impl Display for MyHeader {
    fn fmt(&self, f: &mut FmtFormatter<'_>) -> FmtResult {
        f.write_str(match *self {
            Self::Value1 => "value1",
            Self::Value2 => "value2",
        })
    }
}

impl FromStr for MyHeader {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "value1" => Ok(Self::Value1),
            "value2" => Ok(Self::Value2),
            _ => Err(s.into()),
        }
    }
}

impl Default for MyHeader {
    fn default() -> Self {
        MyHeader::Value1
    }
}
