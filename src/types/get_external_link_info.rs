use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns information about an action to be done when the current user clicks an external link. Don't use this method for links from secret chats if web page preview is disabled in secret chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetExternalLinkInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The link
    link: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetExternalLinkInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDLoginUrlInfo for GetExternalLinkInfo {}

impl RFunction for GetExternalLinkInfo {}

impl GetExternalLinkInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetExternalLinkInfoBuilder {
        let mut inner = GetExternalLinkInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getExternalLinkInfo".to_string();

        RTDGetExternalLinkInfoBuilder { inner }
    }

    pub fn link(&self) -> &String {
        &self.link
    }
}

#[doc(hidden)]
pub struct RTDGetExternalLinkInfoBuilder {
    inner: GetExternalLinkInfo,
}

impl RTDGetExternalLinkInfoBuilder {
    pub fn build(&self) -> GetExternalLinkInfo {
        self.inner.clone()
    }

    pub fn link<T: AsRef<str>>(&mut self, link: T) -> &mut Self {
        self.inner.link = link.as_ref().to_string();
        self
    }
}

impl AsRef<GetExternalLinkInfo> for GetExternalLinkInfo {
    fn as_ref(&self) -> &GetExternalLinkInfo {
        self
    }
}

impl AsRef<GetExternalLinkInfo> for RTDGetExternalLinkInfoBuilder {
    fn as_ref(&self) -> &GetExternalLinkInfo {
        &self.inner
    }
}
