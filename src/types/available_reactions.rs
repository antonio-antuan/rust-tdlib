use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of reactions that can be added to a message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AvailableReactions {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// List of reactions to be shown at the top

    #[serde(default)]
    top_reactions: Vec<AvailableReaction>,
    /// List of recently used reactions

    #[serde(default)]
    recent_reactions: Vec<AvailableReaction>,
    /// List of popular reactions

    #[serde(default)]
    popular_reactions: Vec<AvailableReaction>,
    /// True, if custom emoji reactions could be added by Telegram Premium subscribers

    #[serde(default)]
    allow_custom_emoji: bool,
}

impl RObject for AvailableReactions {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl AvailableReactions {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AvailableReactionsBuilder {
        let mut inner = AvailableReactions::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        AvailableReactionsBuilder { inner }
    }

    pub fn top_reactions(&self) -> &Vec<AvailableReaction> {
        &self.top_reactions
    }

    pub fn recent_reactions(&self) -> &Vec<AvailableReaction> {
        &self.recent_reactions
    }

    pub fn popular_reactions(&self) -> &Vec<AvailableReaction> {
        &self.popular_reactions
    }

    pub fn allow_custom_emoji(&self) -> bool {
        self.allow_custom_emoji
    }
}

#[doc(hidden)]
pub struct AvailableReactionsBuilder {
    inner: AvailableReactions,
}

#[deprecated]
pub type RTDAvailableReactionsBuilder = AvailableReactionsBuilder;

impl AvailableReactionsBuilder {
    pub fn build(&self) -> AvailableReactions {
        self.inner.clone()
    }

    pub fn top_reactions(&mut self, top_reactions: Vec<AvailableReaction>) -> &mut Self {
        self.inner.top_reactions = top_reactions;
        self
    }

    pub fn recent_reactions(&mut self, recent_reactions: Vec<AvailableReaction>) -> &mut Self {
        self.inner.recent_reactions = recent_reactions;
        self
    }

    pub fn popular_reactions(&mut self, popular_reactions: Vec<AvailableReaction>) -> &mut Self {
        self.inner.popular_reactions = popular_reactions;
        self
    }

    pub fn allow_custom_emoji(&mut self, allow_custom_emoji: bool) -> &mut Self {
        self.inner.allow_custom_emoji = allow_custom_emoji;
        self
    }
}

impl AsRef<AvailableReactions> for AvailableReactions {
    fn as_ref(&self) -> &AvailableReactions {
        self
    }
}

impl AsRef<AvailableReactions> for AvailableReactionsBuilder {
    fn as_ref(&self) -> &AvailableReactions {
        &self.inner
    }
}
