use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of websites the current user is logged in with Telegram
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConnectedWebsites {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// List of connected websites

    #[serde(default)]
    websites: Vec<ConnectedWebsite>,
}

impl RObject for ConnectedWebsites {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ConnectedWebsites {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ConnectedWebsitesBuilder {
        let mut inner = ConnectedWebsites::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ConnectedWebsitesBuilder { inner }
    }

    pub fn websites(&self) -> &Vec<ConnectedWebsite> {
        &self.websites
    }
}

#[doc(hidden)]
pub struct ConnectedWebsitesBuilder {
    inner: ConnectedWebsites,
}

#[deprecated]
pub type RTDConnectedWebsitesBuilder = ConnectedWebsitesBuilder;

impl ConnectedWebsitesBuilder {
    pub fn build(&self) -> ConnectedWebsites {
        self.inner.clone()
    }

    pub fn websites(&mut self, websites: Vec<ConnectedWebsite>) -> &mut Self {
        self.inner.websites = websites;
        self
    }
}

impl AsRef<ConnectedWebsites> for ConnectedWebsites {
    fn as_ref(&self) -> &ConnectedWebsites {
        self
    }
}

impl AsRef<ConnectedWebsites> for ConnectedWebsitesBuilder {
    fn as_ref(&self) -> &ConnectedWebsites {
        &self.inner
    }
}
