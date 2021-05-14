use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Edits information about a custom local language pack in the current localization target. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EditCustomLanguagePackInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// New information about the custom local language pack
    info: LanguagePackInfo,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for EditCustomLanguagePackInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for EditCustomLanguagePackInfo {}

impl EditCustomLanguagePackInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDEditCustomLanguagePackInfoBuilder {
        let mut inner = EditCustomLanguagePackInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "editCustomLanguagePackInfo".to_string();

        RTDEditCustomLanguagePackInfoBuilder { inner }
    }

    pub fn info(&self) -> &LanguagePackInfo {
        &self.info
    }
}

#[doc(hidden)]
pub struct RTDEditCustomLanguagePackInfoBuilder {
    inner: EditCustomLanguagePackInfo,
}

impl RTDEditCustomLanguagePackInfoBuilder {
    pub fn build(&self) -> EditCustomLanguagePackInfo {
        self.inner.clone()
    }

    pub fn info<T: AsRef<LanguagePackInfo>>(&mut self, info: T) -> &mut Self {
        self.inner.info = info.as_ref().clone();
        self
    }
}

impl AsRef<EditCustomLanguagePackInfo> for EditCustomLanguagePackInfo {
    fn as_ref(&self) -> &EditCustomLanguagePackInfo {
        self
    }
}

impl AsRef<EditCustomLanguagePackInfo> for RTDEditCustomLanguagePackInfoBuilder {
    fn as_ref(&self) -> &EditCustomLanguagePackInfo {
        &self.inner
    }
}
