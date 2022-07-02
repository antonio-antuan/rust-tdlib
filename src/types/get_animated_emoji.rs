use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns an animated emoji corresponding to a given emoji. Returns a 404 error if the emoji has no animated emoji
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetAnimatedEmoji {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The emoji

    #[serde(default)]
    emoji: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetAnimatedEmoji {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetAnimatedEmoji {}

impl GetAnimatedEmoji {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetAnimatedEmojiBuilder {
        let mut inner = GetAnimatedEmoji::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getAnimatedEmoji".to_string();

        GetAnimatedEmojiBuilder { inner }
    }

    pub fn emoji(&self) -> &String {
        &self.emoji
    }
}

#[doc(hidden)]
pub struct GetAnimatedEmojiBuilder {
    inner: GetAnimatedEmoji,
}

#[deprecated]
pub type RTDGetAnimatedEmojiBuilder = GetAnimatedEmojiBuilder;

impl GetAnimatedEmojiBuilder {
    pub fn build(&self) -> GetAnimatedEmoji {
        self.inner.clone()
    }

    pub fn emoji<T: AsRef<str>>(&mut self, emoji: T) -> &mut Self {
        self.inner.emoji = emoji.as_ref().to_string();
        self
    }
}

impl AsRef<GetAnimatedEmoji> for GetAnimatedEmoji {
    fn as_ref(&self) -> &GetAnimatedEmoji {
        self
    }
}

impl AsRef<GetAnimatedEmoji> for GetAnimatedEmojiBuilder {
    fn as_ref(&self) -> &GetAnimatedEmoji {
        &self.inner
    }
}
