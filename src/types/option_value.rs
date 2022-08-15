use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Represents the value of an option
pub trait TDOptionValue: Debug + RObject {}

/// Represents the value of an option
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum OptionValue {
    #[doc(hidden)]
    _Default,
    /// Returns the value of an option by its name. (Check the list of available options on https://core.telegram.org/tdlib/options.) Can be called before authorization
    #[serde(rename = "getOption")]
    GetOption(GetOption),
    /// Represents a boolean option
    #[serde(rename = "optionValueBoolean")]
    Boolean(OptionValueBoolean),
    /// Represents an unknown option or an option which has a default value
    #[serde(rename = "optionValueEmpty")]
    Empty(OptionValueEmpty),
    /// Represents an integer option
    #[serde(rename = "optionValueInteger")]
    Integer(OptionValueInteger),
    /// Represents a string option
    #[serde(rename = "optionValueString")]
    String(OptionValueString),
}

impl Default for OptionValue {
    fn default() -> Self {
        OptionValue::_Default
    }
}

impl RObject for OptionValue {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            OptionValue::GetOption(t) => t.extra(),
            OptionValue::Boolean(t) => t.extra(),
            OptionValue::Empty(t) => t.extra(),
            OptionValue::Integer(t) => t.extra(),
            OptionValue::String(t) => t.extra(),

            _ => None,
        }
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, OptionValue::_Default)
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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The value of the option

    #[serde(default)]
    value: bool,
}

impl RObject for OptionValueBoolean {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDOptionValue for OptionValueBoolean {}

impl OptionValueBoolean {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> OptionValueBooleanBuilder {
        let mut inner = OptionValueBoolean::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        OptionValueBooleanBuilder { inner }
    }

    pub fn value(&self) -> bool {
        self.value
    }
}

#[doc(hidden)]
pub struct OptionValueBooleanBuilder {
    inner: OptionValueBoolean,
}

#[deprecated]
pub type RTDOptionValueBooleanBuilder = OptionValueBooleanBuilder;

impl OptionValueBooleanBuilder {
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

impl AsRef<OptionValueBoolean> for OptionValueBooleanBuilder {
    fn as_ref(&self) -> &OptionValueBoolean {
        &self.inner
    }
}

/// Represents an unknown option or an option which has a default value
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OptionValueEmpty {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for OptionValueEmpty {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDOptionValue for OptionValueEmpty {}

impl OptionValueEmpty {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> OptionValueEmptyBuilder {
        let mut inner = OptionValueEmpty::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        OptionValueEmptyBuilder { inner }
    }
}

#[doc(hidden)]
pub struct OptionValueEmptyBuilder {
    inner: OptionValueEmpty,
}

#[deprecated]
pub type RTDOptionValueEmptyBuilder = OptionValueEmptyBuilder;

impl OptionValueEmptyBuilder {
    pub fn build(&self) -> OptionValueEmpty {
        self.inner.clone()
    }
}

impl AsRef<OptionValueEmpty> for OptionValueEmpty {
    fn as_ref(&self) -> &OptionValueEmpty {
        self
    }
}

impl AsRef<OptionValueEmpty> for OptionValueEmptyBuilder {
    fn as_ref(&self) -> &OptionValueEmpty {
        &self.inner
    }
}

/// Represents an integer option
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OptionValueInteger {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The value of the option

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    value: i64,
}

impl RObject for OptionValueInteger {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDOptionValue for OptionValueInteger {}

impl OptionValueInteger {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> OptionValueIntegerBuilder {
        let mut inner = OptionValueInteger::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        OptionValueIntegerBuilder { inner }
    }

    pub fn value(&self) -> i64 {
        self.value
    }
}

#[doc(hidden)]
pub struct OptionValueIntegerBuilder {
    inner: OptionValueInteger,
}

#[deprecated]
pub type RTDOptionValueIntegerBuilder = OptionValueIntegerBuilder;

impl OptionValueIntegerBuilder {
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

impl AsRef<OptionValueInteger> for OptionValueIntegerBuilder {
    fn as_ref(&self) -> &OptionValueInteger {
        &self.inner
    }
}

/// Represents a string option
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OptionValueString {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The value of the option

    #[serde(default)]
    value: String,
}

impl RObject for OptionValueString {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDOptionValue for OptionValueString {}

impl OptionValueString {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> OptionValueStringBuilder {
        let mut inner = OptionValueString::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        OptionValueStringBuilder { inner }
    }

    pub fn value(&self) -> &String {
        &self.value
    }
}

#[doc(hidden)]
pub struct OptionValueStringBuilder {
    inner: OptionValueString,
}

#[deprecated]
pub type RTDOptionValueStringBuilder = OptionValueStringBuilder;

impl OptionValueStringBuilder {
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

impl AsRef<OptionValueString> for OptionValueStringBuilder {
    fn as_ref(&self) -> &OptionValueString {
        &self.inner
    }
}
