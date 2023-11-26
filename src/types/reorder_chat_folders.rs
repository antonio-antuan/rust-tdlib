use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes the order of chat folders
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReorderChatFolders {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifiers of chat folders in the new correct order

    #[serde(default)]
    chat_folder_ids: Vec<i32>,
    /// Position of the main chat list among chat folders, 0-based. Can be non-zero only for Premium users

    #[serde(default)]
    main_chat_list_position: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ReorderChatFolders {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ReorderChatFolders {}

impl ReorderChatFolders {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ReorderChatFoldersBuilder {
        let mut inner = ReorderChatFolders::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "reorderChatFolders".to_string();

        ReorderChatFoldersBuilder { inner }
    }

    pub fn chat_folder_ids(&self) -> &Vec<i32> {
        &self.chat_folder_ids
    }

    pub fn main_chat_list_position(&self) -> i32 {
        self.main_chat_list_position
    }
}

#[doc(hidden)]
pub struct ReorderChatFoldersBuilder {
    inner: ReorderChatFolders,
}

#[deprecated]
pub type RTDReorderChatFoldersBuilder = ReorderChatFoldersBuilder;

impl ReorderChatFoldersBuilder {
    pub fn build(&self) -> ReorderChatFolders {
        self.inner.clone()
    }

    pub fn chat_folder_ids(&mut self, chat_folder_ids: Vec<i32>) -> &mut Self {
        self.inner.chat_folder_ids = chat_folder_ids;
        self
    }

    pub fn main_chat_list_position(&mut self, main_chat_list_position: i32) -> &mut Self {
        self.inner.main_chat_list_position = main_chat_list_position;
        self
    }
}

impl AsRef<ReorderChatFolders> for ReorderChatFolders {
    fn as_ref(&self) -> &ReorderChatFolders {
        self
    }
}

impl AsRef<ReorderChatFolders> for ReorderChatFoldersBuilder {
    fn as_ref(&self) -> &ReorderChatFolders {
        &self.inner
    }
}
