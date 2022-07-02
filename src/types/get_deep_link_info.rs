use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns information about a tg:// deep link. Use "tg://need_update_for_some_feature" or "tg:some_unsupported_feature" for testing. Returns a 404 error for unknown links. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetDeepLinkInfo {
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

impl RObject for GetDeepLinkInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetDeepLinkInfo {}

impl GetDeepLinkInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetDeepLinkInfoBuilder {
        let mut inner = GetDeepLinkInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getDeepLinkInfo".to_string();

        GetDeepLinkInfoBuilder { inner }
    }

    pub fn link(&self) -> &String {
        &self.link
    }
}

#[doc(hidden)]
pub struct GetDeepLinkInfoBuilder {
    inner: GetDeepLinkInfo,
}

#[deprecated]
pub type RTDGetDeepLinkInfoBuilder = GetDeepLinkInfoBuilder;

impl GetDeepLinkInfoBuilder {
    pub fn build(&self) -> GetDeepLinkInfo {
        self.inner.clone()
    }

    pub fn link<T: AsRef<str>>(&mut self, link: T) -> &mut Self {
        self.inner.link = link.as_ref().to_string();
        self
    }
}

impl AsRef<GetDeepLinkInfo> for GetDeepLinkInfo {
    fn as_ref(&self) -> &GetDeepLinkInfo {
        self
    }
}

impl AsRef<GetDeepLinkInfo> for GetDeepLinkInfoBuilder {
    fn as_ref(&self) -> &GetDeepLinkInfo {
        &self.inner
    }
}
