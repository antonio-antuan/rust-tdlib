use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns up to 20 recently used inline bots in the order of their last usage
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetRecentInlineBots {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetRecentInlineBots {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetRecentInlineBots {}

impl GetRecentInlineBots {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetRecentInlineBotsBuilder {
        let mut inner = GetRecentInlineBots::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getRecentInlineBots".to_string();

        GetRecentInlineBotsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct GetRecentInlineBotsBuilder {
    inner: GetRecentInlineBots,
}

#[deprecated]
pub type RTDGetRecentInlineBotsBuilder = GetRecentInlineBotsBuilder;

impl GetRecentInlineBotsBuilder {
    pub fn build(&self) -> GetRecentInlineBots {
        self.inner.clone()
    }
}

impl AsRef<GetRecentInlineBots> for GetRecentInlineBots {
    fn as_ref(&self) -> &GetRecentInlineBots {
        self
    }
}

impl AsRef<GetRecentInlineBots> for GetRecentInlineBotsBuilder {
    fn as_ref(&self) -> &GetRecentInlineBots {
        &self.inner
    }
}
