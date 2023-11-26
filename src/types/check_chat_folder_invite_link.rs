use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Checks the validity of an invite link for a chat folder and returns information about the corresponding chat folder
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckChatFolderInviteLink {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Invite link to be checked

    #[serde(default)]
    invite_link: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for CheckChatFolderInviteLink {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CheckChatFolderInviteLink {}

impl CheckChatFolderInviteLink {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CheckChatFolderInviteLinkBuilder {
        let mut inner = CheckChatFolderInviteLink::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "checkChatFolderInviteLink".to_string();

        CheckChatFolderInviteLinkBuilder { inner }
    }

    pub fn invite_link(&self) -> &String {
        &self.invite_link
    }
}

#[doc(hidden)]
pub struct CheckChatFolderInviteLinkBuilder {
    inner: CheckChatFolderInviteLink,
}

#[deprecated]
pub type RTDCheckChatFolderInviteLinkBuilder = CheckChatFolderInviteLinkBuilder;

impl CheckChatFolderInviteLinkBuilder {
    pub fn build(&self) -> CheckChatFolderInviteLink {
        self.inner.clone()
    }

    pub fn invite_link<T: AsRef<str>>(&mut self, invite_link: T) -> &mut Self {
        self.inner.invite_link = invite_link.as_ref().to_string();
        self
    }
}

impl AsRef<CheckChatFolderInviteLink> for CheckChatFolderInviteLink {
    fn as_ref(&self) -> &CheckChatFolderInviteLink {
        self
    }
}

impl AsRef<CheckChatFolderInviteLink> for CheckChatFolderInviteLinkBuilder {
    fn as_ref(&self) -> &CheckChatFolderInviteLink {
        &self.inner
    }
}
