use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Sets a custom emoji sticker set thumbnail; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetCustomEmojiStickerSetThumbnail {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Sticker set name

    #[serde(default)]
    name: String,
    /// Identifier of the custom emoji from the sticker set, which will be set as sticker set thumbnail; pass 0 to remove the sticker set thumbnail

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    custom_emoji_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetCustomEmojiStickerSetThumbnail {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetCustomEmojiStickerSetThumbnail {}

impl SetCustomEmojiStickerSetThumbnail {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetCustomEmojiStickerSetThumbnailBuilder {
        let mut inner = SetCustomEmojiStickerSetThumbnail::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setCustomEmojiStickerSetThumbnail".to_string();

        SetCustomEmojiStickerSetThumbnailBuilder { inner }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn custom_emoji_id(&self) -> i64 {
        self.custom_emoji_id
    }
}

#[doc(hidden)]
pub struct SetCustomEmojiStickerSetThumbnailBuilder {
    inner: SetCustomEmojiStickerSetThumbnail,
}

#[deprecated]
pub type RTDSetCustomEmojiStickerSetThumbnailBuilder = SetCustomEmojiStickerSetThumbnailBuilder;

impl SetCustomEmojiStickerSetThumbnailBuilder {
    pub fn build(&self) -> SetCustomEmojiStickerSetThumbnail {
        self.inner.clone()
    }

    pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
        self.inner.name = name.as_ref().to_string();
        self
    }

    pub fn custom_emoji_id(&mut self, custom_emoji_id: i64) -> &mut Self {
        self.inner.custom_emoji_id = custom_emoji_id;
        self
    }
}

impl AsRef<SetCustomEmojiStickerSetThumbnail> for SetCustomEmojiStickerSetThumbnail {
    fn as_ref(&self) -> &SetCustomEmojiStickerSetThumbnail {
        self
    }
}

impl AsRef<SetCustomEmojiStickerSetThumbnail> for SetCustomEmojiStickerSetThumbnailBuilder {
    fn as_ref(&self) -> &SetCustomEmojiStickerSetThumbnail {
        &self.inner
    }
}
