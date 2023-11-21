use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes a custom emoji to be shown instead of the Telegram Premium badge
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmojiStatus {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the custom emoji in stickerFormatTgs format

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    custom_emoji_id: i64,
    /// Point in time (Unix timestamp) when the status will expire; 0 if never

    #[serde(default)]
    expiration_date: i32,
}

impl RObject for EmojiStatus {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl EmojiStatus {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> EmojiStatusBuilder {
        let mut inner = EmojiStatus::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        EmojiStatusBuilder { inner }
    }

    pub fn custom_emoji_id(&self) -> i64 {
        self.custom_emoji_id
    }

    pub fn expiration_date(&self) -> i32 {
        self.expiration_date
    }
}

#[doc(hidden)]
pub struct EmojiStatusBuilder {
    inner: EmojiStatus,
}

#[deprecated]
pub type RTDEmojiStatusBuilder = EmojiStatusBuilder;

impl EmojiStatusBuilder {
    pub fn build(&self) -> EmojiStatus {
        self.inner.clone()
    }

    pub fn custom_emoji_id(&mut self, custom_emoji_id: i64) -> &mut Self {
        self.inner.custom_emoji_id = custom_emoji_id;
        self
    }

    pub fn expiration_date(&mut self, expiration_date: i32) -> &mut Self {
        self.inner.expiration_date = expiration_date;
        self
    }
}

impl AsRef<EmojiStatus> for EmojiStatus {
    fn as_ref(&self) -> &EmojiStatus {
        self
    }
}

impl AsRef<EmojiStatus> for EmojiStatusBuilder {
    fn as_ref(&self) -> &EmojiStatus {
        &self.inner
    }
}
