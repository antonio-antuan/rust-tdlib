use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Changes the pinned state of a chat. There can be up to GetOption("pinned_chat_count_max")/GetOption("pinned_archived_chat_count_max") pinned non-secret chats and the same number of secret chats in the main/arhive chat list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToggleChatIsPinned {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat list in which to change the pinned state of the chat

    #[serde(skip_serializing_if = "ChatList::_is_default")]
    chat_list: ChatList,
    /// Chat identifier
    chat_id: i64,
    /// True, if the chat is pinned
    is_pinned: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ToggleChatIsPinned {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ToggleChatIsPinned {}

impl ToggleChatIsPinned {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDToggleChatIsPinnedBuilder {
        let mut inner = ToggleChatIsPinned::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "toggleChatIsPinned".to_string();

        RTDToggleChatIsPinnedBuilder { inner }
    }

    pub fn chat_list(&self) -> &ChatList {
        &self.chat_list
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn is_pinned(&self) -> bool {
        self.is_pinned
    }
}

#[doc(hidden)]
pub struct RTDToggleChatIsPinnedBuilder {
    inner: ToggleChatIsPinned,
}

impl RTDToggleChatIsPinnedBuilder {
    pub fn build(&self) -> ToggleChatIsPinned {
        self.inner.clone()
    }

    pub fn chat_list<T: AsRef<ChatList>>(&mut self, chat_list: T) -> &mut Self {
        self.inner.chat_list = chat_list.as_ref().clone();
        self
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
        self.inner.is_pinned = is_pinned;
        self
    }
}

impl AsRef<ToggleChatIsPinned> for ToggleChatIsPinned {
    fn as_ref(&self) -> &ToggleChatIsPinned {
        self
    }
}

impl AsRef<ToggleChatIsPinned> for RTDToggleChatIsPinnedBuilder {
    fn as_ref(&self) -> &ToggleChatIsPinned {
        &self.inner
    }
}
