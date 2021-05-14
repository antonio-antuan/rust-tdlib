use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Deletes all information about a language pack in the current localization target. The language pack which is currently in use (including base language pack) or is being synchronized can't be deleted. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteLanguagePack {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the language pack to delete
    language_pack_id: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for DeleteLanguagePack {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for DeleteLanguagePack {}

impl DeleteLanguagePack {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDDeleteLanguagePackBuilder {
        let mut inner = DeleteLanguagePack::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "deleteLanguagePack".to_string();

        RTDDeleteLanguagePackBuilder { inner }
    }

    pub fn language_pack_id(&self) -> &String {
        &self.language_pack_id
    }
}

#[doc(hidden)]
pub struct RTDDeleteLanguagePackBuilder {
    inner: DeleteLanguagePack,
}

impl RTDDeleteLanguagePackBuilder {
    pub fn build(&self) -> DeleteLanguagePack {
        self.inner.clone()
    }

    pub fn language_pack_id<T: AsRef<str>>(&mut self, language_pack_id: T) -> &mut Self {
        self.inner.language_pack_id = language_pack_id.as_ref().to_string();
        self
    }
}

impl AsRef<DeleteLanguagePack> for DeleteLanguagePack {
    fn as_ref(&self) -> &DeleteLanguagePack {
        self
    }
}

impl AsRef<DeleteLanguagePack> for RTDDeleteLanguagePackBuilder {
    fn as_ref(&self) -> &DeleteLanguagePack {
        &self.inner
    }
}
