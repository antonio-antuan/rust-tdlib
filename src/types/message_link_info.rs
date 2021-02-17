use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a link to a message in a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageLinkInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// True, if the link is a public link for a message in a chat
    is_public: bool,
    /// If found, identifier of the chat to which the message belongs, 0 otherwise
    chat_id: i64,
    /// If found, the linked message; may be null
    message: Option<Message>,
    /// True, if the whole media album to which the message belongs is linked
    for_album: bool,
    /// True, if the message is linked as a channel post comment or from a message thread
    for_comment: bool,
}

impl RObject for MessageLinkInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl MessageLinkInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDMessageLinkInfoBuilder {
        let mut inner = MessageLinkInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDMessageLinkInfoBuilder { inner }
    }

    pub fn is_public(&self) -> bool {
        self.is_public
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message(&self) -> &Option<Message> {
        &self.message
    }

    pub fn for_album(&self) -> bool {
        self.for_album
    }

    pub fn for_comment(&self) -> bool {
        self.for_comment
    }
}

#[doc(hidden)]
pub struct RTDMessageLinkInfoBuilder {
    inner: MessageLinkInfo,
}

impl RTDMessageLinkInfoBuilder {
    pub fn build(&self) -> MessageLinkInfo {
        self.inner.clone()
    }

    pub fn is_public(&mut self, is_public: bool) -> &mut Self {
        self.inner.is_public = is_public;
        self
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message<T: AsRef<Message>>(&mut self, message: T) -> &mut Self {
        self.inner.message = Some(message.as_ref().clone());
        self
    }

    pub fn for_album(&mut self, for_album: bool) -> &mut Self {
        self.inner.for_album = for_album;
        self
    }

    pub fn for_comment(&mut self, for_comment: bool) -> &mut Self {
        self.inner.for_comment = for_comment;
        self
    }
}

impl AsRef<MessageLinkInfo> for MessageLinkInfo {
    fn as_ref(&self) -> &MessageLinkInfo {
        self
    }
}

impl AsRef<MessageLinkInfo> for RTDMessageLinkInfoBuilder {
    fn as_ref(&self) -> &MessageLinkInfo {
        &self.inner
    }
}
