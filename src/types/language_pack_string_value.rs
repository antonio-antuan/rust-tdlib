use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Represents the value of a string in a language pack
pub trait TDLanguagePackStringValue: Debug + RObject {}

/// Represents the value of a string in a language pack
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum LanguagePackStringValue {
    #[doc(hidden)]
    _Default,
    /// Returns a string stored in the local database from the specified localization target and language pack by its key. Returns a 404 error if the string is not found. Can be called synchronously
    #[serde(rename = "getLanguagePackString")]
    GetLanguagePackString(GetLanguagePackString),
    /// A deleted language pack string, the value must be taken from the built-in English language pack
    #[serde(rename = "languagePackStringValueDeleted")]
    Deleted(LanguagePackStringValueDeleted),
    /// An ordinary language pack string
    #[serde(rename = "languagePackStringValueOrdinary")]
    Ordinary(LanguagePackStringValueOrdinary),
    /// A language pack string which has different forms based on the number of some object it mentions. See https://www.unicode.org/cldr/charts/latest/supplemental/language_plural_rules.html for more info
    #[serde(rename = "languagePackStringValuePluralized")]
    Pluralized(LanguagePackStringValuePluralized),
}

impl Default for LanguagePackStringValue {
    fn default() -> Self {
        LanguagePackStringValue::_Default
    }
}

impl RObject for LanguagePackStringValue {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            LanguagePackStringValue::GetLanguagePackString(t) => t.extra(),
            LanguagePackStringValue::Deleted(t) => t.extra(),
            LanguagePackStringValue::Ordinary(t) => t.extra(),
            LanguagePackStringValue::Pluralized(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            LanguagePackStringValue::GetLanguagePackString(t) => t.client_id(),
            LanguagePackStringValue::Deleted(t) => t.client_id(),
            LanguagePackStringValue::Ordinary(t) => t.client_id(),
            LanguagePackStringValue::Pluralized(t) => t.client_id(),

            _ => None,
        }
    }
}

impl LanguagePackStringValue {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, LanguagePackStringValue::_Default)
    }
}

impl AsRef<LanguagePackStringValue> for LanguagePackStringValue {
    fn as_ref(&self) -> &LanguagePackStringValue {
        self
    }
}

/// A deleted language pack string, the value must be taken from the built-in English language pack
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LanguagePackStringValueDeleted {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for LanguagePackStringValueDeleted {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDLanguagePackStringValue for LanguagePackStringValueDeleted {}

impl LanguagePackStringValueDeleted {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> LanguagePackStringValueDeletedBuilder {
        let mut inner = LanguagePackStringValueDeleted::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        LanguagePackStringValueDeletedBuilder { inner }
    }
}

#[doc(hidden)]
pub struct LanguagePackStringValueDeletedBuilder {
    inner: LanguagePackStringValueDeleted,
}

#[deprecated]
pub type RTDLanguagePackStringValueDeletedBuilder = LanguagePackStringValueDeletedBuilder;

impl LanguagePackStringValueDeletedBuilder {
    pub fn build(&self) -> LanguagePackStringValueDeleted {
        self.inner.clone()
    }
}

impl AsRef<LanguagePackStringValueDeleted> for LanguagePackStringValueDeleted {
    fn as_ref(&self) -> &LanguagePackStringValueDeleted {
        self
    }
}

impl AsRef<LanguagePackStringValueDeleted> for LanguagePackStringValueDeletedBuilder {
    fn as_ref(&self) -> &LanguagePackStringValueDeleted {
        &self.inner
    }
}

/// An ordinary language pack string
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LanguagePackStringValueOrdinary {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// String value

    #[serde(default)]
    value: String,
}

impl RObject for LanguagePackStringValueOrdinary {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDLanguagePackStringValue for LanguagePackStringValueOrdinary {}

impl LanguagePackStringValueOrdinary {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> LanguagePackStringValueOrdinaryBuilder {
        let mut inner = LanguagePackStringValueOrdinary::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        LanguagePackStringValueOrdinaryBuilder { inner }
    }

    pub fn value(&self) -> &String {
        &self.value
    }
}

#[doc(hidden)]
pub struct LanguagePackStringValueOrdinaryBuilder {
    inner: LanguagePackStringValueOrdinary,
}

#[deprecated]
pub type RTDLanguagePackStringValueOrdinaryBuilder = LanguagePackStringValueOrdinaryBuilder;

impl LanguagePackStringValueOrdinaryBuilder {
    pub fn build(&self) -> LanguagePackStringValueOrdinary {
        self.inner.clone()
    }

    pub fn value<T: AsRef<str>>(&mut self, value: T) -> &mut Self {
        self.inner.value = value.as_ref().to_string();
        self
    }
}

impl AsRef<LanguagePackStringValueOrdinary> for LanguagePackStringValueOrdinary {
    fn as_ref(&self) -> &LanguagePackStringValueOrdinary {
        self
    }
}

impl AsRef<LanguagePackStringValueOrdinary> for LanguagePackStringValueOrdinaryBuilder {
    fn as_ref(&self) -> &LanguagePackStringValueOrdinary {
        &self.inner
    }
}

/// A language pack string which has different forms based on the number of some object it mentions. See https://www.unicode.org/cldr/charts/latest/supplemental/language_plural_rules.html for more info
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LanguagePackStringValuePluralized {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Value for zero objects

    #[serde(default)]
    zero_value: String,
    /// Value for one object

    #[serde(default)]
    one_value: String,
    /// Value for two objects

    #[serde(default)]
    two_value: String,
    /// Value for few objects

    #[serde(default)]
    few_value: String,
    /// Value for many objects

    #[serde(default)]
    many_value: String,
    /// Default value

    #[serde(default)]
    other_value: String,
}

impl RObject for LanguagePackStringValuePluralized {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDLanguagePackStringValue for LanguagePackStringValuePluralized {}

impl LanguagePackStringValuePluralized {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> LanguagePackStringValuePluralizedBuilder {
        let mut inner = LanguagePackStringValuePluralized::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        LanguagePackStringValuePluralizedBuilder { inner }
    }

    pub fn zero_value(&self) -> &String {
        &self.zero_value
    }

    pub fn one_value(&self) -> &String {
        &self.one_value
    }

    pub fn two_value(&self) -> &String {
        &self.two_value
    }

    pub fn few_value(&self) -> &String {
        &self.few_value
    }

    pub fn many_value(&self) -> &String {
        &self.many_value
    }

    pub fn other_value(&self) -> &String {
        &self.other_value
    }
}

#[doc(hidden)]
pub struct LanguagePackStringValuePluralizedBuilder {
    inner: LanguagePackStringValuePluralized,
}

#[deprecated]
pub type RTDLanguagePackStringValuePluralizedBuilder = LanguagePackStringValuePluralizedBuilder;

impl LanguagePackStringValuePluralizedBuilder {
    pub fn build(&self) -> LanguagePackStringValuePluralized {
        self.inner.clone()
    }

    pub fn zero_value<T: AsRef<str>>(&mut self, zero_value: T) -> &mut Self {
        self.inner.zero_value = zero_value.as_ref().to_string();
        self
    }

    pub fn one_value<T: AsRef<str>>(&mut self, one_value: T) -> &mut Self {
        self.inner.one_value = one_value.as_ref().to_string();
        self
    }

    pub fn two_value<T: AsRef<str>>(&mut self, two_value: T) -> &mut Self {
        self.inner.two_value = two_value.as_ref().to_string();
        self
    }

    pub fn few_value<T: AsRef<str>>(&mut self, few_value: T) -> &mut Self {
        self.inner.few_value = few_value.as_ref().to_string();
        self
    }

    pub fn many_value<T: AsRef<str>>(&mut self, many_value: T) -> &mut Self {
        self.inner.many_value = many_value.as_ref().to_string();
        self
    }

    pub fn other_value<T: AsRef<str>>(&mut self, other_value: T) -> &mut Self {
        self.inner.other_value = other_value.as_ref().to_string();
        self
    }
}

impl AsRef<LanguagePackStringValuePluralized> for LanguagePackStringValuePluralized {
    fn as_ref(&self) -> &LanguagePackStringValuePluralized {
        self
    }
}

impl AsRef<LanguagePackStringValuePluralized> for LanguagePackStringValuePluralizedBuilder {
    fn as_ref(&self) -> &LanguagePackStringValuePluralized {
        &self.inner
    }
}
