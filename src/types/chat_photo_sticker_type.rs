use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes type of a sticker, which was used to create a chat photo
pub trait TDChatPhotoStickerType: Debug + RObject {}

/// Describes type of a sticker, which was used to create a chat photo
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum ChatPhotoStickerType {
    #[doc(hidden)]
    #[default]
    _Default,
    /// Information about the custom emoji, which was used to create the chat photo
    #[serde(rename = "chatPhotoStickerTypeCustomEmoji")]
    CustomEmoji(ChatPhotoStickerTypeCustomEmoji),
    /// Information about the sticker, which was used to create the chat photo
    #[serde(rename = "chatPhotoStickerTypeRegularOrMask")]
    RegularOrMask(ChatPhotoStickerTypeRegularOrMask),
}

impl RObject for ChatPhotoStickerType {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            ChatPhotoStickerType::CustomEmoji(t) => t.extra(),
            ChatPhotoStickerType::RegularOrMask(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            ChatPhotoStickerType::CustomEmoji(t) => t.client_id(),
            ChatPhotoStickerType::RegularOrMask(t) => t.client_id(),

            _ => None,
        }
    }
}

impl ChatPhotoStickerType {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, ChatPhotoStickerType::_Default)
    }
}

impl AsRef<ChatPhotoStickerType> for ChatPhotoStickerType {
    fn as_ref(&self) -> &ChatPhotoStickerType {
        self
    }
}

/// Information about the custom emoji, which was used to create the chat photo
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatPhotoStickerTypeCustomEmoji {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the custom emoji

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    custom_emoji_id: i64,
}

impl RObject for ChatPhotoStickerTypeCustomEmoji {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatPhotoStickerType for ChatPhotoStickerTypeCustomEmoji {}

impl ChatPhotoStickerTypeCustomEmoji {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatPhotoStickerTypeCustomEmojiBuilder {
        let mut inner = ChatPhotoStickerTypeCustomEmoji::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatPhotoStickerTypeCustomEmojiBuilder { inner }
    }

    pub fn custom_emoji_id(&self) -> i64 {
        self.custom_emoji_id
    }
}

#[doc(hidden)]
pub struct ChatPhotoStickerTypeCustomEmojiBuilder {
    inner: ChatPhotoStickerTypeCustomEmoji,
}

#[deprecated]
pub type RTDChatPhotoStickerTypeCustomEmojiBuilder = ChatPhotoStickerTypeCustomEmojiBuilder;

impl ChatPhotoStickerTypeCustomEmojiBuilder {
    pub fn build(&self) -> ChatPhotoStickerTypeCustomEmoji {
        self.inner.clone()
    }

    pub fn custom_emoji_id(&mut self, custom_emoji_id: i64) -> &mut Self {
        self.inner.custom_emoji_id = custom_emoji_id;
        self
    }
}

impl AsRef<ChatPhotoStickerTypeCustomEmoji> for ChatPhotoStickerTypeCustomEmoji {
    fn as_ref(&self) -> &ChatPhotoStickerTypeCustomEmoji {
        self
    }
}

impl AsRef<ChatPhotoStickerTypeCustomEmoji> for ChatPhotoStickerTypeCustomEmojiBuilder {
    fn as_ref(&self) -> &ChatPhotoStickerTypeCustomEmoji {
        &self.inner
    }
}

/// Information about the sticker, which was used to create the chat photo
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatPhotoStickerTypeRegularOrMask {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Sticker set identifier

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    sticker_set_id: i64,
    /// Identifier of the sticker in the set

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    sticker_id: i64,
}

impl RObject for ChatPhotoStickerTypeRegularOrMask {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatPhotoStickerType for ChatPhotoStickerTypeRegularOrMask {}

impl ChatPhotoStickerTypeRegularOrMask {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatPhotoStickerTypeRegularOrMaskBuilder {
        let mut inner = ChatPhotoStickerTypeRegularOrMask::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatPhotoStickerTypeRegularOrMaskBuilder { inner }
    }

    pub fn sticker_set_id(&self) -> i64 {
        self.sticker_set_id
    }

    pub fn sticker_id(&self) -> i64 {
        self.sticker_id
    }
}

#[doc(hidden)]
pub struct ChatPhotoStickerTypeRegularOrMaskBuilder {
    inner: ChatPhotoStickerTypeRegularOrMask,
}

#[deprecated]
pub type RTDChatPhotoStickerTypeRegularOrMaskBuilder = ChatPhotoStickerTypeRegularOrMaskBuilder;

impl ChatPhotoStickerTypeRegularOrMaskBuilder {
    pub fn build(&self) -> ChatPhotoStickerTypeRegularOrMask {
        self.inner.clone()
    }

    pub fn sticker_set_id(&mut self, sticker_set_id: i64) -> &mut Self {
        self.inner.sticker_set_id = sticker_set_id;
        self
    }

    pub fn sticker_id(&mut self, sticker_id: i64) -> &mut Self {
        self.inner.sticker_id = sticker_id;
        self
    }
}

impl AsRef<ChatPhotoStickerTypeRegularOrMask> for ChatPhotoStickerTypeRegularOrMask {
    fn as_ref(&self) -> &ChatPhotoStickerTypeRegularOrMask {
        self
    }
}

impl AsRef<ChatPhotoStickerTypeRegularOrMask> for ChatPhotoStickerTypeRegularOrMaskBuilder {
    fn as_ref(&self) -> &ChatPhotoStickerTypeRegularOrMask {
        &self.inner
    }
}
