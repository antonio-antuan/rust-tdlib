use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns default list of custom emoji stickers for reply background
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetDefaultBackgroundCustomEmojiStickers {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetDefaultBackgroundCustomEmojiStickers {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetDefaultBackgroundCustomEmojiStickers {}

impl GetDefaultBackgroundCustomEmojiStickers {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetDefaultBackgroundCustomEmojiStickersBuilder {
        let mut inner = GetDefaultBackgroundCustomEmojiStickers::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getDefaultBackgroundCustomEmojiStickers".to_string();

        GetDefaultBackgroundCustomEmojiStickersBuilder { inner }
    }
}

#[doc(hidden)]
pub struct GetDefaultBackgroundCustomEmojiStickersBuilder {
    inner: GetDefaultBackgroundCustomEmojiStickers,
}

#[deprecated]
pub type RTDGetDefaultBackgroundCustomEmojiStickersBuilder =
    GetDefaultBackgroundCustomEmojiStickersBuilder;

impl GetDefaultBackgroundCustomEmojiStickersBuilder {
    pub fn build(&self) -> GetDefaultBackgroundCustomEmojiStickers {
        self.inner.clone()
    }
}

impl AsRef<GetDefaultBackgroundCustomEmojiStickers> for GetDefaultBackgroundCustomEmojiStickers {
    fn as_ref(&self) -> &GetDefaultBackgroundCustomEmojiStickers {
        self
    }
}

impl AsRef<GetDefaultBackgroundCustomEmojiStickers>
    for GetDefaultBackgroundCustomEmojiStickersBuilder
{
    fn as_ref(&self) -> &GetDefaultBackgroundCustomEmojiStickers {
        &self.inner
    }
}
