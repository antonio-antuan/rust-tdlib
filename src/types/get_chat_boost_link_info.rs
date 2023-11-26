use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns information about a link to boost a chat. Can be called for any internal link of the type internalLinkTypeChatBoost
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatBoostLinkInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The link to boost a chat

    #[serde(default)]
    url: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetChatBoostLinkInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetChatBoostLinkInfo {}

impl GetChatBoostLinkInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetChatBoostLinkInfoBuilder {
        let mut inner = GetChatBoostLinkInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getChatBoostLinkInfo".to_string();

        GetChatBoostLinkInfoBuilder { inner }
    }

    pub fn url(&self) -> &String {
        &self.url
    }
}

#[doc(hidden)]
pub struct GetChatBoostLinkInfoBuilder {
    inner: GetChatBoostLinkInfo,
}

#[deprecated]
pub type RTDGetChatBoostLinkInfoBuilder = GetChatBoostLinkInfoBuilder;

impl GetChatBoostLinkInfoBuilder {
    pub fn build(&self) -> GetChatBoostLinkInfo {
        self.inner.clone()
    }

    pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
        self.inner.url = url.as_ref().to_string();
        self
    }
}

impl AsRef<GetChatBoostLinkInfo> for GetChatBoostLinkInfo {
    fn as_ref(&self) -> &GetChatBoostLinkInfo {
        self
    }
}

impl AsRef<GetChatBoostLinkInfo> for GetChatBoostLinkInfoBuilder {
    fn as_ref(&self) -> &GetChatBoostLinkInfo {
        &self.inner
    }
}
