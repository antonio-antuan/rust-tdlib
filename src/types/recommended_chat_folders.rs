use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of recommended chat folders
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecommendedChatFolders {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// List of recommended chat folders

    #[serde(default)]
    chat_folders: Vec<RecommendedChatFolder>,
}

impl RObject for RecommendedChatFolders {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RecommendedChatFolders {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RecommendedChatFoldersBuilder {
        let mut inner = RecommendedChatFolders::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RecommendedChatFoldersBuilder { inner }
    }

    pub fn chat_folders(&self) -> &Vec<RecommendedChatFolder> {
        &self.chat_folders
    }
}

#[doc(hidden)]
pub struct RecommendedChatFoldersBuilder {
    inner: RecommendedChatFolders,
}

#[deprecated]
pub type RTDRecommendedChatFoldersBuilder = RecommendedChatFoldersBuilder;

impl RecommendedChatFoldersBuilder {
    pub fn build(&self) -> RecommendedChatFolders {
        self.inner.clone()
    }

    pub fn chat_folders(&mut self, chat_folders: Vec<RecommendedChatFolder>) -> &mut Self {
        self.inner.chat_folders = chat_folders;
        self
    }
}

impl AsRef<RecommendedChatFolders> for RecommendedChatFolders {
    fn as_ref(&self) -> &RecommendedChatFolders {
        self
    }
}

impl AsRef<RecommendedChatFolders> for RecommendedChatFoldersBuilder {
    fn as_ref(&self) -> &RecommendedChatFolders {
        &self.inner
    }
}
