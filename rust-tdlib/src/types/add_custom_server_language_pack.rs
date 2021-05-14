use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Adds a custom server language pack to the list of installed language packs in current localization target. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddCustomServerLanguagePack {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of a language pack to be added; may be different from a name that is used in an "https://t.me/setlanguage/" link
    language_pack_id: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for AddCustomServerLanguagePack {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for AddCustomServerLanguagePack {}

impl AddCustomServerLanguagePack {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDAddCustomServerLanguagePackBuilder {
        let mut inner = AddCustomServerLanguagePack::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "addCustomServerLanguagePack".to_string();

        RTDAddCustomServerLanguagePackBuilder { inner }
    }

    pub fn language_pack_id(&self) -> &String {
        &self.language_pack_id
    }
}

#[doc(hidden)]
pub struct RTDAddCustomServerLanguagePackBuilder {
    inner: AddCustomServerLanguagePack,
}

impl RTDAddCustomServerLanguagePackBuilder {
    pub fn build(&self) -> AddCustomServerLanguagePack {
        self.inner.clone()
    }

    pub fn language_pack_id<T: AsRef<str>>(&mut self, language_pack_id: T) -> &mut Self {
        self.inner.language_pack_id = language_pack_id.as_ref().to_string();
        self
    }
}

impl AsRef<AddCustomServerLanguagePack> for AddCustomServerLanguagePack {
    fn as_ref(&self) -> &AddCustomServerLanguagePack {
        self
    }
}

impl AsRef<AddCustomServerLanguagePack> for RTDAddCustomServerLanguagePackBuilder {
    fn as_ref(&self) -> &AddCustomServerLanguagePack {
        &self.inner
    }
}
