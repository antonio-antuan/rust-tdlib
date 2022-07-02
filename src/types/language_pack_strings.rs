use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of language pack strings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LanguagePackStrings {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// A list of language pack strings

    #[serde(default)]
    strings: Vec<LanguagePackString>,
}

impl RObject for LanguagePackStrings {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl LanguagePackStrings {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> LanguagePackStringsBuilder {
        let mut inner = LanguagePackStrings::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        LanguagePackStringsBuilder { inner }
    }

    pub fn strings(&self) -> &Vec<LanguagePackString> {
        &self.strings
    }
}

#[doc(hidden)]
pub struct LanguagePackStringsBuilder {
    inner: LanguagePackStrings,
}

#[deprecated]
pub type RTDLanguagePackStringsBuilder = LanguagePackStringsBuilder;

impl LanguagePackStringsBuilder {
    pub fn build(&self) -> LanguagePackStrings {
        self.inner.clone()
    }

    pub fn strings(&mut self, strings: Vec<LanguagePackString>) -> &mut Self {
        self.inner.strings = strings;
        self
    }
}

impl AsRef<LanguagePackStrings> for LanguagePackStrings {
    fn as_ref(&self) -> &LanguagePackStrings {
        self
    }
}

impl AsRef<LanguagePackStrings> for LanguagePackStringsBuilder {
    fn as_ref(&self) -> &LanguagePackStrings {
        &self.inner
    }
}
