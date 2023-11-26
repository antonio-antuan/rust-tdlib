use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes type of message reaction
pub trait TDReactionType: Debug + RObject {}

/// Describes type of message reaction
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum ReactionType {
    #[doc(hidden)]
    #[default]
    _Default,
    /// A reaction with a custom emoji
    #[serde(rename = "reactionTypeCustomEmoji")]
    CustomEmoji(ReactionTypeCustomEmoji),
    /// A reaction with an emoji
    #[serde(rename = "reactionTypeEmoji")]
    Emoji(ReactionTypeEmoji),
}

impl RObject for ReactionType {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            ReactionType::CustomEmoji(t) => t.extra(),
            ReactionType::Emoji(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            ReactionType::CustomEmoji(t) => t.client_id(),
            ReactionType::Emoji(t) => t.client_id(),

            _ => None,
        }
    }
}

impl ReactionType {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, ReactionType::_Default)
    }
}

impl AsRef<ReactionType> for ReactionType {
    fn as_ref(&self) -> &ReactionType {
        self
    }
}

/// A reaction with a custom emoji
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReactionTypeCustomEmoji {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unique identifier of the custom emoji

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    custom_emoji_id: i64,
}

impl RObject for ReactionTypeCustomEmoji {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDReactionType for ReactionTypeCustomEmoji {}

impl ReactionTypeCustomEmoji {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ReactionTypeCustomEmojiBuilder {
        let mut inner = ReactionTypeCustomEmoji::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ReactionTypeCustomEmojiBuilder { inner }
    }

    pub fn custom_emoji_id(&self) -> i64 {
        self.custom_emoji_id
    }
}

#[doc(hidden)]
pub struct ReactionTypeCustomEmojiBuilder {
    inner: ReactionTypeCustomEmoji,
}

#[deprecated]
pub type RTDReactionTypeCustomEmojiBuilder = ReactionTypeCustomEmojiBuilder;

impl ReactionTypeCustomEmojiBuilder {
    pub fn build(&self) -> ReactionTypeCustomEmoji {
        self.inner.clone()
    }

    pub fn custom_emoji_id(&mut self, custom_emoji_id: i64) -> &mut Self {
        self.inner.custom_emoji_id = custom_emoji_id;
        self
    }
}

impl AsRef<ReactionTypeCustomEmoji> for ReactionTypeCustomEmoji {
    fn as_ref(&self) -> &ReactionTypeCustomEmoji {
        self
    }
}

impl AsRef<ReactionTypeCustomEmoji> for ReactionTypeCustomEmojiBuilder {
    fn as_ref(&self) -> &ReactionTypeCustomEmoji {
        &self.inner
    }
}

/// A reaction with an emoji
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReactionTypeEmoji {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Text representation of the reaction

    #[serde(default)]
    emoji: String,
}

impl RObject for ReactionTypeEmoji {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDReactionType for ReactionTypeEmoji {}

impl ReactionTypeEmoji {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ReactionTypeEmojiBuilder {
        let mut inner = ReactionTypeEmoji::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ReactionTypeEmojiBuilder { inner }
    }

    pub fn emoji(&self) -> &String {
        &self.emoji
    }
}

#[doc(hidden)]
pub struct ReactionTypeEmojiBuilder {
    inner: ReactionTypeEmoji,
}

#[deprecated]
pub type RTDReactionTypeEmojiBuilder = ReactionTypeEmojiBuilder;

impl ReactionTypeEmojiBuilder {
    pub fn build(&self) -> ReactionTypeEmoji {
        self.inner.clone()
    }

    pub fn emoji<T: AsRef<str>>(&mut self, emoji: T) -> &mut Self {
        self.inner.emoji = emoji.as_ref().to_string();
        self
    }
}

impl AsRef<ReactionTypeEmoji> for ReactionTypeEmoji {
    fn as_ref(&self) -> &ReactionTypeEmoji {
        self
    }
}

impl AsRef<ReactionTypeEmoji> for ReactionTypeEmojiBuilder {
    fn as_ref(&self) -> &ReactionTypeEmoji {
        &self.inner
    }
}
