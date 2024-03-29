use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Uses an invite link to add the current user to the chat if possible
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JoinChatByInviteLink {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Invite link to use

    #[serde(default)]
    invite_link: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for JoinChatByInviteLink {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for JoinChatByInviteLink {}

impl JoinChatByInviteLink {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> JoinChatByInviteLinkBuilder {
        let mut inner = JoinChatByInviteLink::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "joinChatByInviteLink".to_string();

        JoinChatByInviteLinkBuilder { inner }
    }

    pub fn invite_link(&self) -> &String {
        &self.invite_link
    }
}

#[doc(hidden)]
pub struct JoinChatByInviteLinkBuilder {
    inner: JoinChatByInviteLink,
}

#[deprecated]
pub type RTDJoinChatByInviteLinkBuilder = JoinChatByInviteLinkBuilder;

impl JoinChatByInviteLinkBuilder {
    pub fn build(&self) -> JoinChatByInviteLink {
        self.inner.clone()
    }

    pub fn invite_link<T: AsRef<str>>(&mut self, invite_link: T) -> &mut Self {
        self.inner.invite_link = invite_link.as_ref().to_string();
        self
    }
}

impl AsRef<JoinChatByInviteLink> for JoinChatByInviteLink {
    fn as_ref(&self) -> &JoinChatByInviteLink {
        self
    }
}

impl AsRef<JoinChatByInviteLink> for JoinChatByInviteLinkBuilder {
    fn as_ref(&self) -> &JoinChatByInviteLink {
        &self.inner
    }
}
