use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes a recommended chat folder
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecommendedChatFolder {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The chat folder
    folder: ChatFolder,
    /// Describes a recommended chat folder

    #[serde(default)]
    description: String,
}

impl RObject for RecommendedChatFolder {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RecommendedChatFolder {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RecommendedChatFolderBuilder {
        let mut inner = RecommendedChatFolder::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RecommendedChatFolderBuilder { inner }
    }

    pub fn folder(&self) -> &ChatFolder {
        &self.folder
    }

    pub fn description(&self) -> &String {
        &self.description
    }
}

#[doc(hidden)]
pub struct RecommendedChatFolderBuilder {
    inner: RecommendedChatFolder,
}

#[deprecated]
pub type RTDRecommendedChatFolderBuilder = RecommendedChatFolderBuilder;

impl RecommendedChatFolderBuilder {
    pub fn build(&self) -> RecommendedChatFolder {
        self.inner.clone()
    }

    pub fn folder<T: AsRef<ChatFolder>>(&mut self, folder: T) -> &mut Self {
        self.inner.folder = folder.as_ref().clone();
        self
    }

    pub fn description<T: AsRef<str>>(&mut self, description: T) -> &mut Self {
        self.inner.description = description.as_ref().to_string();
        self
    }
}

impl AsRef<RecommendedChatFolder> for RecommendedChatFolder {
    fn as_ref(&self) -> &RecommendedChatFolder {
        self
    }
}

impl AsRef<RecommendedChatFolder> for RecommendedChatFolderBuilder {
    fn as_ref(&self) -> &RecommendedChatFolder {
        &self.inner
    }
}
