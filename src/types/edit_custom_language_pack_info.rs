use crate::errors::Result;
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> EditCustomLanguagePackInfoBuilder {
        let mut inner = EditCustomLanguagePackInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "editCustomLanguagePackInfo".to_string();

        EditCustomLanguagePackInfoBuilder { inner }
    }

    pub fn info(&self) -> &LanguagePackInfo {
        &self.info
    }
}

#[doc(hidden)]
pub struct EditCustomLanguagePackInfoBuilder {
    inner: EditCustomLanguagePackInfo,
}

#[deprecated]
pub type RTDEditCustomLanguagePackInfoBuilder = EditCustomLanguagePackInfoBuilder;

impl EditCustomLanguagePackInfoBuilder {
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

impl AsRef<EditCustomLanguagePackInfo> for EditCustomLanguagePackInfoBuilder {
    fn as_ref(&self) -> &EditCustomLanguagePackInfo {
        &self.inner
    }
}
