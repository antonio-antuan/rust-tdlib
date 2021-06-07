use crate::errors::*;
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
    #[serde(rename(serialize = "getOption", deserialize = "getOption"))]
    GetOption(GetOption),
    /// Represents a boolean option
    #[serde(rename(serialize = "optionValueBoolean", deserialize = "optionValueBoolean"))]
    Boolean(OptionValueBoolean),
    /// Represents an unknown option or an option which has a default value
    #[serde(rename(serialize = "optionValueEmpty", deserialize = "optionValueEmpty"))]
    Empty(OptionValueEmpty),
    /// Represents an integer option
    #[serde(rename(serialize = "optionValueInteger", deserialize = "optionValueInteger"))]
    Integer(OptionValueInteger),
    /// Represents a string option
    #[serde(rename(serialize = "optionValueString", deserialize = "optionValueString"))]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDOptionValueBooleanBuilder {
        let mut inner = OptionValueBoolean::default();
        inner.extra = Some(Uuid::new_v4().to_string());

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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDOptionValueEmptyBuilder {
        let mut inner = OptionValueEmpty::default();
        inner.extra = Some(Uuid::new_v4().to_string());

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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDOptionValueIntegerBuilder {
        let mut inner = OptionValueInteger::default();
        inner.extra = Some(Uuid::new_v4().to_string());

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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The value of the option
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDOptionValueStringBuilder {
        let mut inner = OptionValueString::default();
        inner.extra = Some(Uuid::new_v4().to_string());

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
