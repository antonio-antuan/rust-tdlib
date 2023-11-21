use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains basic information about a forum topic
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ForumTopicInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Message thread identifier of the topic

    #[serde(default)]
    message_thread_id: i64,
    /// Name of the topic

    #[serde(default)]
    name: String,
    /// Icon of the topic
    icon: ForumTopicIcon,
    /// Point in time (Unix timestamp) when the topic was created

    #[serde(default)]
    creation_date: i32,
    /// Identifier of the creator of the topic

    #[serde(skip_serializing_if = "MessageSender::_is_default")]
    creator_id: MessageSender,
    /// True, if the topic is the General topic list

    #[serde(default)]
    is_general: bool,
    /// True, if the topic was created by the current user

    #[serde(default)]
    is_outgoing: bool,
    /// True, if the topic is closed

    #[serde(default)]
    is_closed: bool,
    /// True, if the topic is hidden above the topic list and closed; for General topic only

    #[serde(default)]
    is_hidden: bool,
}

impl RObject for ForumTopicInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ForumTopicInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ForumTopicInfoBuilder {
        let mut inner = ForumTopicInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ForumTopicInfoBuilder { inner }
    }

    pub fn message_thread_id(&self) -> i64 {
        self.message_thread_id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn icon(&self) -> &ForumTopicIcon {
        &self.icon
    }

    pub fn creation_date(&self) -> i32 {
        self.creation_date
    }

    pub fn creator_id(&self) -> &MessageSender {
        &self.creator_id
    }

    pub fn is_general(&self) -> bool {
        self.is_general
    }

    pub fn is_outgoing(&self) -> bool {
        self.is_outgoing
    }

    pub fn is_closed(&self) -> bool {
        self.is_closed
    }

    pub fn is_hidden(&self) -> bool {
        self.is_hidden
    }
}

#[doc(hidden)]
pub struct ForumTopicInfoBuilder {
    inner: ForumTopicInfo,
}

#[deprecated]
pub type RTDForumTopicInfoBuilder = ForumTopicInfoBuilder;

impl ForumTopicInfoBuilder {
    pub fn build(&self) -> ForumTopicInfo {
        self.inner.clone()
    }

    pub fn message_thread_id(&mut self, message_thread_id: i64) -> &mut Self {
        self.inner.message_thread_id = message_thread_id;
        self
    }

    pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
        self.inner.name = name.as_ref().to_string();
        self
    }

    pub fn icon<T: AsRef<ForumTopicIcon>>(&mut self, icon: T) -> &mut Self {
        self.inner.icon = icon.as_ref().clone();
        self
    }

    pub fn creation_date(&mut self, creation_date: i32) -> &mut Self {
        self.inner.creation_date = creation_date;
        self
    }

    pub fn creator_id<T: AsRef<MessageSender>>(&mut self, creator_id: T) -> &mut Self {
        self.inner.creator_id = creator_id.as_ref().clone();
        self
    }

    pub fn is_general(&mut self, is_general: bool) -> &mut Self {
        self.inner.is_general = is_general;
        self
    }

    pub fn is_outgoing(&mut self, is_outgoing: bool) -> &mut Self {
        self.inner.is_outgoing = is_outgoing;
        self
    }

    pub fn is_closed(&mut self, is_closed: bool) -> &mut Self {
        self.inner.is_closed = is_closed;
        self
    }

    pub fn is_hidden(&mut self, is_hidden: bool) -> &mut Self {
        self.inner.is_hidden = is_hidden;
        self
    }
}

impl AsRef<ForumTopicInfo> for ForumTopicInfo {
    fn as_ref(&self) -> &ForumTopicInfo {
        self
    }
}

impl AsRef<ForumTopicInfo> for ForumTopicInfoBuilder {
    fn as_ref(&self) -> &ForumTopicInfo {
        &self.inner
    }
}
