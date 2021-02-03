use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Represents the value of an option
pub trait TDOptionValue: Debug + RObject {}

/// Represents the value of an option
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum OptionValue {
    #[doc(hidden)]
    _Default(()),
    /// Returns the value of an option by its name. (Check the list of available options on https://core.telegram.org/tdlib/options.) Can be called before authorization
    GetOption(GetOption),
    /// Represents a boolean option
    Boolean(OptionValueBoolean),
    /// Represents an unknown option or an option which has a default value
    Empty(OptionValueEmpty),
    /// Represents an integer option
    Integer(OptionValueInteger),
    /// Represents a string option
    String(OptionValueString),
}

impl Default for OptionValue {
    fn default() -> Self {
        OptionValue::_Default(())
    }
}

impl<'de> Deserialize<'de> for OptionValue {
    fn deserialize<D>(deserializer: D) -> Result<OptionValue, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        rtd_enum_deserialize!(
          OptionValue,
          (getOption, GetOption);
          (optionValueBoolean, Boolean);
          (optionValueEmpty, Empty);
          (optionValueInteger, Integer);
          (optionValueString, String);

        )(deserializer)
    }
}

impl RObject for OptionValue {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        match self {
            OptionValue::GetOption(t) => t.td_name(),
            OptionValue::Boolean(t) => t.td_name(),
            OptionValue::Empty(t) => t.td_name(),
            OptionValue::Integer(t) => t.td_name(),
            OptionValue::String(t) => t.td_name(),

            _ => "-1",
        }
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        match self {
            OptionValue::GetOption(t) => t.extra(),
            OptionValue::Boolean(t) => t.extra(),
            OptionValue::Empty(t) => t.extra(),
            OptionValue::Integer(t) => t.extra(),
            OptionValue::String(t) => t.extra(),

            _ => None,
        }
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            OptionValue::GetOption(t) => t.client_id(),
            OptionValue::Boolean(t) => t.client_id(),
            OptionValue::Empty(t) => t.client_id(),
            OptionValue::Integer(t) => t.client_id(),
            OptionValue::String(t) => t.client_id(),

            _ => None,
        }
    }
}

impl OptionValue {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, OptionValue::_Default(_))
    }
}

impl AsRef<OptionValue> for OptionValue {
    fn as_ref(&self) -> &OptionValue {
        self
    }
}

/// Represents a boolean option
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OptionValueBoolean {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The value of the option
    value: bool,
}

impl RObject for OptionValueBoolean {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "optionValueBoolean"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDOptionValue for OptionValueBoolean {}

impl OptionValueBoolean {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDOptionValueBooleanBuilder {
        let mut inner = OptionValueBoolean::default();
        inner.td_name = "optionValueBoolean".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDOptionValueBooleanBuilder { inner }
    }

    pub fn value(&self) -> bool {
        self.value
    }
}

#[doc(hidden)]
pub struct RTDOptionValueBooleanBuilder {
    inner: OptionValueBoolean,
}

impl RTDOptionValueBooleanBuilder {
    pub fn build(&self) -> OptionValueBoolean {
        self.inner.clone()
    }

    pub fn value(&mut self, value: bool) -> &mut Self {
        self.inner.value = value;
        self
    }
}

impl AsRef<OptionValueBoolean> for OptionValueBoolean {
    fn as_ref(&self) -> &OptionValueBoolean {
        self
    }
}

impl AsRef<OptionValueBoolean> for RTDOptionValueBooleanBuilder {
    fn as_ref(&self) -> &OptionValueBoolean {
        &self.inner
    }
}

/// Represents an unknown option or an option which has a default value
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OptionValueEmpty {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for OptionValueEmpty {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "optionValueEmpty"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDOptionValue for OptionValueEmpty {}

impl OptionValueEmpty {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDOptionValueEmptyBuilder {
        let mut inner = OptionValueEmpty::default();
        inner.td_name = "optionValueEmpty".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDOptionValueEmptyBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDOptionValueEmptyBuilder {
    inner: OptionValueEmpty,
}

impl RTDOptionValueEmptyBuilder {
    pub fn build(&self) -> OptionValueEmpty {
        self.inner.clone()
    }
}

impl AsRef<OptionValueEmpty> for OptionValueEmpty {
    fn as_ref(&self) -> &OptionValueEmpty {
        self
    }
}

impl AsRef<OptionValueEmpty> for RTDOptionValueEmptyBuilder {
    fn as_ref(&self) -> &OptionValueEmpty {
        &self.inner
    }
}

/// Represents an integer option
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OptionValueInteger {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The value of the option
    #[serde(deserialize_with = "super::_common::number_from_string")]
    value: i64,
}

impl RObject for OptionValueInteger {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "optionValueInteger"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDOptionValue for OptionValueInteger {}

impl OptionValueInteger {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDOptionValueIntegerBuilder {
        let mut inner = OptionValueInteger::default();
        inner.td_name = "optionValueInteger".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDOptionValueIntegerBuilder { inner }
    }

    pub fn value(&self) -> i64 {
        self.value
    }
}

#[doc(hidden)]
pub struct RTDOptionValueIntegerBuilder {
    inner: OptionValueInteger,
}

impl RTDOptionValueIntegerBuilder {
    pub fn build(&self) -> OptionValueInteger {
        self.inner.clone()
    }

    pub fn value(&mut self, value: i64) -> &mut Self {
        self.inner.value = value;
        self
    }
}

impl AsRef<OptionValueInteger> for OptionValueInteger {
    fn as_ref(&self) -> &OptionValueInteger {
        self
    }
}

impl AsRef<OptionValueInteger> for RTDOptionValueIntegerBuilder {
    fn as_ref(&self) -> &OptionValueInteger {
        &self.inner
    }
}

/// Represents a string option
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OptionValueString {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The value of the option
    value: String,
}

impl RObject for OptionValueString {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "optionValueString"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDOptionValue for OptionValueString {}

impl OptionValueString {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDOptionValueStringBuilder {
        let mut inner = OptionValueString::default();
        inner.td_name = "optionValueString".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDOptionValueStringBuilder { inner }
    }

    pub fn value(&self) -> &String {
        &self.value
    }
}

#[doc(hidden)]
pub struct RTDOptionValueStringBuilder {
    inner: OptionValueString,
}

impl RTDOptionValueStringBuilder {
    pub fn build(&self) -> OptionValueString {
        self.inner.clone()
    }

    pub fn value<T: AsRef<str>>(&mut self, value: T) -> &mut Self {
        self.inner.value = value.as_ref().to_string();
        self
    }
}

impl AsRef<OptionValueString> for OptionValueString {
    fn as_ref(&self) -> &OptionValueString {
        self
    }
}

impl AsRef<OptionValueString> for RTDOptionValueStringBuilder {
    fn as_ref(&self) -> &OptionValueString {
        &self.inner
    }
}
