use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents one language pack string
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LanguagePackString {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// String key
    key: String,
    /// String value

    #[serde(skip_serializing_if = "LanguagePackStringValue::_is_default")]
    value: LanguagePackStringValue,
}

impl RObject for LanguagePackString {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl LanguagePackString {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDLanguagePackStringBuilder {
        let mut inner = LanguagePackString::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDLanguagePackStringBuilder { inner }
    }

    pub fn key(&self) -> &String {
        &self.key
    }

    pub fn value(&self) -> &LanguagePackStringValue {
        &self.value
    }
}

#[doc(hidden)]
pub struct RTDLanguagePackStringBuilder {
    inner: LanguagePackString,
}

impl RTDLanguagePackStringBuilder {
    pub fn build(&self) -> LanguagePackString {
        self.inner.clone()
    }

    pub fn key<T: AsRef<str>>(&mut self, key: T) -> &mut Self {
        self.inner.key = key.as_ref().to_string();
        self
    }

    pub fn value<T: AsRef<LanguagePackStringValue>>(&mut self, value: T) -> &mut Self {
        self.inner.value = value.as_ref().clone();
        self
    }
}

impl AsRef<LanguagePackString> for LanguagePackString {
    fn as_ref(&self) -> &LanguagePackString {
        self
    }
}

impl AsRef<LanguagePackString> for RTDLanguagePackStringBuilder {
    fn as_ref(&self) -> &LanguagePackString {
        &self.inner
    }
}
