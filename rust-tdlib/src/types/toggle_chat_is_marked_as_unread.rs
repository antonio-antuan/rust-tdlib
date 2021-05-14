use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Changes the marked as unread state of a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToggleChatIsMarkedAsUnread {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier
    chat_id: i64,
    /// New value of is_marked_as_unread
    is_marked_as_unread: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ToggleChatIsMarkedAsUnread {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ToggleChatIsMarkedAsUnread {}

impl ToggleChatIsMarkedAsUnread {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDToggleChatIsMarkedAsUnreadBuilder {
        let mut inner = ToggleChatIsMarkedAsUnread::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "toggleChatIsMarkedAsUnread".to_string();

        RTDToggleChatIsMarkedAsUnreadBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn is_marked_as_unread(&self) -> bool {
        self.is_marked_as_unread
    }
}

#[doc(hidden)]
pub struct RTDToggleChatIsMarkedAsUnreadBuilder {
    inner: ToggleChatIsMarkedAsUnread,
}

impl RTDToggleChatIsMarkedAsUnreadBuilder {
    pub fn build(&self) -> ToggleChatIsMarkedAsUnread {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn is_marked_as_unread(&mut self, is_marked_as_unread: bool) -> &mut Self {
        self.inner.is_marked_as_unread = is_marked_as_unread;
        self
    }
}

impl AsRef<ToggleChatIsMarkedAsUnread> for ToggleChatIsMarkedAsUnread {
    fn as_ref(&self) -> &ToggleChatIsMarkedAsUnread {
        self
    }
}

impl AsRef<ToggleChatIsMarkedAsUnread> for RTDToggleChatIsMarkedAsUnreadBuilder {
    fn as_ref(&self) -> &ToggleChatIsMarkedAsUnread {
        &self.inner
    }
}
