use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns the link for downloading official Telegram application to be used when the current user invites friends to Telegram
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetApplicationDownloadLink {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetApplicationDownloadLink {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetApplicationDownloadLink {}

impl GetApplicationDownloadLink {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetApplicationDownloadLinkBuilder {
        let mut inner = GetApplicationDownloadLink::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getApplicationDownloadLink".to_string();

        GetApplicationDownloadLinkBuilder { inner }
    }
}

#[doc(hidden)]
pub struct GetApplicationDownloadLinkBuilder {
    inner: GetApplicationDownloadLink,
}

#[deprecated]
pub type RTDGetApplicationDownloadLinkBuilder = GetApplicationDownloadLinkBuilder;

impl GetApplicationDownloadLinkBuilder {
    pub fn build(&self) -> GetApplicationDownloadLink {
        self.inner.clone()
    }
}

impl AsRef<GetApplicationDownloadLink> for GetApplicationDownloadLink {
    fn as_ref(&self) -> &GetApplicationDownloadLink {
        self
    }
}

impl AsRef<GetApplicationDownloadLink> for GetApplicationDownloadLinkBuilder {
    fn as_ref(&self) -> &GetApplicationDownloadLink {
        &self.inner
    }
}
