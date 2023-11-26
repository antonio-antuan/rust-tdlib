use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns identifiers of chats from a chat folder, suitable for adding to a chat folder invite link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatsForChatFolderInviteLink {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat folder identifier

    #[serde(default)]
    chat_folder_id: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetChatsForChatFolderInviteLink {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetChatsForChatFolderInviteLink {}

impl GetChatsForChatFolderInviteLink {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetChatsForChatFolderInviteLinkBuilder {
        let mut inner = GetChatsForChatFolderInviteLink::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getChatsForChatFolderInviteLink".to_string();

        GetChatsForChatFolderInviteLinkBuilder { inner }
    }

    pub fn chat_folder_id(&self) -> i32 {
        self.chat_folder_id
    }
}

#[doc(hidden)]
pub struct GetChatsForChatFolderInviteLinkBuilder {
    inner: GetChatsForChatFolderInviteLink,
}

#[deprecated]
pub type RTDGetChatsForChatFolderInviteLinkBuilder = GetChatsForChatFolderInviteLinkBuilder;

impl GetChatsForChatFolderInviteLinkBuilder {
    pub fn build(&self) -> GetChatsForChatFolderInviteLink {
        self.inner.clone()
    }

    pub fn chat_folder_id(&mut self, chat_folder_id: i32) -> &mut Self {
        self.inner.chat_folder_id = chat_folder_id;
        self
    }
}

impl AsRef<GetChatsForChatFolderInviteLink> for GetChatsForChatFolderInviteLink {
    fn as_ref(&self) -> &GetChatsForChatFolderInviteLink {
        self
    }
}

impl AsRef<GetChatsForChatFolderInviteLink> for GetChatsForChatFolderInviteLinkBuilder {
    fn as_ref(&self) -> &GetChatsForChatFolderInviteLink {
        &self.inner
    }
}
