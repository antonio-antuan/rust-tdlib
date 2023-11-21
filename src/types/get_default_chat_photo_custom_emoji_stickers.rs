use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns default list of custom emoji stickers for placing on a chat photo
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetDefaultChatPhotoCustomEmojiStickers {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetDefaultChatPhotoCustomEmojiStickers {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetDefaultChatPhotoCustomEmojiStickers {}

impl GetDefaultChatPhotoCustomEmojiStickers {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetDefaultChatPhotoCustomEmojiStickersBuilder {
        let mut inner = GetDefaultChatPhotoCustomEmojiStickers::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getDefaultChatPhotoCustomEmojiStickers".to_string();

        GetDefaultChatPhotoCustomEmojiStickersBuilder { inner }
    }
}

#[doc(hidden)]
pub struct GetDefaultChatPhotoCustomEmojiStickersBuilder {
    inner: GetDefaultChatPhotoCustomEmojiStickers,
}

#[deprecated]
pub type RTDGetDefaultChatPhotoCustomEmojiStickersBuilder =
    GetDefaultChatPhotoCustomEmojiStickersBuilder;

impl GetDefaultChatPhotoCustomEmojiStickersBuilder {
    pub fn build(&self) -> GetDefaultChatPhotoCustomEmojiStickers {
        self.inner.clone()
    }
}

impl AsRef<GetDefaultChatPhotoCustomEmojiStickers> for GetDefaultChatPhotoCustomEmojiStickers {
    fn as_ref(&self) -> &GetDefaultChatPhotoCustomEmojiStickers {
        self
    }
}

impl AsRef<GetDefaultChatPhotoCustomEmojiStickers>
    for GetDefaultChatPhotoCustomEmojiStickersBuilder
{
    fn as_ref(&self) -> &GetDefaultChatPhotoCustomEmojiStickers {
        &self.inner
    }
}
