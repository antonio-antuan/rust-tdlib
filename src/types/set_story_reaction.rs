use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes chosen reaction on a story
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetStoryReaction {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The identifier of the sender of the story

    #[serde(default)]
    story_sender_chat_id: i64,
    /// The identifier of the story

    #[serde(default)]
    story_id: i32,
    /// Type of the reaction to set; pass null to remove the reaction. `reactionTypeCustomEmoji` reactions can be used only by Telegram Premium users

    #[serde(skip_serializing_if = "ReactionType::_is_default")]
    reaction_type: ReactionType,
    /// Pass true if the reaction needs to be added to recent reactions

    #[serde(default)]
    update_recent_reactions: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetStoryReaction {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetStoryReaction {}

impl SetStoryReaction {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetStoryReactionBuilder {
        let mut inner = SetStoryReaction::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setStoryReaction".to_string();

        SetStoryReactionBuilder { inner }
    }

    pub fn story_sender_chat_id(&self) -> i64 {
        self.story_sender_chat_id
    }

    pub fn story_id(&self) -> i32 {
        self.story_id
    }

    pub fn reaction_type(&self) -> &ReactionType {
        &self.reaction_type
    }

    pub fn update_recent_reactions(&self) -> bool {
        self.update_recent_reactions
    }
}

#[doc(hidden)]
pub struct SetStoryReactionBuilder {
    inner: SetStoryReaction,
}

#[deprecated]
pub type RTDSetStoryReactionBuilder = SetStoryReactionBuilder;

impl SetStoryReactionBuilder {
    pub fn build(&self) -> SetStoryReaction {
        self.inner.clone()
    }

    pub fn story_sender_chat_id(&mut self, story_sender_chat_id: i64) -> &mut Self {
        self.inner.story_sender_chat_id = story_sender_chat_id;
        self
    }

    pub fn story_id(&mut self, story_id: i32) -> &mut Self {
        self.inner.story_id = story_id;
        self
    }

    pub fn reaction_type<T: AsRef<ReactionType>>(&mut self, reaction_type: T) -> &mut Self {
        self.inner.reaction_type = reaction_type.as_ref().clone();
        self
    }

    pub fn update_recent_reactions(&mut self, update_recent_reactions: bool) -> &mut Self {
        self.inner.update_recent_reactions = update_recent_reactions;
        self
    }
}

impl AsRef<SetStoryReaction> for SetStoryReaction {
    fn as_ref(&self) -> &SetStoryReaction {
        self
    }
}

impl AsRef<SetStoryReaction> for SetStoryReactionBuilder {
    fn as_ref(&self) -> &SetStoryReaction {
        &self.inner
    }
}
