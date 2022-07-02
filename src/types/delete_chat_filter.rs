use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Deletes existing chat filter
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteChatFilter {
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

impl RObject for DeleteChatFilter {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for DeleteChatFilter {}

impl DeleteChatFilter {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> DeleteChatFilterBuilder {
        let mut inner = DeleteChatFilter::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "deleteChatFilter".to_string();

        DeleteChatFilterBuilder { inner }
    }

    pub fn chat_filter_id(&self) -> i32 {
        self.chat_filter_id
    }
}

#[doc(hidden)]
pub struct DeleteChatFilterBuilder {
    inner: DeleteChatFilter,
}

#[deprecated]
pub type RTDDeleteChatFilterBuilder = DeleteChatFilterBuilder;

impl DeleteChatFilterBuilder {
    pub fn build(&self) -> DeleteChatFilter {
        self.inner.clone()
    }

    pub fn chat_filter_id(&mut self, chat_filter_id: i32) -> &mut Self {
        self.inner.chat_filter_id = chat_filter_id;
        self
    }
}

impl AsRef<DeleteChatFilter> for DeleteChatFilter {
    fn as_ref(&self) -> &DeleteChatFilter {
        self
    }
}

impl AsRef<DeleteChatFilter> for DeleteChatFilterBuilder {
    fn as_ref(&self) -> &DeleteChatFilter {
        &self.inner
    }
}
