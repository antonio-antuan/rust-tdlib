use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes the pinned state of a forum topic; requires can_manage_topics administrator right in the supergroup. There can be up to getOption("pinned_forum_topic_count_max") pinned forum topics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToggleForumTopicIsPinned {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// Message thread identifier of the forum topic

    #[serde(default)]
    message_thread_id: i64,
    /// Pass true to pin the topic; pass false to unpin it

    #[serde(default)]
    is_pinned: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ToggleForumTopicIsPinned {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ToggleForumTopicIsPinned {}

impl ToggleForumTopicIsPinned {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ToggleForumTopicIsPinnedBuilder {
        let mut inner = ToggleForumTopicIsPinned::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "toggleForumTopicIsPinned".to_string();

        ToggleForumTopicIsPinnedBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_thread_id(&self) -> i64 {
        self.message_thread_id
    }

    pub fn is_pinned(&self) -> bool {
        self.is_pinned
    }
}

#[doc(hidden)]
pub struct ToggleForumTopicIsPinnedBuilder {
    inner: ToggleForumTopicIsPinned,
}

#[deprecated]
pub type RTDToggleForumTopicIsPinnedBuilder = ToggleForumTopicIsPinnedBuilder;

impl ToggleForumTopicIsPinnedBuilder {
    pub fn build(&self) -> ToggleForumTopicIsPinned {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_thread_id(&mut self, message_thread_id: i64) -> &mut Self {
        self.inner.message_thread_id = message_thread_id;
        self
    }

    pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
        self.inner.is_pinned = is_pinned;
        self
    }
}

impl AsRef<ToggleForumTopicIsPinned> for ToggleForumTopicIsPinned {
    fn as_ref(&self) -> &ToggleForumTopicIsPinned {
        self
    }
}

impl AsRef<ToggleForumTopicIsPinned> for ToggleForumTopicIsPinnedBuilder {
    fn as_ref(&self) -> &ToggleForumTopicIsPinned {
        &self.inner
    }
}
