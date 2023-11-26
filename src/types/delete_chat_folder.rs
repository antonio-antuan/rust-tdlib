use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Deletes existing chat folder
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteChatFolder {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat folder identifier

    #[serde(default)]
    chat_folder_id: i32,
    /// Identifiers of the chats to leave. The chats must be pinned or always included in the folder

    #[serde(default)]
    leave_chat_ids: Vec<i64>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for DeleteChatFolder {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for DeleteChatFolder {}

impl DeleteChatFolder {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> DeleteChatFolderBuilder {
        let mut inner = DeleteChatFolder::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "deleteChatFolder".to_string();

        DeleteChatFolderBuilder { inner }
    }

    pub fn chat_folder_id(&self) -> i32 {
        self.chat_folder_id
    }

    pub fn leave_chat_ids(&self) -> &Vec<i64> {
        &self.leave_chat_ids
    }
}

#[doc(hidden)]
pub struct DeleteChatFolderBuilder {
    inner: DeleteChatFolder,
}

#[deprecated]
pub type RTDDeleteChatFolderBuilder = DeleteChatFolderBuilder;

impl DeleteChatFolderBuilder {
    pub fn build(&self) -> DeleteChatFolder {
        self.inner.clone()
    }

    pub fn chat_folder_id(&mut self, chat_folder_id: i32) -> &mut Self {
        self.inner.chat_folder_id = chat_folder_id;
        self
    }

    pub fn leave_chat_ids(&mut self, leave_chat_ids: Vec<i64>) -> &mut Self {
        self.inner.leave_chat_ids = leave_chat_ids;
        self
    }
}

impl AsRef<DeleteChatFolder> for DeleteChatFolder {
    fn as_ref(&self) -> &DeleteChatFolder {
        self
    }
}

impl AsRef<DeleteChatFolder> for DeleteChatFolderBuilder {
    fn as_ref(&self) -> &DeleteChatFolder {
        &self.inner
    }
}
