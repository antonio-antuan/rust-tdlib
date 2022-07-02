use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns information about a chat filter by its identifier
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatFilter {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat filter identifier

    #[serde(default)]
    chat_filter_id: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetChatFilter {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetChatFilter {}

impl GetChatFilter {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetChatFilterBuilder {
        let mut inner = GetChatFilter::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getChatFilter".to_string();

        GetChatFilterBuilder { inner }
    }

    pub fn chat_filter_id(&self) -> i32 {
        self.chat_filter_id
    }
}

#[doc(hidden)]
pub struct GetChatFilterBuilder {
    inner: GetChatFilter,
}

#[deprecated]
pub type RTDGetChatFilterBuilder = GetChatFilterBuilder;

impl GetChatFilterBuilder {
    pub fn build(&self) -> GetChatFilter {
        self.inner.clone()
    }

    pub fn chat_filter_id(&mut self, chat_filter_id: i32) -> &mut Self {
        self.inner.chat_filter_id = chat_filter_id;
        self
    }
}

impl AsRef<GetChatFilter> for GetChatFilter {
    fn as_ref(&self) -> &GetChatFilter {
        self
    }
}

impl AsRef<GetChatFilter> for GetChatFilterBuilder {
    fn as_ref(&self) -> &GetChatFilter {
        &self.inner
    }
}
