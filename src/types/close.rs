use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Closes the TDLib instance. All databases will be flushed to disk and properly closed. After the close completes, updateAuthorizationState with authorizationStateClosed will be sent. Can be called before initialization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Close {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for Close {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for Close {}

impl Close {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CloseBuilder {
        let mut inner = Close::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "close".to_string();

        CloseBuilder { inner }
    }
}

#[doc(hidden)]
pub struct CloseBuilder {
    inner: Close,
}

#[deprecated]
pub type RTDCloseBuilder = CloseBuilder;

impl CloseBuilder {
    pub fn build(&self) -> Close {
        self.inner.clone()
    }
}

impl AsRef<Close> for Close {
    fn as_ref(&self) -> &Close {
        self
    }
}

impl AsRef<Close> for CloseBuilder {
    fn as_ref(&self) -> &Close {
        &self.inner
    }
}
