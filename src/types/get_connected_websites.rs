use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns all website where the current user used Telegram to log in
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetConnectedWebsites {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetConnectedWebsites {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetConnectedWebsites {}

impl GetConnectedWebsites {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetConnectedWebsitesBuilder {
        let mut inner = GetConnectedWebsites::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getConnectedWebsites".to_string();

        GetConnectedWebsitesBuilder { inner }
    }
}

#[doc(hidden)]
pub struct GetConnectedWebsitesBuilder {
    inner: GetConnectedWebsites,
}

#[deprecated]
pub type RTDGetConnectedWebsitesBuilder = GetConnectedWebsitesBuilder;

impl GetConnectedWebsitesBuilder {
    pub fn build(&self) -> GetConnectedWebsites {
        self.inner.clone()
    }
}

impl AsRef<GetConnectedWebsites> for GetConnectedWebsites {
    fn as_ref(&self) -> &GetConnectedWebsites {
        self
    }
}

impl AsRef<GetConnectedWebsites> for GetConnectedWebsitesBuilder {
    fn as_ref(&self) -> &GetConnectedWebsites {
        &self.inner
    }
}
