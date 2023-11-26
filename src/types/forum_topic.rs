use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes a forum topic
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ForumTopic {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Basic information about the topic
    info: ForumTopicInfo,
    /// Last message in the topic; may be null if unknown
    last_message: Option<Message>,
    /// True, if the topic is pinned in the topic list

    #[serde(default)]
    is_pinned: bool,
    /// Number of unread messages in the topic

    #[serde(default)]
    unread_count: i32,
    /// Identifier of the last read incoming message

    #[serde(default)]
    last_read_inbox_message_id: i64,
    /// Identifier of the last read outgoing message

    #[serde(default)]
    last_read_outbox_message_id: i64,
    /// Number of unread messages with a mention/reply in the topic

    #[serde(default)]
    unread_mention_count: i32,
    /// Number of messages with unread reactions in the topic

    #[serde(default)]
    unread_reaction_count: i32,
    /// Notification settings for the topic
    notification_settings: ChatNotificationSettings,
    /// A draft of a message in the topic; may be null if none
    draft_message: Option<DraftMessage>,
}

impl RObject for ForumTopic {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ForumTopic {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ForumTopicBuilder {
        let mut inner = ForumTopic::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ForumTopicBuilder { inner }
    }

    pub fn info(&self) -> &ForumTopicInfo {
        &self.info
    }

    pub fn last_message(&self) -> &Option<Message> {
        &self.last_message
    }

    pub fn is_pinned(&self) -> bool {
        self.is_pinned
    }

    pub fn unread_count(&self) -> i32 {
        self.unread_count
    }

    pub fn last_read_inbox_message_id(&self) -> i64 {
        self.last_read_inbox_message_id
    }

    pub fn last_read_outbox_message_id(&self) -> i64 {
        self.last_read_outbox_message_id
    }

    pub fn unread_mention_count(&self) -> i32 {
        self.unread_mention_count
    }

    pub fn unread_reaction_count(&self) -> i32 {
        self.unread_reaction_count
    }

    pub fn notification_settings(&self) -> &ChatNotificationSettings {
        &self.notification_settings
    }

    pub fn draft_message(&self) -> &Option<DraftMessage> {
        &self.draft_message
    }
}

#[doc(hidden)]
pub struct ForumTopicBuilder {
    inner: ForumTopic,
}

#[deprecated]
pub type RTDForumTopicBuilder = ForumTopicBuilder;

impl ForumTopicBuilder {
    pub fn build(&self) -> ForumTopic {
        self.inner.clone()
    }

    pub fn info<T: AsRef<ForumTopicInfo>>(&mut self, info: T) -> &mut Self {
        self.inner.info = info.as_ref().clone();
        self
    }

    pub fn last_message<T: AsRef<Message>>(&mut self, last_message: T) -> &mut Self {
        self.inner.last_message = Some(last_message.as_ref().clone());
        self
    }

    pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
        self.inner.is_pinned = is_pinned;
        self
    }

    pub fn unread_count(&mut self, unread_count: i32) -> &mut Self {
        self.inner.unread_count = unread_count;
        self
    }

    pub fn last_read_inbox_message_id(&mut self, last_read_inbox_message_id: i64) -> &mut Self {
        self.inner.last_read_inbox_message_id = last_read_inbox_message_id;
        self
    }

    pub fn last_read_outbox_message_id(&mut self, last_read_outbox_message_id: i64) -> &mut Self {
        self.inner.last_read_outbox_message_id = last_read_outbox_message_id;
        self
    }

    pub fn unread_mention_count(&mut self, unread_mention_count: i32) -> &mut Self {
        self.inner.unread_mention_count = unread_mention_count;
        self
    }

    pub fn unread_reaction_count(&mut self, unread_reaction_count: i32) -> &mut Self {
        self.inner.unread_reaction_count = unread_reaction_count;
        self
    }

    pub fn notification_settings<T: AsRef<ChatNotificationSettings>>(
        &mut self,
        notification_settings: T,
    ) -> &mut Self {
        self.inner.notification_settings = notification_settings.as_ref().clone();
        self
    }

    pub fn draft_message<T: AsRef<DraftMessage>>(&mut self, draft_message: T) -> &mut Self {
        self.inner.draft_message = Some(draft_message.as_ref().clone());
        self
    }
}

impl AsRef<ForumTopic> for ForumTopic {
    fn as_ref(&self) -> &ForumTopic {
        self
    }
}

impl AsRef<ForumTopic> for ForumTopicBuilder {
    fn as_ref(&self) -> &ForumTopic {
        &self.inner
    }
}
