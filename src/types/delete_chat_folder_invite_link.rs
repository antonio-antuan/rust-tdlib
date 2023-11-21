use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Deletes an invite link for a chat folder
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteChatFolderInviteLink {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat folder identifier

    #[serde(default)]
    chat_folder_id: i32,
    /// Invite link to be deleted

    #[serde(default)]
    invite_link: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for DeleteChatFolderInviteLink {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for DeleteChatFolderInviteLink {}

impl DeleteChatFolderInviteLink {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> DeleteChatFolderInviteLinkBuilder {
        let mut inner = DeleteChatFolderInviteLink::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "deleteChatFolderInviteLink".to_string();

        DeleteChatFolderInviteLinkBuilder { inner }
    }

    pub fn chat_folder_id(&self) -> i32 {
        self.chat_folder_id
    }

    pub fn invite_link(&self) -> &String {
        &self.invite_link
    }
}

#[doc(hidden)]
pub struct DeleteChatFolderInviteLinkBuilder {
    inner: DeleteChatFolderInviteLink,
}

#[deprecated]
pub type RTDDeleteChatFolderInviteLinkBuilder = DeleteChatFolderInviteLinkBuilder;

impl DeleteChatFolderInviteLinkBuilder {
    pub fn build(&self) -> DeleteChatFolderInviteLink {
        self.inner.clone()
    }

    pub fn chat_folder_id(&mut self, chat_folder_id: i32) -> &mut Self {
        self.inner.chat_folder_id = chat_folder_id;
        self
    }

    pub fn invite_link<T: AsRef<str>>(&mut self, invite_link: T) -> &mut Self {
        self.inner.invite_link = invite_link.as_ref().to_string();
        self
    }
}

impl AsRef<DeleteChatFolderInviteLink> for DeleteChatFolderInviteLink {
    fn as_ref(&self) -> &DeleteChatFolderInviteLink {
        self
    }
}

impl AsRef<DeleteChatFolderInviteLink> for DeleteChatFolderInviteLinkBuilder {
    fn as_ref(&self) -> &DeleteChatFolderInviteLink {
        &self.inner
    }
}
