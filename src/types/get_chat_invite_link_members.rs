use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns chat members joined a chat via an invite link. Requires administrator privileges and can_invite_users right in the chat for own links and owner privileges for other links
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatInviteLinkMembers {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// Invite link for which to return chat members

    #[serde(default)]
    invite_link: String,
    /// A chat member from which to return next chat members; pass null to get results from the beginning
    offset_member: ChatInviteLinkMember,
    /// The maximum number of chat members to return; up to 100

    #[serde(default)]
    limit: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetChatInviteLinkMembers {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetChatInviteLinkMembers {}

impl GetChatInviteLinkMembers {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetChatInviteLinkMembersBuilder {
        let mut inner = GetChatInviteLinkMembers::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getChatInviteLinkMembers".to_string();

        GetChatInviteLinkMembersBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn invite_link(&self) -> &String {
        &self.invite_link
    }

    pub fn offset_member(&self) -> &ChatInviteLinkMember {
        &self.offset_member
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }
}

#[doc(hidden)]
pub struct GetChatInviteLinkMembersBuilder {
    inner: GetChatInviteLinkMembers,
}

#[deprecated]
pub type RTDGetChatInviteLinkMembersBuilder = GetChatInviteLinkMembersBuilder;

impl GetChatInviteLinkMembersBuilder {
    pub fn build(&self) -> GetChatInviteLinkMembers {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn invite_link<T: AsRef<str>>(&mut self, invite_link: T) -> &mut Self {
        self.inner.invite_link = invite_link.as_ref().to_string();
        self
    }

    pub fn offset_member<T: AsRef<ChatInviteLinkMember>>(&mut self, offset_member: T) -> &mut Self {
        self.inner.offset_member = offset_member.as_ref().clone();
        self
    }

    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self.inner.limit = limit;
        self
    }
}

impl AsRef<GetChatInviteLinkMembers> for GetChatInviteLinkMembers {
    fn as_ref(&self) -> &GetChatInviteLinkMembers {
        self
    }
}

impl AsRef<GetChatInviteLinkMembers> for GetChatInviteLinkMembersBuilder {
    fn as_ref(&self) -> &GetChatInviteLinkMembers {
        &self.inner
    }
}
