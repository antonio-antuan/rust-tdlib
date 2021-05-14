use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Changes the discussion group of a channel chat; requires can_change_info rights in the channel if it is specified
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetChatDiscussionGroup {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the channel chat. Pass 0 to remove a link from the supergroup passed in the second argument to a linked channel chat (requires can_pin_messages rights in the supergroup)
    chat_id: i64,
    /// Identifier of a new channel's discussion group. Use 0 to remove the discussion group. Use the method getSuitableDiscussionChats to find all suitable groups. Basic group chats must be first upgraded to supergroup chats. If new chat members don't have access to old messages in the supergroup, then toggleSupergroupIsAllHistoryAvailable must be used first to change that
    discussion_chat_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetChatDiscussionGroup {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetChatDiscussionGroup {}

impl SetChatDiscussionGroup {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSetChatDiscussionGroupBuilder {
        let mut inner = SetChatDiscussionGroup::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setChatDiscussionGroup".to_string();

        RTDSetChatDiscussionGroupBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn discussion_chat_id(&self) -> i64 {
        self.discussion_chat_id
    }
}

#[doc(hidden)]
pub struct RTDSetChatDiscussionGroupBuilder {
    inner: SetChatDiscussionGroup,
}

impl RTDSetChatDiscussionGroupBuilder {
    pub fn build(&self) -> SetChatDiscussionGroup {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn discussion_chat_id(&mut self, discussion_chat_id: i64) -> &mut Self {
        self.inner.discussion_chat_id = discussion_chat_id;
        self
    }
}

impl AsRef<SetChatDiscussionGroup> for SetChatDiscussionGroup {
    fn as_ref(&self) -> &SetChatDiscussionGroup {
        self
    }
}

impl AsRef<SetChatDiscussionGroup> for RTDSetChatDiscussionGroupBuilder {
    fn as_ref(&self) -> &SetChatDiscussionGroup {
        &self.inner
    }
}
