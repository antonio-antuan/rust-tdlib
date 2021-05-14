use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns information about a public or private message link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetMessageLinkInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The message link in the format "https://t.me/c/...", or "tg://privatepost?...", or "https://t.me/username/...", or "tg://resolve?..."
    url: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetMessageLinkInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetMessageLinkInfo {}

impl GetMessageLinkInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetMessageLinkInfoBuilder {
        let mut inner = GetMessageLinkInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getMessageLinkInfo".to_string();

        RTDGetMessageLinkInfoBuilder { inner }
    }

    pub fn url(&self) -> &String {
        &self.url
    }
}

#[doc(hidden)]
pub struct RTDGetMessageLinkInfoBuilder {
    inner: GetMessageLinkInfo,
}

impl RTDGetMessageLinkInfoBuilder {
    pub fn build(&self) -> GetMessageLinkInfo {
        self.inner.clone()
    }

    pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
        self.inner.url = url.as_ref().to_string();
        self
    }
}

impl AsRef<GetMessageLinkInfo> for GetMessageLinkInfo {
    fn as_ref(&self) -> &GetMessageLinkInfo {
        self
    }
}

impl AsRef<GetMessageLinkInfo> for RTDGetMessageLinkInfoBuilder {
    fn as_ref(&self) -> &GetMessageLinkInfo {
        &self.inner
    }
}
