use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns an HTTPS link, which can be used to get information about the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetUserLink {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetUserLink {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetUserLink {}

impl GetUserLink {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetUserLinkBuilder {
        let mut inner = GetUserLink::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getUserLink".to_string();

        GetUserLinkBuilder { inner }
    }
}

#[doc(hidden)]
pub struct GetUserLinkBuilder {
    inner: GetUserLink,
}

#[deprecated]
pub type RTDGetUserLinkBuilder = GetUserLinkBuilder;

impl GetUserLinkBuilder {
    pub fn build(&self) -> GetUserLink {
        self.inner.clone()
    }
}

impl AsRef<GetUserLink> for GetUserLink {
    fn as_ref(&self) -> &GetUserLink {
        self
    }
}

impl AsRef<GetUserLink> for GetUserLinkBuilder {
    fn as_ref(&self) -> &GetUserLink {
        &self.inner
    }
}
