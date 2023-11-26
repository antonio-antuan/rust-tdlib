use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns information about a chat folder by its identifier
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatFolder {
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

impl RObject for GetChatFolder {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetChatFolder {}

impl GetChatFolder {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetChatFolderBuilder {
        let mut inner = GetChatFolder::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getChatFolder".to_string();

        GetChatFolderBuilder { inner }
    }

    pub fn chat_folder_id(&self) -> i32 {
        self.chat_folder_id
    }
}

#[doc(hidden)]
pub struct GetChatFolderBuilder {
    inner: GetChatFolder,
}

#[deprecated]
pub type RTDGetChatFolderBuilder = GetChatFolderBuilder;

impl GetChatFolderBuilder {
    pub fn build(&self) -> GetChatFolder {
        self.inner.clone()
    }

    pub fn chat_folder_id(&mut self, chat_folder_id: i32) -> &mut Self {
        self.inner.chat_folder_id = chat_folder_id;
        self
    }
}

impl AsRef<GetChatFolder> for GetChatFolder {
    fn as_ref(&self) -> &GetChatFolder {
        self
    }
}

impl AsRef<GetChatFolder> for GetChatFolderBuilder {
    fn as_ref(&self) -> &GetChatFolder {
        &self.inner
    }
}
