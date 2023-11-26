use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes the emoji status of the current user; for Telegram Premium users only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetEmojiStatus {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// New emoji status; pass null to switch to the default badge
    emoji_status: EmojiStatus,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetEmojiStatus {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetEmojiStatus {}

impl SetEmojiStatus {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetEmojiStatusBuilder {
        let mut inner = SetEmojiStatus::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setEmojiStatus".to_string();

        SetEmojiStatusBuilder { inner }
    }

    pub fn emoji_status(&self) -> &EmojiStatus {
        &self.emoji_status
    }
}

#[doc(hidden)]
pub struct SetEmojiStatusBuilder {
    inner: SetEmojiStatus,
}

#[deprecated]
pub type RTDSetEmojiStatusBuilder = SetEmojiStatusBuilder;

impl SetEmojiStatusBuilder {
    pub fn build(&self) -> SetEmojiStatus {
        self.inner.clone()
    }

    pub fn emoji_status<T: AsRef<EmojiStatus>>(&mut self, emoji_status: T) -> &mut Self {
        self.inner.emoji_status = emoji_status.as_ref().clone();
        self
    }
}

impl AsRef<SetEmojiStatus> for SetEmojiStatus {
    fn as_ref(&self) -> &SetEmojiStatus {
        self
    }
}

impl AsRef<SetEmojiStatus> for SetEmojiStatusBuilder {
    fn as_ref(&self) -> &SetEmojiStatus {
        &self.inner
    }
}
