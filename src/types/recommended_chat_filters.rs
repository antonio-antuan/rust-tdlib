use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of recommended chat filters
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecommendedChatFilters {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// List of recommended chat filters
    chat_filters: Vec<RecommendedChatFilter>,
}

impl RObject for RecommendedChatFilters {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RecommendedChatFilters {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDRecommendedChatFiltersBuilder {
        let mut inner = RecommendedChatFilters::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDRecommendedChatFiltersBuilder { inner }
    }

    pub fn chat_filters(&self) -> &Vec<RecommendedChatFilter> {
        &self.chat_filters
    }
}

#[doc(hidden)]
pub struct RTDRecommendedChatFiltersBuilder {
    inner: RecommendedChatFilters,
}

impl RTDRecommendedChatFiltersBuilder {
    pub fn build(&self) -> RecommendedChatFilters {
        self.inner.clone()
    }

    pub fn chat_filters(&mut self, chat_filters: Vec<RecommendedChatFilter>) -> &mut Self {
        self.inner.chat_filters = chat_filters;
        self
    }
}

impl AsRef<RecommendedChatFilters> for RecommendedChatFilters {
    fn as_ref(&self) -> &RecommendedChatFilters {
        self
    }
}

impl AsRef<RecommendedChatFilters> for RTDRecommendedChatFiltersBuilder {
    fn as_ref(&self) -> &RecommendedChatFilters {
        &self.inner
    }
}
