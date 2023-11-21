use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains a chat folder invite link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatFolderInviteLink {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The chat folder invite link

    #[serde(default)]
    invite_link: String,
    /// Name of the link

    #[serde(default)]
    name: String,
    /// Identifiers of chats, included in the link

    #[serde(default)]
    chat_ids: Vec<i64>,
}

impl RObject for ChatFolderInviteLink {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatFolderInviteLink {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatFolderInviteLinkBuilder {
        let mut inner = ChatFolderInviteLink::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatFolderInviteLinkBuilder { inner }
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
pub struct ChatFolderInviteLinkBuilder {
    inner: ChatFolderInviteLink,
}

#[deprecated]
pub type RTDChatFolderInviteLinkBuilder = ChatFolderInviteLinkBuilder;

impl ChatFolderInviteLinkBuilder {
    pub fn build(&self) -> ChatFolderInviteLink {
        self.inner.clone()
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

impl AsRef<ChatFolderInviteLink> for ChatFolderInviteLink {
    fn as_ref(&self) -> &ChatFolderInviteLink {
        self
    }
}

impl AsRef<ChatFolderInviteLink> for ChatFolderInviteLinkBuilder {
    fn as_ref(&self) -> &ChatFolderInviteLink {
        &self.inner
    }
}
