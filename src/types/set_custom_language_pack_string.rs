use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Adds, edits or deletes a string in a custom local language pack. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetCustomLanguagePackString {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of a previously added custom local language pack in the current localization target
    language_pack_id: String,
    /// New language pack string
    new_string: LanguagePackString,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetCustomLanguagePackString {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetCustomLanguagePackString {}

impl SetCustomLanguagePackString {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSetCustomLanguagePackStringBuilder {
        let mut inner = SetCustomLanguagePackString::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setCustomLanguagePackString".to_string();

        RTDSetCustomLanguagePackStringBuilder { inner }
    }

    pub fn language_pack_id(&self) -> &String {
        &self.language_pack_id
    }

    pub fn new_string(&self) -> &LanguagePackString {
        &self.new_string
    }
}

#[doc(hidden)]
pub struct RTDSetCustomLanguagePackStringBuilder {
    inner: SetCustomLanguagePackString,
}

impl RTDSetCustomLanguagePackStringBuilder {
    pub fn build(&self) -> SetCustomLanguagePackString {
        self.inner.clone()
    }

    pub fn language_pack_id<T: AsRef<str>>(&mut self, language_pack_id: T) -> &mut Self {
        self.inner.language_pack_id = language_pack_id.as_ref().to_string();
        self
    }

    pub fn new_string<T: AsRef<LanguagePackString>>(&mut self, new_string: T) -> &mut Self {
        self.inner.new_string = new_string.as_ref().clone();
        self
    }
}

impl AsRef<SetCustomLanguagePackString> for SetCustomLanguagePackString {
    fn as_ref(&self) -> &SetCustomLanguagePackString {
        self
    }
}

impl AsRef<SetCustomLanguagePackString> for RTDSetCustomLanguagePackStringBuilder {
    fn as_ref(&self) -> &SetCustomLanguagePackString {
        &self.inner
    }
}
