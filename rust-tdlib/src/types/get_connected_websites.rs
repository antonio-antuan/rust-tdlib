use crate::errors::*;
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetConnectedWebsitesBuilder {
        let mut inner = GetConnectedWebsites::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getConnectedWebsites".to_string();

        RTDGetConnectedWebsitesBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDGetConnectedWebsitesBuilder {
    inner: GetConnectedWebsites,
}

impl RTDGetConnectedWebsitesBuilder {
    pub fn build(&self) -> GetConnectedWebsites {
        self.inner.clone()
    }
}

impl AsRef<GetConnectedWebsites> for GetConnectedWebsites {
    fn as_ref(&self) -> &GetConnectedWebsites {
        self
    }
}

impl AsRef<GetConnectedWebsites> for RTDGetConnectedWebsitesBuilder {
    fn as_ref(&self) -> &GetConnectedWebsites {
        &self.inner
    }
}
