use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns a suggested name for a new sticker set with a given title
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetSuggestedStickerSetName {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Sticker set title; 1-64 characters

    #[serde(default)]
    title: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetSuggestedStickerSetName {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetSuggestedStickerSetName {}

impl GetSuggestedStickerSetName {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetSuggestedStickerSetNameBuilder {
        let mut inner = GetSuggestedStickerSetName::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getSuggestedStickerSetName".to_string();

        GetSuggestedStickerSetNameBuilder { inner }
    }

    pub fn title(&self) -> &String {
        &self.title
    }
}

#[doc(hidden)]
pub struct GetSuggestedStickerSetNameBuilder {
    inner: GetSuggestedStickerSetName,
}

#[deprecated]
pub type RTDGetSuggestedStickerSetNameBuilder = GetSuggestedStickerSetNameBuilder;

impl GetSuggestedStickerSetNameBuilder {
    pub fn build(&self) -> GetSuggestedStickerSetName {
        self.inner.clone()
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }
}

impl AsRef<GetSuggestedStickerSetName> for GetSuggestedStickerSetName {
    fn as_ref(&self) -> &GetSuggestedStickerSetName {
        self
    }
}

impl AsRef<GetSuggestedStickerSetName> for GetSuggestedStickerSetNameBuilder {
    fn as_ref(&self) -> &GetSuggestedStickerSetName {
        &self.inner
    }
}
