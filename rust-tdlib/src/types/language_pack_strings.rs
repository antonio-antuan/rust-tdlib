use crate::errors::*;
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDLanguagePackStringsBuilder {
        let mut inner = LanguagePackStrings::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDLanguagePackStringsBuilder { inner }
    }

    pub fn strings(&self) -> &Vec<LanguagePackString> {
        &self.strings
    }
}

#[doc(hidden)]
pub struct RTDLanguagePackStringsBuilder {
    inner: LanguagePackStrings,
}

impl RTDLanguagePackStringsBuilder {
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

impl AsRef<LanguagePackStrings> for RTDLanguagePackStringsBuilder {
    fn as_ref(&self) -> &LanguagePackStrings {
        &self.inner
    }
}
