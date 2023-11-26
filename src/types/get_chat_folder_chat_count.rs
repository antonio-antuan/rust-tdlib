use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns approximate number of chats in a being created chat folder. Main and archive chat lists must be fully preloaded for this function to work correctly
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatFolderChatCount {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The new chat folder
    folder: ChatFolder,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetChatFolderChatCount {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetChatFolderChatCount {}

impl GetChatFolderChatCount {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetChatFolderChatCountBuilder {
        let mut inner = GetChatFolderChatCount::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getChatFolderChatCount".to_string();

        GetChatFolderChatCountBuilder { inner }
    }

    pub fn folder(&self) -> &ChatFolder {
        &self.folder
    }
}

#[doc(hidden)]
pub struct GetChatFolderChatCountBuilder {
    inner: GetChatFolderChatCount,
}

#[deprecated]
pub type RTDGetChatFolderChatCountBuilder = GetChatFolderChatCountBuilder;

impl GetChatFolderChatCountBuilder {
    pub fn build(&self) -> GetChatFolderChatCount {
        self.inner.clone()
    }

    pub fn folder<T: AsRef<ChatFolder>>(&mut self, folder: T) -> &mut Self {
        self.inner.folder = folder.as_ref().clone();
        self
    }
}

impl AsRef<GetChatFolderChatCount> for GetChatFolderChatCount {
    fn as_ref(&self) -> &GetChatFolderChatCount {
        self
    }
}

impl AsRef<GetChatFolderChatCount> for GetChatFolderChatCountBuilder {
    fn as_ref(&self) -> &GetChatFolderChatCount {
        &self.inner
    }
}
