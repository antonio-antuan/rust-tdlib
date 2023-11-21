use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Edits existing chat folder. Returns information about the edited chat folder
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EditChatFolder {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat folder identifier

    #[serde(default)]
    chat_folder_id: i32,
    /// The edited chat folder
    folder: ChatFolder,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for EditChatFolder {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for EditChatFolder {}

impl EditChatFolder {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> EditChatFolderBuilder {
        let mut inner = EditChatFolder::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "editChatFolder".to_string();

        EditChatFolderBuilder { inner }
    }

    pub fn chat_folder_id(&self) -> i32 {
        self.chat_folder_id
    }

    pub fn folder(&self) -> &ChatFolder {
        &self.folder
    }
}

#[doc(hidden)]
pub struct EditChatFolderBuilder {
    inner: EditChatFolder,
}

#[deprecated]
pub type RTDEditChatFolderBuilder = EditChatFolderBuilder;

impl EditChatFolderBuilder {
    pub fn build(&self) -> EditChatFolder {
        self.inner.clone()
    }

    pub fn chat_folder_id(&mut self, chat_folder_id: i32) -> &mut Self {
        self.inner.chat_folder_id = chat_folder_id;
        self
    }

    pub fn folder<T: AsRef<ChatFolder>>(&mut self, folder: T) -> &mut Self {
        self.inner.folder = folder.as_ref().clone();
        self
    }
}

impl AsRef<EditChatFolder> for EditChatFolder {
    fn as_ref(&self) -> &EditChatFolder {
        self
    }
}

impl AsRef<EditChatFolder> for EditChatFolderBuilder {
    fn as_ref(&self) -> &EditChatFolder {
        &self.inner
    }
}
