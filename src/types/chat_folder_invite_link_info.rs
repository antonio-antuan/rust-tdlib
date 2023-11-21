use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains information about an invite link to a chat folder
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatFolderInviteLinkInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Basic information about the chat folder; chat folder identifier will be 0 if the user didn't have the chat folder yet
    chat_folder_info: ChatFolderInfo,
    /// Identifiers of the chats from the link, which aren't added to the folder yet

    #[serde(default)]
    missing_chat_ids: Vec<i64>,
    /// Identifiers of the chats from the link, which are added to the folder already

    #[serde(default)]
    added_chat_ids: Vec<i64>,
}

impl RObject for ChatFolderInviteLinkInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatFolderInviteLinkInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatFolderInviteLinkInfoBuilder {
        let mut inner = ChatFolderInviteLinkInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatFolderInviteLinkInfoBuilder { inner }
    }

    pub fn chat_folder_info(&self) -> &ChatFolderInfo {
        &self.chat_folder_info
    }

    pub fn missing_chat_ids(&self) -> &Vec<i64> {
        &self.missing_chat_ids
    }

    pub fn added_chat_ids(&self) -> &Vec<i64> {
        &self.added_chat_ids
    }
}

#[doc(hidden)]
pub struct ChatFolderInviteLinkInfoBuilder {
    inner: ChatFolderInviteLinkInfo,
}

#[deprecated]
pub type RTDChatFolderInviteLinkInfoBuilder = ChatFolderInviteLinkInfoBuilder;

impl ChatFolderInviteLinkInfoBuilder {
    pub fn build(&self) -> ChatFolderInviteLinkInfo {
        self.inner.clone()
    }

    pub fn chat_folder_info<T: AsRef<ChatFolderInfo>>(&mut self, chat_folder_info: T) -> &mut Self {
        self.inner.chat_folder_info = chat_folder_info.as_ref().clone();
        self
    }

    pub fn missing_chat_ids(&mut self, missing_chat_ids: Vec<i64>) -> &mut Self {
        self.inner.missing_chat_ids = missing_chat_ids;
        self
    }

    pub fn added_chat_ids(&mut self, added_chat_ids: Vec<i64>) -> &mut Self {
        self.inner.added_chat_ids = added_chat_ids;
        self
    }
}

impl AsRef<ChatFolderInviteLinkInfo> for ChatFolderInviteLinkInfo {
    fn as_ref(&self) -> &ChatFolderInviteLinkInfo {
        self
    }
}

impl AsRef<ChatFolderInviteLinkInfo> for ChatFolderInviteLinkInfoBuilder {
    fn as_ref(&self) -> &ChatFolderInviteLinkInfo {
        &self.inner
    }
}
