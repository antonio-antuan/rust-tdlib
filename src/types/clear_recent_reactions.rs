use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Clears the list of recently used reactions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClearRecentReactions {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ClearRecentReactions {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ClearRecentReactions {}

impl ClearRecentReactions {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ClearRecentReactionsBuilder {
        let mut inner = ClearRecentReactions::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "clearRecentReactions".to_string();

        ClearRecentReactionsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ClearRecentReactionsBuilder {
    inner: ClearRecentReactions,
}

#[deprecated]
pub type RTDClearRecentReactionsBuilder = ClearRecentReactionsBuilder;

impl ClearRecentReactionsBuilder {
    pub fn build(&self) -> ClearRecentReactions {
        self.inner.clone()
    }
}

impl AsRef<ClearRecentReactions> for ClearRecentReactions {
    fn as_ref(&self) -> &ClearRecentReactions {
        self
    }
}

impl AsRef<ClearRecentReactions> for ClearRecentReactionsBuilder {
    fn as_ref(&self) -> &ClearRecentReactions {
        &self.inner
    }
}
