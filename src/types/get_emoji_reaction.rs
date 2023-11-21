use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns information about a emoji reaction. Returns a 404 error if the reaction is not found
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetEmojiReaction {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Text representation of the reaction

    #[serde(default)]
    emoji: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetEmojiReaction {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetEmojiReaction {}

impl GetEmojiReaction {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetEmojiReactionBuilder {
        let mut inner = GetEmojiReaction::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getEmojiReaction".to_string();

        GetEmojiReactionBuilder { inner }
    }

    pub fn emoji(&self) -> &String {
        &self.emoji
    }
}

#[doc(hidden)]
pub struct GetEmojiReactionBuilder {
    inner: GetEmojiReaction,
}

#[deprecated]
pub type RTDGetEmojiReactionBuilder = GetEmojiReactionBuilder;

impl GetEmojiReactionBuilder {
    pub fn build(&self) -> GetEmojiReaction {
        self.inner.clone()
    }

    pub fn emoji<T: AsRef<str>>(&mut self, emoji: T) -> &mut Self {
        self.inner.emoji = emoji.as_ref().to_string();
        self
    }
}

impl AsRef<GetEmojiReaction> for GetEmojiReaction {
    fn as_ref(&self) -> &GetEmojiReaction {
        self
    }
}

impl AsRef<GetEmojiReaction> for GetEmojiReactionBuilder {
    fn as_ref(&self) -> &GetEmojiReaction {
        &self.inner
    }
}
