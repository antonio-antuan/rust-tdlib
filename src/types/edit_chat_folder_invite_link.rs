use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Edits an invite link for a chat folder
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EditChatFolderInviteLink {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat folder identifier

    #[serde(default)]
    chat_folder_id: i32,
    /// Invite link to be edited

    #[serde(default)]
    invite_link: String,
    /// New name of the link; 0-32 characters

    #[serde(default)]
    name: String,
    /// New identifiers of chats to be accessible by the invite link. Use getChatsForChatFolderInviteLink to get suitable chats. Basic groups will be automatically converted to supergroups before link editing

    #[serde(default)]
    chat_ids: Vec<i64>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for EditChatFolderInviteLink {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for EditChatFolderInviteLink {}

impl EditChatFolderInviteLink {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> EditChatFolderInviteLinkBuilder {
        let mut inner = EditChatFolderInviteLink::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "editChatFolderInviteLink".to_string();

        EditChatFolderInviteLinkBuilder { inner }
    }

    pub fn chat_folder_id(&self) -> i32 {
        self.chat_folder_id
    }

    pub fn invite_link(&self) -> &String {
        &self.invite_link
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn chat_ids(&self) -> &Vec<i64> {
        &self.chat_ids
    }
}

#[doc(hidden)]
pub struct EditChatFolderInviteLinkBuilder {
    inner: EditChatFolderInviteLink,
}

#[deprecated]
pub type RTDEditChatFolderInviteLinkBuilder = EditChatFolderInviteLinkBuilder;

impl EditChatFolderInviteLinkBuilder {
    pub fn build(&self) -> EditChatFolderInviteLink {
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

    pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
        self.inner.name = name.as_ref().to_string();
        self
    }

    pub fn chat_ids(&mut self, chat_ids: Vec<i64>) -> &mut Self {
        self.inner.chat_ids = chat_ids;
        self
    }
}

impl AsRef<EditChatFolderInviteLink> for EditChatFolderInviteLink {
    fn as_ref(&self) -> &EditChatFolderInviteLink {
        self
    }
}

impl AsRef<EditChatFolderInviteLink> for EditChatFolderInviteLinkBuilder {
    fn as_ref(&self) -> &EditChatFolderInviteLink {
        &self.inner
    }
}
