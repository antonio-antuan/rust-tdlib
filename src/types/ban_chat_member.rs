use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Bans a member in a chat. Members can't be banned in private or secret chats. In supergroups and channels, the user will not be able to return to the group on their own using invite links, etc., unless unbanned first
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BanChatMember {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// Member identifier

    #[serde(skip_serializing_if = "MessageSender::_is_default")]
    member_id: MessageSender,
    /// Point in time (Unix timestamp) when the user will be unbanned; 0 if never. If the user is banned for more than 366 days or for less than 30 seconds from the current time, the user is considered to be banned forever. Ignored in basic groups and if a chat is banned

    #[serde(default)]
    banned_until_date: i32,
    /// Pass true to delete all messages in the chat for the user that is being removed. Always true for supergroups and channels

    #[serde(default)]
    revoke_messages: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for BanChatMember {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for BanChatMember {}

impl BanChatMember {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> BanChatMemberBuilder {
        let mut inner = BanChatMember::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "banChatMember".to_string();

        BanChatMemberBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn member_id(&self) -> &MessageSender {
        &self.member_id
    }

    pub fn banned_until_date(&self) -> i32 {
        self.banned_until_date
    }

    pub fn revoke_messages(&self) -> bool {
        self.revoke_messages
    }
}

#[doc(hidden)]
pub struct BanChatMemberBuilder {
    inner: BanChatMember,
}

#[deprecated]
pub type RTDBanChatMemberBuilder = BanChatMemberBuilder;

impl BanChatMemberBuilder {
    pub fn build(&self) -> BanChatMember {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn member_id<T: AsRef<MessageSender>>(&mut self, member_id: T) -> &mut Self {
        self.inner.member_id = member_id.as_ref().clone();
        self
    }

    pub fn banned_until_date(&mut self, banned_until_date: i32) -> &mut Self {
        self.inner.banned_until_date = banned_until_date;
        self
    }

    pub fn revoke_messages(&mut self, revoke_messages: bool) -> &mut Self {
        self.inner.revoke_messages = revoke_messages;
        self
    }
}

impl AsRef<BanChatMember> for BanChatMember {
    fn as_ref(&self) -> &BanChatMember {
        self
    }
}

impl AsRef<BanChatMember> for BanChatMemberBuilder {
    fn as_ref(&self) -> &BanChatMember {
        &self.inner
    }
}
