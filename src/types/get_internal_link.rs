use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns an HTTPS or a tg: link with the given type. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetInternalLink {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Expected type of the link

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "InternalLinkType::_is_default")]
    type_: InternalLinkType,
    /// Pass true to create an HTTPS link (only available for some link types); pass false to create a tg: link

    #[serde(default)]
    is_http: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetInternalLink {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetInternalLink {}

impl GetInternalLink {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetInternalLinkBuilder {
        let mut inner = GetInternalLink::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getInternalLink".to_string();

        GetInternalLinkBuilder { inner }
    }

    pub fn type_(&self) -> &InternalLinkType {
        &self.type_
    }

    pub fn is_http(&self) -> bool {
        self.is_http
    }
}

#[doc(hidden)]
pub struct GetInternalLinkBuilder {
    inner: GetInternalLink,
}

#[deprecated]
pub type RTDGetInternalLinkBuilder = GetInternalLinkBuilder;

impl GetInternalLinkBuilder {
    pub fn build(&self) -> GetInternalLink {
        self.inner.clone()
    }

    pub fn type_<T: AsRef<InternalLinkType>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().clone();
        self
    }

    pub fn is_http(&mut self, is_http: bool) -> &mut Self {
        self.inner.is_http = is_http;
        self
    }
}

impl AsRef<GetInternalLink> for GetInternalLink {
    fn as_ref(&self) -> &GetInternalLink {
        self
    }
}

impl AsRef<GetInternalLink> for GetInternalLinkBuilder {
    fn as_ref(&self) -> &GetInternalLink {
        &self.inner
    }
}
