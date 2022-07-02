use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes a recommended chat filter
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecommendedChatFilter {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The chat filter
    filter: ChatFilter,
    /// Describes a recommended chat filter

    #[serde(default)]
    description: String,
}

impl RObject for RecommendedChatFilter {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RecommendedChatFilter {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RecommendedChatFilterBuilder {
        let mut inner = RecommendedChatFilter::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RecommendedChatFilterBuilder { inner }
    }

    pub fn filter(&self) -> &ChatFilter {
        &self.filter
    }

    pub fn description(&self) -> &String {
        &self.description
    }
}

#[doc(hidden)]
pub struct RecommendedChatFilterBuilder {
    inner: RecommendedChatFilter,
}

#[deprecated]
pub type RTDRecommendedChatFilterBuilder = RecommendedChatFilterBuilder;

impl RecommendedChatFilterBuilder {
    pub fn build(&self) -> RecommendedChatFilter {
        self.inner.clone()
    }

    pub fn filter<T: AsRef<ChatFilter>>(&mut self, filter: T) -> &mut Self {
        self.inner.filter = filter.as_ref().clone();
        self
    }

    pub fn description<T: AsRef<str>>(&mut self, description: T) -> &mut Self {
        self.inner.description = description.as_ref().to_string();
        self
    }
}

impl AsRef<RecommendedChatFilter> for RecommendedChatFilter {
    fn as_ref(&self) -> &RecommendedChatFilter {
        self
    }
}

impl AsRef<RecommendedChatFilter> for RecommendedChatFilterBuilder {
    fn as_ref(&self) -> &RecommendedChatFilter {
        &self.inner
    }
}
