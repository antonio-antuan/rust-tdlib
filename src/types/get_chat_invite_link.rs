use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns information about an invite link. Requires administrator privileges and can_invite_users right in the chat to get own links and owner privileges to get other links
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatInviteLink {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// Invite link to get

    #[serde(default)]
    invite_link: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetChatInviteLink {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetChatInviteLink {}

impl GetChatInviteLink {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetChatInviteLinkBuilder {
        let mut inner = GetChatInviteLink::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getChatInviteLink".to_string();

        GetChatInviteLinkBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn invite_link(&self) -> &String {
        &self.invite_link
    }
}

#[doc(hidden)]
pub struct GetChatInviteLinkBuilder {
    inner: GetChatInviteLink,
}

#[deprecated]
pub type RTDGetChatInviteLinkBuilder = GetChatInviteLinkBuilder;

impl GetChatInviteLinkBuilder {
    pub fn build(&self) -> GetChatInviteLink {
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
}

impl AsRef<GetChatInviteLink> for GetChatInviteLink {
    fn as_ref(&self) -> &GetChatInviteLink {
        self
    }
}

impl AsRef<GetChatInviteLink> for GetChatInviteLinkBuilder {
    fn as_ref(&self) -> &GetChatInviteLink {
        &self.inner
    }
}
