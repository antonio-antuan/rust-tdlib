use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Toggles whether a story is accessible after expiration. Can be called only if story.can_toggle_is_pinned == true
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToggleStoryIsPinned {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat that posted the story

    #[serde(default)]
    story_sender_chat_id: i64,
    /// Identifier of the story

    #[serde(default)]
    story_id: i32,
    /// Pass true to make the story accessible after expiration; pass false to make it private

    #[serde(default)]
    is_pinned: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ToggleStoryIsPinned {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ToggleStoryIsPinned {}

impl ToggleStoryIsPinned {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ToggleStoryIsPinnedBuilder {
        let mut inner = ToggleStoryIsPinned::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "toggleStoryIsPinned".to_string();

        ToggleStoryIsPinnedBuilder { inner }
    }

    pub fn story_sender_chat_id(&self) -> i64 {
        self.story_sender_chat_id
    }

    pub fn story_id(&self) -> i32 {
        self.story_id
    }

    pub fn is_pinned(&self) -> bool {
        self.is_pinned
    }
}

#[doc(hidden)]
pub struct ToggleStoryIsPinnedBuilder {
    inner: ToggleStoryIsPinned,
}

#[deprecated]
pub type RTDToggleStoryIsPinnedBuilder = ToggleStoryIsPinnedBuilder;

impl ToggleStoryIsPinnedBuilder {
    pub fn build(&self) -> ToggleStoryIsPinned {
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

    pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
        self.inner.is_pinned = is_pinned;
        self
    }
}

impl AsRef<ToggleStoryIsPinned> for ToggleStoryIsPinned {
    fn as_ref(&self) -> &ToggleStoryIsPinned {
        self
    }
}

impl AsRef<ToggleStoryIsPinned> for ToggleStoryIsPinnedBuilder {
    fn as_ref(&self) -> &ToggleStoryIsPinned {
        &self.inner
    }
}
