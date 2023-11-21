use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Adds a chat folder by an invite link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddChatFolderByInviteLink {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Invite link for the chat folder

    #[serde(default)]
    invite_link: String,
    /// Identifiers of the chats added to the chat folder. The chats are automatically joined if they aren't joined yet

    #[serde(default)]
    chat_ids: Vec<i64>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for AddChatFolderByInviteLink {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for AddChatFolderByInviteLink {}

impl AddChatFolderByInviteLink {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AddChatFolderByInviteLinkBuilder {
        let mut inner = AddChatFolderByInviteLink::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "addChatFolderByInviteLink".to_string();

        AddChatFolderByInviteLinkBuilder { inner }
    }

    pub fn invite_link(&self) -> &String {
        &self.invite_link
    }

    pub fn chat_ids(&self) -> &Vec<i64> {
        &self.chat_ids
    }
}

#[doc(hidden)]
pub struct AddChatFolderByInviteLinkBuilder {
    inner: AddChatFolderByInviteLink,
}

#[deprecated]
pub type RTDAddChatFolderByInviteLinkBuilder = AddChatFolderByInviteLinkBuilder;

impl AddChatFolderByInviteLinkBuilder {
    pub fn build(&self) -> AddChatFolderByInviteLink {
        self.inner.clone()
    }

    pub fn invite_link<T: AsRef<str>>(&mut self, invite_link: T) -> &mut Self {
        self.inner.invite_link = invite_link.as_ref().to_string();
        self
    }

    pub fn chat_ids(&mut self, chat_ids: Vec<i64>) -> &mut Self {
        self.inner.chat_ids = chat_ids;
        self
    }
}

impl AsRef<AddChatFolderByInviteLink> for AddChatFolderByInviteLink {
    fn as_ref(&self) -> &AddChatFolderByInviteLink {
        self
    }
}

impl AsRef<AddChatFolderByInviteLink> for AddChatFolderByInviteLinkBuilder {
    fn as_ref(&self) -> &AddChatFolderByInviteLink {
        &self.inner
    }
}
