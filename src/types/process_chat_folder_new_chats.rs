use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Process new chats added to a shareable chat folder by its owner
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProcessChatFolderNewChats {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat folder identifier

    #[serde(default)]
    chat_folder_id: i32,
    /// Identifiers of the new chats, which are added to the chat folder. The chats are automatically joined if they aren't joined yet

    #[serde(default)]
    added_chat_ids: Vec<i64>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ProcessChatFolderNewChats {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ProcessChatFolderNewChats {}

impl ProcessChatFolderNewChats {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ProcessChatFolderNewChatsBuilder {
        let mut inner = ProcessChatFolderNewChats::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "processChatFolderNewChats".to_string();

        ProcessChatFolderNewChatsBuilder { inner }
    }

    pub fn chat_folder_id(&self) -> i32 {
        self.chat_folder_id
    }

    pub fn added_chat_ids(&self) -> &Vec<i64> {
        &self.added_chat_ids
    }
}

#[doc(hidden)]
pub struct ProcessChatFolderNewChatsBuilder {
    inner: ProcessChatFolderNewChats,
}

#[deprecated]
pub type RTDProcessChatFolderNewChatsBuilder = ProcessChatFolderNewChatsBuilder;

impl ProcessChatFolderNewChatsBuilder {
    pub fn build(&self) -> ProcessChatFolderNewChats {
        self.inner.clone()
    }

    pub fn chat_folder_id(&mut self, chat_folder_id: i32) -> &mut Self {
        self.inner.chat_folder_id = chat_folder_id;
        self
    }

    pub fn added_chat_ids(&mut self, added_chat_ids: Vec<i64>) -> &mut Self {
        self.inner.added_chat_ids = added_chat_ids;
        self
    }
}

impl AsRef<ProcessChatFolderNewChats> for ProcessChatFolderNewChats {
    fn as_ref(&self) -> &ProcessChatFolderNewChats {
        self
    }
}

impl AsRef<ProcessChatFolderNewChats> for ProcessChatFolderNewChatsBuilder {
    fn as_ref(&self) -> &ProcessChatFolderNewChats {
        &self.inner
    }
}
