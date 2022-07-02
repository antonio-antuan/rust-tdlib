use crate::errors::Result;
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

    #[serde(default)]
    chat_id: i64,
    /// New value of is_marked_as_unread

    #[serde(default)]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ToggleChatIsMarkedAsUnreadBuilder {
        let mut inner = ToggleChatIsMarkedAsUnread::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "toggleChatIsMarkedAsUnread".to_string();

        ToggleChatIsMarkedAsUnreadBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn is_marked_as_unread(&self) -> bool {
        self.is_marked_as_unread
    }
}

#[doc(hidden)]
pub struct ToggleChatIsMarkedAsUnreadBuilder {
    inner: ToggleChatIsMarkedAsUnread,
}

#[deprecated]
pub type RTDToggleChatIsMarkedAsUnreadBuilder = ToggleChatIsMarkedAsUnreadBuilder;

impl ToggleChatIsMarkedAsUnreadBuilder {
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

impl AsRef<ToggleChatIsMarkedAsUnread> for ToggleChatIsMarkedAsUnreadBuilder {
    fn as_ref(&self) -> &ToggleChatIsMarkedAsUnread {
        &self.inner
    }
}
