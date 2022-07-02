use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns information about the type of an internal link. Returns a 404 error if the link is not internal. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetInternalLinkType {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The link

    #[serde(default)]
    link: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetInternalLinkType {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInternalLinkType for GetInternalLinkType {}

impl RFunction for GetInternalLinkType {}

impl GetInternalLinkType {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetInternalLinkTypeBuilder {
        let mut inner = GetInternalLinkType::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getInternalLinkType".to_string();

        GetInternalLinkTypeBuilder { inner }
    }

    pub fn link(&self) -> &String {
        &self.link
    }
}

#[doc(hidden)]
pub struct GetInternalLinkTypeBuilder {
    inner: GetInternalLinkType,
}

#[deprecated]
pub type RTDGetInternalLinkTypeBuilder = GetInternalLinkTypeBuilder;

impl GetInternalLinkTypeBuilder {
    pub fn build(&self) -> GetInternalLinkType {
        self.inner.clone()
    }

    pub fn link<T: AsRef<str>>(&mut self, link: T) -> &mut Self {
        self.inner.link = link.as_ref().to_string();
        self
    }
}

impl AsRef<GetInternalLinkType> for GetInternalLinkType {
    fn as_ref(&self) -> &GetInternalLinkType {
        self
    }
}

impl AsRef<GetInternalLinkType> for GetInternalLinkTypeBuilder {
    fn as_ref(&self) -> &GetInternalLinkType {
        &self.inner
    }
}
