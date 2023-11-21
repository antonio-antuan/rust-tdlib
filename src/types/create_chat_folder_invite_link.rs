use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Creates a new invite link for a chat folder. A link can be created for a chat folder if it has only pinned and included chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateChatFolderInviteLink {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat folder identifier

    #[serde(default)]
    chat_folder_id: i32,
    /// Name of the link; 0-32 characters

    #[serde(default)]
    name: String,
    /// Identifiers of chats to be accessible by the invite link. Use getChatsForChatFolderInviteLink to get suitable chats. Basic groups will be automatically converted to supergroups before link creation

    #[serde(default)]
    chat_ids: Vec<i64>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for CreateChatFolderInviteLink {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CreateChatFolderInviteLink {}

impl CreateChatFolderInviteLink {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CreateChatFolderInviteLinkBuilder {
        let mut inner = CreateChatFolderInviteLink::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "createChatFolderInviteLink".to_string();

        CreateChatFolderInviteLinkBuilder { inner }
    }

    pub fn chat_folder_id(&self) -> i32 {
        self.chat_folder_id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn chat_ids(&self) -> &Vec<i64> {
        &self.chat_ids
    }
}

#[doc(hidden)]
pub struct CreateChatFolderInviteLinkBuilder {
    inner: CreateChatFolderInviteLink,
}

#[deprecated]
pub type RTDCreateChatFolderInviteLinkBuilder = CreateChatFolderInviteLinkBuilder;

impl CreateChatFolderInviteLinkBuilder {
    pub fn build(&self) -> CreateChatFolderInviteLink {
        self.inner.clone()
    }

    pub fn chat_folder_id(&mut self, chat_folder_id: i32) -> &mut Self {
        self.inner.chat_folder_id = chat_folder_id;
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

impl AsRef<CreateChatFolderInviteLink> for CreateChatFolderInviteLink {
    fn as_ref(&self) -> &CreateChatFolderInviteLink {
        self
    }
}

impl AsRef<CreateChatFolderInviteLink> for CreateChatFolderInviteLinkBuilder {
    fn as_ref(&self) -> &CreateChatFolderInviteLink {
        &self.inner
    }
}
