use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns recommended chat folders for the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetRecommendedChatFolders {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetRecommendedChatFolders {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetRecommendedChatFolders {}

impl GetRecommendedChatFolders {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetRecommendedChatFoldersBuilder {
        let mut inner = GetRecommendedChatFolders::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getRecommendedChatFolders".to_string();

        GetRecommendedChatFoldersBuilder { inner }
    }
}

#[doc(hidden)]
pub struct GetRecommendedChatFoldersBuilder {
    inner: GetRecommendedChatFolders,
}

#[deprecated]
pub type RTDGetRecommendedChatFoldersBuilder = GetRecommendedChatFoldersBuilder;

impl GetRecommendedChatFoldersBuilder {
    pub fn build(&self) -> GetRecommendedChatFolders {
        self.inner.clone()
    }
}

impl AsRef<GetRecommendedChatFolders> for GetRecommendedChatFolders {
    fn as_ref(&self) -> &GetRecommendedChatFolders {
        self
    }
}

impl AsRef<GetRecommendedChatFolders> for GetRecommendedChatFoldersBuilder {
    fn as_ref(&self) -> &GetRecommendedChatFolders {
        &self.inner
    }
}
