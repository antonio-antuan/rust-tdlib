use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns an HTTP URL which can be used to automatically authorize the current user on a website after clicking an HTTP link. Use the method getExternalLinkInfo to find whether a prior user confirmation is needed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetExternalLink {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The HTTP link

    #[serde(default)]
    link: String,
    /// True, if the current user allowed the bot, returned in getExternalLinkInfo, to send them messages

    #[serde(default)]
    allow_write_access: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetExternalLink {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetExternalLink {}

impl GetExternalLink {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetExternalLinkBuilder {
        let mut inner = GetExternalLink::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getExternalLink".to_string();

        GetExternalLinkBuilder { inner }
    }

    pub fn link(&self) -> &String {
        &self.link
    }

    pub fn allow_write_access(&self) -> bool {
        self.allow_write_access
    }
}

#[doc(hidden)]
pub struct GetExternalLinkBuilder {
    inner: GetExternalLink,
}

#[deprecated]
pub type RTDGetExternalLinkBuilder = GetExternalLinkBuilder;

impl GetExternalLinkBuilder {
    pub fn build(&self) -> GetExternalLink {
        self.inner.clone()
    }

    pub fn link<T: AsRef<str>>(&mut self, link: T) -> &mut Self {
        self.inner.link = link.as_ref().to_string();
        self
    }

    pub fn allow_write_access(&mut self, allow_write_access: bool) -> &mut Self {
        self.inner.allow_write_access = allow_write_access;
        self
    }
}

impl AsRef<GetExternalLink> for GetExternalLink {
    fn as_ref(&self) -> &GetExternalLink {
        self
    }
}

impl AsRef<GetExternalLink> for GetExternalLinkBuilder {
    fn as_ref(&self) -> &GetExternalLink {
        &self.inner
    }
}
