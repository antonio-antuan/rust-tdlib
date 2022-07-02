use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Disconnects all websites from the current user's Telegram account
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DisconnectAllWebsites {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for DisconnectAllWebsites {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for DisconnectAllWebsites {}

impl DisconnectAllWebsites {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> DisconnectAllWebsitesBuilder {
        let mut inner = DisconnectAllWebsites::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "disconnectAllWebsites".to_string();

        DisconnectAllWebsitesBuilder { inner }
    }
}

#[doc(hidden)]
pub struct DisconnectAllWebsitesBuilder {
    inner: DisconnectAllWebsites,
}

#[deprecated]
pub type RTDDisconnectAllWebsitesBuilder = DisconnectAllWebsitesBuilder;

impl DisconnectAllWebsitesBuilder {
    pub fn build(&self) -> DisconnectAllWebsites {
        self.inner.clone()
    }
}

impl AsRef<DisconnectAllWebsites> for DisconnectAllWebsites {
    fn as_ref(&self) -> &DisconnectAllWebsites {
        self
    }
}

impl AsRef<DisconnectAllWebsites> for DisconnectAllWebsitesBuilder {
    fn as_ref(&self) -> &DisconnectAllWebsites {
        &self.inner
    }
}
