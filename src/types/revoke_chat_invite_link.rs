use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Revokes invite link for a chat. Available for basic groups, supergroups, and channels. Requires administrator privileges and can_invite_users right in the chat for own links and owner privileges for other links. If a primary link is revoked, then additionally to the revoked link returns new primary link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RevokeChatInviteLink {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// Invite link to be revoked

    #[serde(default)]
    invite_link: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for RevokeChatInviteLink {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for RevokeChatInviteLink {}

impl RevokeChatInviteLink {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RevokeChatInviteLinkBuilder {
        let mut inner = RevokeChatInviteLink::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "revokeChatInviteLink".to_string();

        RevokeChatInviteLinkBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn invite_link(&self) -> &String {
        &self.invite_link
    }
}

#[doc(hidden)]
pub struct RevokeChatInviteLinkBuilder {
    inner: RevokeChatInviteLink,
}

#[deprecated]
pub type RTDRevokeChatInviteLinkBuilder = RevokeChatInviteLinkBuilder;

impl RevokeChatInviteLinkBuilder {
    pub fn build(&self) -> RevokeChatInviteLink {
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

impl AsRef<RevokeChatInviteLink> for RevokeChatInviteLink {
    fn as_ref(&self) -> &RevokeChatInviteLink {
        self
    }
}

impl AsRef<RevokeChatInviteLink> for RevokeChatInviteLinkBuilder {
    fn as_ref(&self) -> &RevokeChatInviteLink {
        &self.inner
    }
}
