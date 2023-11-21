use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns identifiers of pinned or always included chats from a chat folder, which are suggested to be left when the chat folder is deleted
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatFolderChatsToLeave {
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

impl RObject for GetChatFolderChatsToLeave {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetChatFolderChatsToLeave {}

impl GetChatFolderChatsToLeave {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetChatFolderChatsToLeaveBuilder {
        let mut inner = GetChatFolderChatsToLeave::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getChatFolderChatsToLeave".to_string();

        GetChatFolderChatsToLeaveBuilder { inner }
    }

    pub fn chat_folder_id(&self) -> i32 {
        self.chat_folder_id
    }
}

#[doc(hidden)]
pub struct GetChatFolderChatsToLeaveBuilder {
    inner: GetChatFolderChatsToLeave,
}

#[deprecated]
pub type RTDGetChatFolderChatsToLeaveBuilder = GetChatFolderChatsToLeaveBuilder;

impl GetChatFolderChatsToLeaveBuilder {
    pub fn build(&self) -> GetChatFolderChatsToLeave {
        self.inner.clone()
    }

    pub fn chat_folder_id(&mut self, chat_folder_id: i32) -> &mut Self {
        self.inner.chat_folder_id = chat_folder_id;
        self
    }
}

impl AsRef<GetChatFolderChatsToLeave> for GetChatFolderChatsToLeave {
    fn as_ref(&self) -> &GetChatFolderChatsToLeave {
        self
    }
}

impl AsRef<GetChatFolderChatsToLeave> for GetChatFolderChatsToLeaveBuilder {
    fn as_ref(&self) -> &GetChatFolderChatsToLeave {
        &self.inner
    }
}
