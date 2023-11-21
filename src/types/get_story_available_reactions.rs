use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns reactions, which can be chosen for a story
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetStoryAvailableReactions {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Number of reaction per row, 5-25

    #[serde(default)]
    row_size: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetStoryAvailableReactions {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetStoryAvailableReactions {}

impl GetStoryAvailableReactions {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetStoryAvailableReactionsBuilder {
        let mut inner = GetStoryAvailableReactions::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getStoryAvailableReactions".to_string();

        GetStoryAvailableReactionsBuilder { inner }
    }

    pub fn row_size(&self) -> i32 {
        self.row_size
    }
}

#[doc(hidden)]
pub struct GetStoryAvailableReactionsBuilder {
    inner: GetStoryAvailableReactions,
}

#[deprecated]
pub type RTDGetStoryAvailableReactionsBuilder = GetStoryAvailableReactionsBuilder;

impl GetStoryAvailableReactionsBuilder {
    pub fn build(&self) -> GetStoryAvailableReactions {
        self.inner.clone()
    }

    pub fn row_size(&mut self, row_size: i32) -> &mut Self {
        self.inner.row_size = row_size;
        self
    }
}

impl AsRef<GetStoryAvailableReactions> for GetStoryAvailableReactions {
    fn as_ref(&self) -> &GetStoryAvailableReactions {
        self
    }
}

impl AsRef<GetStoryAvailableReactions> for GetStoryAvailableReactionsBuilder {
    fn as_ref(&self) -> &GetStoryAvailableReactions {
        &self.inner
    }
}
