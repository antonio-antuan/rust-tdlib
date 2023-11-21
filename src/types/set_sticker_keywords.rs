use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes the list of keywords of a sticker; for bots only. The sticker must belong to a regular or custom emoji sticker set created by the bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetStickerKeywords {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Sticker

    #[serde(skip_serializing_if = "InputFile::_is_default")]
    sticker: InputFile,
    /// List of up to 20 keywords with total length up to 64 characters, which can be used to find the sticker

    #[serde(default)]
    keywords: Vec<String>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetStickerKeywords {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetStickerKeywords {}

impl SetStickerKeywords {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetStickerKeywordsBuilder {
        let mut inner = SetStickerKeywords::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setStickerKeywords".to_string();

        SetStickerKeywordsBuilder { inner }
    }

    pub fn sticker(&self) -> &InputFile {
        &self.sticker
    }

    pub fn keywords(&self) -> &Vec<String> {
        &self.keywords
    }
}

#[doc(hidden)]
pub struct SetStickerKeywordsBuilder {
    inner: SetStickerKeywords,
}

#[deprecated]
pub type RTDSetStickerKeywordsBuilder = SetStickerKeywordsBuilder;

impl SetStickerKeywordsBuilder {
    pub fn build(&self) -> SetStickerKeywords {
        self.inner.clone()
    }

    pub fn sticker<T: AsRef<InputFile>>(&mut self, sticker: T) -> &mut Self {
        self.inner.sticker = sticker.as_ref().clone();
        self
    }

    pub fn keywords(&mut self, keywords: Vec<String>) -> &mut Self {
        self.inner.keywords = keywords;
        self
    }
}

impl AsRef<SetStickerKeywords> for SetStickerKeywords {
    fn as_ref(&self) -> &SetStickerKeywords {
        self
    }
}

impl AsRef<SetStickerKeywords> for SetStickerKeywordsBuilder {
    fn as_ref(&self) -> &SetStickerKeywords {
        &self.inner
    }
}
