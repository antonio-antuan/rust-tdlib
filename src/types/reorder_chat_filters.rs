use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes the order of chat filters
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReorderChatFilters {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifiers of chat filters in the new correct order

    #[serde(default)]
    chat_filter_ids: Vec<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ReorderChatFilters {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ReorderChatFilters {}

impl ReorderChatFilters {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ReorderChatFiltersBuilder {
        let mut inner = ReorderChatFilters::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "reorderChatFilters".to_string();

        ReorderChatFiltersBuilder { inner }
    }

    pub fn chat_filter_ids(&self) -> &Vec<i32> {
        &self.chat_filter_ids
    }
}

#[doc(hidden)]
pub struct ReorderChatFiltersBuilder {
    inner: ReorderChatFilters,
}

#[deprecated]
pub type RTDReorderChatFiltersBuilder = ReorderChatFiltersBuilder;

impl ReorderChatFiltersBuilder {
    pub fn build(&self) -> ReorderChatFilters {
        self.inner.clone()
    }

    pub fn chat_filter_ids(&mut self, chat_filter_ids: Vec<i32>) -> &mut Self {
        self.inner.chat_filter_ids = chat_filter_ids;
        self
    }
}

impl AsRef<ReorderChatFilters> for ReorderChatFilters {
    fn as_ref(&self) -> &ReorderChatFilters {
        self
    }
}

impl AsRef<ReorderChatFilters> for ReorderChatFiltersBuilder {
    fn as_ref(&self) -> &ReorderChatFilters {
        &self.inner
    }
}
