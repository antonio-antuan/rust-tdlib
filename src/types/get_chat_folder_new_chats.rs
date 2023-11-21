use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns new chats added to a shareable chat folder by its owner. The method must be called at most once in getOption("chat_folder_new_chats_update_period") for the given chat folder
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatFolderNewChats {
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

impl RObject for GetChatFolderNewChats {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetChatFolderNewChats {}

impl GetChatFolderNewChats {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetChatFolderNewChatsBuilder {
        let mut inner = GetChatFolderNewChats::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getChatFolderNewChats".to_string();

        GetChatFolderNewChatsBuilder { inner }
    }

    pub fn chat_folder_id(&self) -> i32 {
        self.chat_folder_id
    }
}

#[doc(hidden)]
pub struct GetChatFolderNewChatsBuilder {
    inner: GetChatFolderNewChats,
}

#[deprecated]
pub type RTDGetChatFolderNewChatsBuilder = GetChatFolderNewChatsBuilder;

impl GetChatFolderNewChatsBuilder {
    pub fn build(&self) -> GetChatFolderNewChats {
        self.inner.clone()
    }

    pub fn chat_folder_id(&mut self, chat_folder_id: i32) -> &mut Self {
        self.inner.chat_folder_id = chat_folder_id;
        self
    }
}

impl AsRef<GetChatFolderNewChats> for GetChatFolderNewChats {
    fn as_ref(&self) -> &GetChatFolderNewChats {
        self
    }
}

impl AsRef<GetChatFolderNewChats> for GetChatFolderNewChatsBuilder {
    fn as_ref(&self) -> &GetChatFolderNewChats {
        &self.inner
    }
}
