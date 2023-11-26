use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of custom emoji identifiers, which can be set as emoji statuses
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmojiStatuses {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The list of custom emoji identifiers

    #[serde(deserialize_with = "super::_common::vec_of_i64_from_str")]
    #[serde(default)]
    custom_emoji_ids: Vec<i64>,
}

impl RObject for EmojiStatuses {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl EmojiStatuses {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> EmojiStatusesBuilder {
        let mut inner = EmojiStatuses::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        EmojiStatusesBuilder { inner }
    }

    pub fn custom_emoji_ids(&self) -> &Vec<i64> {
        &self.custom_emoji_ids
    }
}

#[doc(hidden)]
pub struct EmojiStatusesBuilder {
    inner: EmojiStatuses,
}

#[deprecated]
pub type RTDEmojiStatusesBuilder = EmojiStatusesBuilder;

impl EmojiStatusesBuilder {
    pub fn build(&self) -> EmojiStatuses {
        self.inner.clone()
    }

    pub fn custom_emoji_ids(&mut self, custom_emoji_ids: Vec<i64>) -> &mut Self {
        self.inner.custom_emoji_ids = custom_emoji_ids;
        self
    }
}

impl AsRef<EmojiStatuses> for EmojiStatuses {
    fn as_ref(&self) -> &EmojiStatuses {
        self
    }
}

impl AsRef<EmojiStatuses> for EmojiStatusesBuilder {
    fn as_ref(&self) -> &EmojiStatuses {
        &self.inner
    }
}
